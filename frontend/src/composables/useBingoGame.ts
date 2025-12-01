import { ref } from 'vue'
import { useAudio } from './useAudio'
import { bingoApi } from '../services/bingoApi'

export function useBingoGame() {
    const { playBeep, playWin, resumeAudioContext } = useAudio()

    const currentNumber = ref<number | null>(null)
    const displayText = ref<string | number>('Merry Christmas!')
    const history = ref<number[]>([])
    const isSpinning = ref(false)

    const resetGame = async () => {
        if (!confirm('本当にリセットしますか？')) return
        try {
            await bingoApi.resetGame()
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

        const targetNumber = data.number

        if (targetNumber === null) {
            displayText.value = "Finished!"
            isSpinning.value = false
            return
        }

        // ルーレット演出
        let speed = 50
        const duration = 3000 // 3秒
        const startTime = Date.now()

        const animate = () => {
            const elapsed = Date.now() - startTime

            if (elapsed < duration) {
                // ランダムな数字を表示
                displayText.value = Math.floor(Math.random() * 75) + 1
                playBeep()

                // 徐々に遅くする
                if (elapsed > duration * 0.7) {
                    speed += 10
                }

                setTimeout(() => requestAnimationFrame(animate), speed)
            } else {
                // 停止
                displayText.value = targetNumber
                currentNumber.value = targetNumber
                history.value = data.history
                isSpinning.value = false
                playWin()
            }
        }
        animate()
    }

    return {
        currentNumber,
        displayText,
        history,
        isSpinning,
        spin,
        resetGame,
    }
}
