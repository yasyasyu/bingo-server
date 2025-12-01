<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue'
import type { HorizontalLine } from '../composables/useAmidaGame'

const props = defineProps<{
  horizontalLines: HorizontalLine[]
  bottomPrizes: string[]
}>()

const emit = defineEmits<{
  (e: 'edit'): void
}>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const selectedStart = ref<number | null>(null)
const isAnimating = ref(false)
const resultIndex = ref<number | null>(null)
const revealedIndices = ref<Set<number>>(new Set())

const HORIZONTAL_LINES_COUNT = 15

const drawAmida = () => {
  const canvas = canvasRef.value
  if (!canvas) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const width = canvas.width
  const height = canvas.height
  const lineSpacing = width / 11
  const startY = 50
  const endY = height - 50
  const levelHeight = (endY - startY) / HORIZONTAL_LINES_COUNT

  ctx.clearRect(0, 0, width, height)
  
  // Draw Vertical Lines
  ctx.strokeStyle = 'white'
  ctx.lineWidth = 3
  ctx.lineCap = 'round'

  for (let i = 0; i < 10; i++) {
    const x = lineSpacing * (i + 1)
    ctx.beginPath()
    ctx.moveTo(x, startY)
    ctx.lineTo(x, endY)
    ctx.stroke()
  }

  // Draw Horizontal Lines
  for (const line of props.horizontalLines) {
    const x1 = lineSpacing * (line.leftIndex + 1)
    const x2 = lineSpacing * (line.leftIndex + 2)
    const y = startY + (line.level + 0.5) * levelHeight
    
    ctx.beginPath()
    ctx.moveTo(x1, y)
    ctx.lineTo(x2, y)
    ctx.stroke()
  }
}

const startAnimation = async (startIndex: number) => {
  if (isAnimating.value) return
  selectedStart.value = startIndex
  isAnimating.value = true
  resultIndex.value = null

  const canvas = canvasRef.value
  if (!canvas) return
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const width = canvas.width
  const height = canvas.height
  const lineSpacing = width / 11
  const startY = 50
  const endY = height - 50
  const levelHeight = (endY - startY) / HORIZONTAL_LINES_COUNT

  let currentXIndex = startIndex
  let currentY = startY

  // Animation Loop
  ctx.strokeStyle = '#ff0000' // Red for active path
  ctx.lineWidth = 5

  for (let level = 0; level < HORIZONTAL_LINES_COUNT; level++) {
    const nextY = startY + (level + 1) * levelHeight
    const midY = startY + (level + 0.5) * levelHeight
    
    // 1. Go down to mid of level
    await animateLine(ctx, 
      lineSpacing * (currentXIndex + 1), currentY,
      lineSpacing * (currentXIndex + 1), midY
    )
    currentY = midY

    // 2. Check for horizontal line
    const hLine = props.horizontalLines.find(l => l.level === level && (l.leftIndex === currentXIndex || l.leftIndex === currentXIndex - 1))
    
    if (hLine) {
      // Cross over
      const targetXIndex = hLine.leftIndex === currentXIndex ? currentXIndex + 1 : currentXIndex - 1
      await animateLine(ctx,
        lineSpacing * (currentXIndex + 1), currentY,
        lineSpacing * (targetXIndex + 1), currentY
      )
      currentXIndex = targetXIndex
    }

    // 3. Go down to bottom of level
    await animateLine(ctx,
      lineSpacing * (currentXIndex + 1), currentY,
      lineSpacing * (currentXIndex + 1), nextY
    )
    currentY = nextY
  }

  // Final segment to bottom
  await animateLine(ctx,
    lineSpacing * (currentXIndex + 1), currentY,
    lineSpacing * (currentXIndex + 1), endY
  )

  resultIndex.value = currentXIndex
  if (currentXIndex !== null) {
    revealedIndices.value.add(currentXIndex)
  }
  isAnimating.value = false
}

