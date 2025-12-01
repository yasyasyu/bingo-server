import { ref } from 'vue'

export interface HorizontalLine {
    level: number
    leftIndex: number
}

export function useAmidaGame() {
    const HORIZONTAL_LINES_COUNT = 15
    const horizontalLines = ref<HorizontalLine[]>([])
    const bottomPrizes = ref<string[]>(new Array(10).fill('???'))

    const generateAmida = () => {
        const lines: HorizontalLine[] = []
        // Generate random horizontal lines
        // Ensure no overlapping lines at same level
        for (let level = 0; level < HORIZONTAL_LINES_COUNT; level++) {
            // Try to place a few lines at each level
            const usedIndices = new Set<number>()
            // Attempt to place 1-3 lines per level
            const numLines = Math.floor(Math.random() * 3) + 1

            for (let k = 0; k < numLines; k++) {
                // Random position 0 to 8 (since there are 9 gaps between 10 lines)
                const leftIndex = Math.floor(Math.random() * 9)

                // Check if this index or adjacent ones are already used in this level
                if (!usedIndices.has(leftIndex) && !usedIndices.has(leftIndex - 1) && !usedIndices.has(leftIndex + 1)) {
                    lines.push({ level, leftIndex })
                    usedIndices.add(leftIndex)
                }
            }
        }
        horizontalLines.value = lines
    }

    const calculatePrizes = (
        guestNames: string[],
        results: [string, string][]
    ) => {
        // Create a map of Guest -> Prize
        const prizeMap = new Map<string, string>()
        results.forEach(([guest, prize]) => {
            prizeMap.set(guest, prize)
        })

        const newBottomPrizes = new Array(10).fill('???')

        for (let i = 0; i < 10; i++) {
            const guestName = guestNames[i]
            if (!guestName) continue

            // Simulate path
            let currentX = i
            for (let level = 0; level < HORIZONTAL_LINES_COUNT; level++) {
                const hLine = horizontalLines.value.find(l => l.level === level && (l.leftIndex === currentX || l.leftIndex === currentX - 1))
                if (hLine) {
                    if (hLine.leftIndex === currentX) currentX++
                    else currentX--
                }
            }

            const prize = prizeMap.get(guestName)
            if (prize) {
                newBottomPrizes[currentX] = prize
            }
        }
        bottomPrizes.value = newBottomPrizes
    }

    return {
        HORIZONTAL_LINES_COUNT,
        horizontalLines,
        bottomPrizes,
        generateAmida,
        calculatePrizes
    }
}
