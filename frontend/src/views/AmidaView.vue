<script setup lang="ts">
import { onMounted, ref, nextTick, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAmida } from '../composables/useAmida'
import { useAmidaGame } from '../composables/useAmidaGame'

const router = useRouter()
const route = useRoute()
const { items, isConfigured, isLoading, fetchAmida, setupAmida, fetchResults } = useAmida()
const { 
  HORIZONTAL_LINES_COUNT, 
  horizontalLines, 
  bottomPrizes, 
  generateAmida, 
  calculatePrizes 
} = useAmidaGame()

// Setup Mode State
const inputItems = ref<string[]>(new Array(10).fill(''))
const revealedIndices = ref<Set<number>>(new Set())

// Game Mode State
const canvasRef = ref<HTMLCanvasElement | null>(null)
const selectedStart = ref<number | null>(null)
const isAnimating = ref(false)
const resultIndex = ref<number | null>(null)

onMounted(async () => {
  await fetchAmida()
  if (isConfigured.value) {
    inputItems.value = [...items.value]
    if (route.path === '/amida/result') {
      generateAmida()
      await updatePrizes()
      await nextTick()
      drawAmida()
    }
  } else if (route.path === '/amida/result') {
    router.replace('/amida')
  }
})

watch(() => route.path, async (newPath) => {
  if (newPath === '/amida/result') {
    if (!isConfigured.value) {
      router.replace('/amida')
      return
    }
    if (horizontalLines.value.length === 0) {
      generateAmida()
    }
    await updatePrizes()
    await nextTick()
    drawAmida()
  }
})

const saveInput = async () => {
  await setupAmida(inputItems.value)
}

const handleSubmit = async () => {
  await saveInput()
  const results = await fetchResults()
  if (results && results.length > 0) {
    router.push('/amida/result')
  } else {
    alert("Please enter all items before starting.")
  }
}

const updatePrizes = async () => {
  const results = await fetchResults()
  if (results) {
    calculatePrizes(results)
  }
}

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
  for (const line of horizontalLines.value) {
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
  // We will trace the path step by step
  // Path logic: Go down to next level center. Check if there is a horizontal line.
  // If yes, cross. If no, continue down.
  
  // To animate smoothly, we need to interpolate.
  // Let's simplify: We animate segment by segment.
  
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
    const hLine = horizontalLines.value.find(l => l.level === level && (l.leftIndex === currentXIndex || l.leftIndex === currentXIndex - 1))
    
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

const resetView = () => {
  isConfigured.value = false
  resultIndex.value = null
  selectedStart.value = null
  revealedIndices.value.clear()
  router.push('/amida')
}

const clearResult = () => {
    resultIndex.value = null
    selectedStart.value = null
    drawAmida()
}

</script>

<template>
  <div class="amida-container">
    <h1 class="title">🎅 Amidakuji 🎅</h1>

    <!-- Setup Mode -->
    <div v-if="route.path === '/amida'" class="setup-panel">
      <h2>Enter 10 Items</h2>
      <div class="inputs-grid">
        <div v-for="(_item, index) in inputItems" :key="index" class="input-group">
          <label>{{ index + 1 }}</label>
          <input v-model="inputItems[index]" placeholder="Item name..." @blur="saveInput" />
        </div>
      </div>
      <button @click="handleSubmit" :disabled="isLoading" class="start-btn">
        Start Game
      </button>
    </div>

    <!-- Game Mode -->
    <div v-else class="game-panel">
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
          <button @click="resetView" :disabled="isAnimating" class="control-btn danger">Edit Items</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.amida-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100%;
  width: 100%;
  position: relative;
  z-index: 1;
  padding: 20px;
  box-sizing: border-box;
}

.title {
  font-size: 3rem;
  color: #d4af37;
  text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
  margin-bottom: 20px;
  font-family: 'Brush Script MT', cursive;
}

.setup-panel {
  background: rgba(0, 0, 0, 0.6);
  padding: 30px;
  border-radius: 15px;
  text-align: center;
}

.inputs-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  margin-bottom: 20px;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.input-group label {
  width: 20px;
  font-weight: bold;
}

.input-group input {
  padding: 8px;
  border-radius: 5px;
  border: none;
  width: 200px;
}

.start-btn {
  padding: 10px 30px;
  font-size: 1.2rem;
  background: #d4af37;
  border: none;
  border-radius: 25px;
  cursor: pointer;
  font-weight: bold;
  color: #1a472a;
  transition: transform 0.2s;
}

.start-btn:hover {
  transform: scale(1.05);
}

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