const animateLine = (ctx: CanvasRenderingContext2D, x1: number, y1: number, x2: number, y2: number): Promise<void> => {
  return new Promise(resolve => {
    const duration = 100 // ms per segment
    const startTime = performance.now()

    const animate = (time: number) => {
      const elapsed = time - startTime
      const progress = Math.min(elapsed / duration, 1)

      const currentX = x1 + (x2 - x1) * progress
      const currentY = y1 + (y2 - y1) * progress

      ctx.beginPath()
      ctx.moveTo(x1, y1)
      ctx.lineTo(currentX, currentY)
      ctx.stroke()

      if (progress < 1) {
        requestAnimationFrame(animate)
      } else {
        resolve()
      }
    }
    requestAnimationFrame(animate)
  })
}

const clearResult = () => {
    resultIndex.value = null
    selectedStart.value = null
    drawAmida()
}

watch(() => props.horizontalLines, () => {
    nextTick(() => drawAmida())
}, { deep: true })

onMounted(() => {
    drawAmida()
})
</script>

<template>
  <div class="game-panel">
    <div class="start-buttons">
      <button 
        v-for="i in 10" 
        :key="i" 
        @click="startAnimation(i-1)"
        :disabled="isAnimating"
        class="choice-btn"
        :class="{ active: selectedStart === i-1 }"
      >
        {{ i }}
      </button>
    </div>

    <canvas ref="canvasRef" width="800" height="500" class="amida-canvas"></canvas>

    <div class="results-row">
      <div v-for="i in 10" :key="i" class="result-item" :class="{ highlight: resultIndex === i-1 }">
        {{ revealedIndices.has(i-1) ? bottomPrizes[i-1] : '???' }}
      </div>
    </div>

    <div class="controls">
        <button @click="clearResult" :disabled="isAnimating" class="control-btn">Clear Path</button>
        <button @click="$emit('edit')" :disabled="isAnimating" class="control-btn danger">Edit Items</button>
    </div>
  </div>
</template>

<style scoped>
.game-panel {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  max-width: 900px;
}

.start-buttons {
  display: flex;
  justify-content: space-between;
  width: 800px; /* Match canvas width */
  margin-bottom: 10px;
  padding: 0 20px; /* Adjust for line spacing roughly */
}

.choice-btn {
  min-width: 40px;
  width: 60px; /* Fixed width to align with grid */
  height: auto;
  min-height: 40px;
  padding: 5px;
  border-radius: 5px;
  border: 2px solid white;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s;
  font-size: 0.8rem;
  word-break: break-word;
  display: flex;
  align-items: center;
  justify-content: center;
}

.choice-btn:hover {
  background: rgba(255, 255, 255, 0.5);
}

.choice-btn.active {
  background: #ff0000;
  border-color: #ff0000;
}

.amida-canvas {
  background: rgba(0, 0, 0, 0.3);
  border-radius: 10px;
  margin-bottom: 10px;
}

.results-row {
  display: flex;
  justify-content: space-between;
  width: 800px;
  padding: 0 10px; /* Adjust alignment */
}

.result-item {
  width: 60px; /* Approx width */
  text-align: center;
  font-size: 0.9rem;
  word-break: break-word;
  padding: 5px;
  border-radius: 5px;
  transition: background 0.3s;
}

.result-item.highlight {
  background: #d4af37;
  color: #1a472a;
  font-weight: bold;
  transform: scale(1.1);
}

.controls {
    margin-top: 20px;
    display: flex;
    gap: 10px;
}

.control-btn {
    padding: 8px 16px;
    border-radius: 5px;
    border: none;
    cursor: pointer;
    background: rgba(255, 255, 255, 0.2);
    color: white;
}

.control-btn:hover {
    background: rgba(255, 255, 255, 0.4);
}

.control-btn.danger {
    background: rgba(255, 0, 0, 0.3);
}
.control-btn.danger:hover {
    background: rgba(255, 0, 0, 0.5);
}
</style>
