<script setup lang="ts">
import { ref, onMounted, watch, nextTick, computed } from 'vue'
import type { HorizontalLine } from '../composables/useAmidaGame'
import { useDrumRoll } from '../composables/useDrumRoll'

const props = defineProps<{
    horizontalLines: HorizontalLine[]
    bottomPrizes: string[]
}>()

const count = computed(() => props.bottomPrizes.length)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const activeStartIndices = ref<Set<number>>(new Set())
const isAnimating = ref(false)
const lastResultIndices = ref<Set<number>>(new Set())
const revealedIndices = ref<Set<number>>(new Set())
const resultMap = ref<Map<number, number>>(new Map())
const usedStartIndices = ref<Set<number>>(new Set())
const isSoundEnabled = ref(true)

const { play: playDrum, stop: stopDrum, playCymbal } = useDrumRoll()

watch(isSoundEnabled, (enabled) => {
    if (!enabled) {
        stopDrum()
    }
})

const HORIZONTAL_LINES_COUNT = 16

const drawAmida = () => {
    const canvas = canvasRef.value
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const width = canvas.width
    const height = canvas.height
    const lineSpacing = width / (count.value + 1)
    const startY = 50
    const endY = height - 50
    const levelHeight = (endY - startY) / HORIZONTAL_LINES_COUNT

    ctx.clearRect(0, 0, width, height)

    // Draw Vertical Lines
    ctx.strokeStyle = 'white'
    ctx.lineWidth = 14
    ctx.lineCap = 'round'

    for (let i = 0; i < count.value; i++) {
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

const animateSinglePath = async (ctx: CanvasRenderingContext2D, startIndex: number, color: string, offset: number, lineWidth: number): Promise<number | null> => {
    const canvas = canvasRef.value
    if (!canvas) return null
    const width = canvas.width
    const height = canvas.height
    const lineSpacing = width / (count.value + 1)
    const startY = 50
    const endY = height - 50
    const levelHeight = (endY - startY) / HORIZONTAL_LINES_COUNT

    let currentXIndex = startIndex
    let currentY = startY

    for (let level = 0; level < HORIZONTAL_LINES_COUNT; level++) {
        const nextY = startY + (level + 1) * levelHeight
        const midY = startY + (level + 0.5) * levelHeight

        // 1. Go down to mid of level
        await animateLine(ctx,
            lineSpacing * (currentXIndex + 1) + offset, currentY + offset,
            lineSpacing * (currentXIndex + 1) + offset, midY + offset,
            color, lineWidth
        )
        currentY = midY

        // 2. Check for horizontal line
        const hLine = props.horizontalLines.find(l => l.level === level && (l.leftIndex === currentXIndex || l.leftIndex === currentXIndex - 1))

        if (hLine) {
            // Cross over
            const targetXIndex = hLine.leftIndex === currentXIndex ? currentXIndex + 1 : currentXIndex - 1
            await animateLine(ctx,
                lineSpacing * (currentXIndex + 1) + offset, currentY + offset,
                lineSpacing * (targetXIndex + 1) + offset, currentY + offset,
                color, lineWidth
            )
            currentXIndex = targetXIndex
        }

        // 3. Go down to bottom of level
        await animateLine(ctx,
            lineSpacing * (currentXIndex + 1) + offset, currentY + offset,
            lineSpacing * (currentXIndex + 1) + offset, nextY + offset,
            color, lineWidth
        )
        currentY = nextY
    }

    // Final segment to bottom
    await animateLine(ctx,
        lineSpacing * (currentXIndex + 1) + offset, currentY + offset,
        lineSpacing * (currentXIndex + 1) + offset, endY + offset,
        color, lineWidth
    )

    return currentXIndex
}

const startAnimation = async (startIndex: number) => {
    if (isAnimating.value || usedStartIndices.value.has(startIndex)) return

    const remaining = []
    for (let i = 0; i < 8; i++) {
        if (!usedStartIndices.value.has(i)) remaining.push(i)
    }

    const targets: number[] = []
    if (remaining.length === 2 && remaining.includes(startIndex)) {
        targets.push(...remaining)
    } else {
        targets.push(startIndex)
    }

    drawAmida()
    activeStartIndices.value.clear()
    targets.forEach(t => activeStartIndices.value.add(t))
    isAnimating.value = true
    lastResultIndices.value.clear()

    const canvas = canvasRef.value
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Animation Loop
    // Colors: If 2 targets (last 2), use Red and Blue. Otherwise Red.
    const isDual = targets.length === 2
    const colors = isDual ? ['#ff0000', '#0088ff'] : ['#ff0000']
    const offsets = isDual ? [-3, 3] : [0]
    const lineWidths = isDual ? [6, 6] : [8]

    if (isSoundEnabled.value) {
        playDrum(isDual)
    }

    const promises = targets.map((idx, i) => animateSinglePath(ctx, idx, colors[i] || '#ff0000', offsets[i] || 0, lineWidths[i] || 5))
    const results = await Promise.all(promises)

    stopDrum()
    if (isSoundEnabled.value) {
        await new Promise(resolve => setTimeout(resolve, 500))
        playCymbal()
    }

    results.forEach((resIdx, i) => {
        const startIdx = targets[i]
        if (resIdx !== null && startIdx !== undefined) {
            revealedIndices.value.add(resIdx)
            resultMap.value.set(resIdx, startIdx)
            lastResultIndices.value.add(resIdx)
            usedStartIndices.value.add(startIdx)
        }
    })

    isAnimating.value = false
}

const animateLine = (ctx: CanvasRenderingContext2D, x1: number, y1: number, x2: number, y2: number, color: string, lineWidth: number): Promise<void> => {
    return new Promise(resolve => {
        const duration = 100 // ms per segment
        const startTime = performance.now()

        const animate = (time: number) => {
            const elapsed = time - startTime
            const progress = Math.min(elapsed / duration, 1)

            const currentX = x1 + (x2 - x1) * progress
            const currentY = y1 + (y2 - y1) * progress

            ctx.strokeStyle = color
            ctx.lineWidth = lineWidth
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
    lastResultIndices.value.clear()
    activeStartIndices.value.clear()
    revealedIndices.value.clear()
    resultMap.value.clear()
    usedStartIndices.value.clear()
    drawAmida()
}

watch(() => props.horizontalLines, () => {
    nextTick(() => drawAmida())
}, { deep: true })

onMounted(() => {
    drawAmida()
})
const getLabel = (index: number) => `No${index + 1}`
</script>

<template>
    <div class="game-panel">
        <div class="start-buttons">
            <button v-for="i in count" :key="i" @click="startAnimation(i - 1)"
                :disabled="isAnimating || usedStartIndices.has(i - 1)" class="choice-btn"
                :class="{ active: activeStartIndices.has(i - 1), used: usedStartIndices.has(i - 1) }"
                :style="{ left: `${(i) * (100 / (count + 1))}%` }">
                {{ getLabel(i - 1) }}
            </button>
        </div>

        <canvas ref="canvasRef" width="1400" height="800" class="amida-canvas"></canvas>

        <div class="results-row">
            <div v-for="i in count" :key="i" class="result-item" :class="{ highlight: lastResultIndices.has(i - 1) }"
                :style="{ left: `${(i) * (100 / (count + 1))}%` }">
                <div v-if="resultMap.has(i - 1)" class="prize-number">
                    {{ getLabel(resultMap.get(i - 1)!) }}
                </div>
                <div class="guest-name">{{ revealedIndices.has(i - 1) ? bottomPrizes[i - 1] : '???' }}</div>
            </div>
        </div>

        <div class="controls">
            <button @click="isSoundEnabled = !isSoundEnabled" class="control-btn"
                :style="{ opacity: isSoundEnabled ? 1 : 0.6 }">
                Sound: {{ isSoundEnabled ? 'ON' : 'OFF' }}
            </button>
            <button @click="clearResult" :disabled="isAnimating" class="control-btn">
                RESET
            </button>
        </div>
    </div>
</template>

<style scoped>
.game-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    max-width: 1500px;
}

.start-buttons {
    position: relative;
    width: 1400px;
    height: 80px;
    margin-top: 60px;
}

.choice-btn {
    position: absolute;
    transform: translateX(-50%);
    bottom: 0;
    min-width: 50px;
    width: 80px;
    height: auto;
    min-height: 50px;
    padding: 5px;
    border-radius: 5px;
    border: none;
    background: transparent;
    color: #d4af37;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.3s;
    font-size: 1.5rem;
    word-break: break-word;
    display: flex;
    align-items: center;
    justify-content: center;
}

.choice-btn:hover {
    transform: translateX(-50%) scale(1.2);
    background: transparent;
    text-shadow: 0 0 10px rgba(212, 175, 55, 0.8);
}

.choice-btn.active {
    color: #ff0000;
    background: transparent;
    border-color: transparent;
    transform: translateX(-50%) scale(1.3);
}

.choice-btn.used {
    color: white;
    opacity: 0.8;
    transform: translateX(-50%) scale(1);
    text-shadow: none;
}

.amida-canvas {
    background: rgba(0, 0, 0, 0.6);
    border-radius: 10px;
    margin-bottom: 10px;
}

.results-row {
    position: relative;
    width: 1400px;
    height: 200px;
    margin-top: 10px;
}

.result-item {
    position: absolute;
    transform: translateX(-50%);
    top: 0;
    width: 150px;
    text-align: center;
    font-size: 2.5rem;
    word-break: break-word;
    padding: 5px;
    border-radius: 5px;
    transition: background 0.3s;
    line-height: 1.0;
    color: white;
    z-index: 5;
}

.result-item.highlight {
    background: #d4af37;
    color: #1a472a;
    font-weight: bold;
    transform: translateX(-50%) scale(1.1);
}

.controls {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 30px;
    bottom: 10px;
    right: 5px;
    position: absolute;

}

.control-btn {
    padding: 5px 15px;
    font-size: 0.8 rem;
    border: none;
    border-radius: 50px;
    cursor: pointer;
    transition: transform 0.1s, box-shadow 0.1s;
    font-weight: bold;
    text-transform: uppercase;

    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 2px solid white;
}

.control-btn:active {
    transform: scale(0.95);
}

.control-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.prize-number {
    font-size: 1.5rem;
    color: #d4af37;
    /* margin-bottom: 5px; */
    font-weight: bold;
}

.result-item.highlight .prize-number {
    color: #1a472a;
}
</style>
