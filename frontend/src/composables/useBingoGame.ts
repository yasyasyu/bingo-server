import { ref } from 'vue'
import { useAudio } from './useAudio'
import { bingoApi } from '../services/bingoApi'

// Global State
const currentNumber = ref<number | null>(null)
const displayText = ref<string | number>('Merry Christmas!')
const history = ref<number[]>([])
const isSpinning = ref(false)
const seed = ref<number | null>(null)

export function useBingoGame() {
    const { playBeep, playWin, resumeAudioContext } = useAudio()

    const resetGame = async () => {
        if (!confirm('本当にリセットしますか？')) return
        try {
            const data = await bingoApi.resetGame()
            if (data) {
                seed.value = data.seed
            }
            currentNumber.value = null
            displayText.value = 'Merry Christmas!'
            history.value = []
        } catch (e) {
            console.error(e)
        }
    }

    const spin = async () => {
        if (isSpinning.value) return

        await resumeAudioContext()

        isSpinning.value = true

        // バックエンドから数字取得
        const data = await bingoApi.fetchNextNumber()

        if (!data) {
            isSpinning.value = false
            alert('サーバーエラーです')
            return
        }

        seed.value = data.seed
        const targetNumber = data.number

        if (targetNumber === null) {
            displayText.value = "Finished!"
            isSpinning.value = false
            return
        }

        // ルーレット演出
        const duration = 5000 // 5秒
        const startTime = Date.now()

        const animate = () => {
            const elapsed = Date.now() - startTime
            const progress = Math.max(0.1, elapsed) / duration

            if (elapsed < duration) {
                // ランダムな数字を表示（履歴にない数字のみ）
                let randomNum
                do {
                    randomNum = Math.floor(Math.random() * 75) + 1
                } while (history.value.includes(randomNum))
                displayText.value = randomNum
                playBeep()

                // イージング関数で徐々に減速（二次関数的な減速）
                const easeOut = 1 - Math.pow(1 - progress, 2)
                const speed = 10 + easeOut * 190 // 10ms → 200ms

                setTimeout(() => requestAnimationFrame(animate), speed)
            } else {
                // 停止前に少し待つ
                setTimeout(() => {
                    displayText.value = targetNumber
                    currentNumber.value = targetNumber
                    history.value = data.history
                    isSpinning.value = false
                    playWin()
                }, 150)
            }
        }
        animate()
    }

    return {
        currentNumber,
        displayText,
        history,
        isSpinning,
        seed,
        spin,
        resetGame,
    }
}
