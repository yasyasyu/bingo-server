import { ref } from 'vue'

export interface HorizontalLine {
    level: number
    leftIndex: number
}

export function useAmidaGame() {
    const HORIZONTAL_LINES_COUNT = 15
    const horizontalLines = ref<HorizontalLine[]>([])
    const bottomPrizes = ref<string[]>(new Array(10).fill('???'))

    /**
     * あみだくじの線をランダムに生成する
     * 
     * 各レベル（高さ）に対して、ランダムな位置に横線を配置します。
     * 隣接する線が重ならないように制御されます。
     */
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

    /**
     * あみだくじの結果（ゴール地点のゲスト）を計算する
     * 
     * サーバーから取得した結果に基づいて、あみだくじのゴール地点（下部）に表示するゲスト名を決定します。
     * あみだくじのパスをシミュレーションし、上部の番号（景品）に対応するゴール地点に
     * 正しいゲスト名が配置されるようにします。
     * 
     * @param results - [GuestName, PrizeNumber] のペアの配列
     */
    const calculatePrizes = (
        results: [string, string][]
    ) => {
        // Prize -> Guest のマップを作成
        // 例: "1" -> "Guest A"
        const prizeMap = new Map<string, string>()
        results.forEach(([guest, prize]) => {
            prizeMap.set(prize, guest)
        })

        const newBottomPrizes = new Array(10).fill('???')

        // 上部（1〜10の番号）からスタートして、あみだくじを辿ります
        for (let i = 0; i < 10; i++) {
            const prizeName = (i + 1).toString()

            // パスをシミュレーション
            let currentX = i
            for (let level = 0; level < HORIZONTAL_LINES_COUNT; level++) {
                // 現在のレベル・位置に横線があるかチェック
                const hLine = horizontalLines.value.find(l => l.level === level && (l.leftIndex === currentX || l.leftIndex === currentX - 1))
                if (hLine) {
                    // 横線があれば移動
                    if (hLine.leftIndex === currentX) currentX++
                    else currentX--
                }
            }

            // ゴール地点 (currentX) に、その番号に対応するゲストを配置
            const guest = prizeMap.get(prizeName)
            if (guest) {
                newBottomPrizes[currentX] = guest
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
