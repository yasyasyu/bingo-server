import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useBingoGame } from './useBingoGame'
import { bingoApi } from '../services/bingoApi'

// モックの定義
vi.mock('../services/bingoApi', () => ({
    bingoApi: {
        fetchNextNumber: vi.fn(),
        resetGame: vi.fn(),
    },
}))

vi.mock('./useAudio', () => ({
    useAudio: () => ({
        playBeep: vi.fn(),
        playWin: vi.fn(),
        resumeAudioContext: vi.fn(),
    }),
}))

// window.confirm のモック
window.confirm = vi.fn(() => true)
// window.alert のモック
window.alert = vi.fn()

describe('useBingoGame', () => {
    beforeEach(() => {
        vi.clearAllMocks()
    })

    it('初期状態が正しいこと', () => {
        const { currentNumber, displayText, history, isSpinning } = useBingoGame()

        expect(currentNumber.value).toBeNull()
        expect(displayText.value).toBe('Merry Christmas!')
        expect(history.value).toEqual([])
        expect(isSpinning.value).toBe(false)
    })

    it('spinを実行するとAPIから数字を取得して状態が更新されること', async () => {
        // requestAnimationFrame と setTimeout のモックが必要だが、
        // 今回はアニメーション完了後の状態をテストするのは難しい（非同期ループがあるため）。
        // なので、API呼び出しとフラグの変化を中心にテストする。

        // タイマーをモックして時間を進められるようにする
        vi.useFakeTimers()

        const mockData = {
            number: 42,
            history: [1, 2, 42],
            message: 'Success',
            seed: 123456789,
        }
        vi.mocked(bingoApi.fetchNextNumber).mockResolvedValue(mockData)

        const { spin, isSpinning, currentNumber, history, displayText } = useBingoGame()

        // スピン開始
        spin()

        expect(isSpinning.value).toBe(true)
        expect(bingoApi.fetchNextNumber).toHaveBeenCalled()

        // アニメーション時間を進める (3秒 + α)
        await vi.advanceTimersByTimeAsync(4000)

        // アニメーションループを完了させるために少し待つ必要があるかもしれないが、
        // useBingoGameの実装では再帰的にsetTimeoutしているので、advanceTimersByTimeで進むはず。

        // 注意: useBingoGameのアニメーションロジックは requestAnimationFrame を使っている。
        // jsdom環境では requestAnimationFrame はシミュレートされるが、
        // vi.advanceTimersByTime との兼ね合いが難しい場合がある。
        // ここでは簡易的に、APIが呼ばれたことと、開始直後の状態を確認するにとどめるのが安全かもしれないが、
        // 完了まで確認したい。

        // requestAnimationFrameのモック
        vi.stubGlobal('requestAnimationFrame', (cb: FrameRequestCallback) => {
            return setTimeout(() => cb(Date.now()), 16)
        })

        // もう一度時間を進める
        await vi.advanceTimersByTimeAsync(5000)

        expect(isSpinning.value).toBe(false)
        expect(currentNumber.value).toBe(42)
        expect(displayText.value).toBe(42)
        expect(history.value).toEqual([1, 2, 42])

        vi.useRealTimers()
    })

    it('resetGameを実行すると状態が初期化されること', async () => {
        const { resetGame, currentNumber, displayText, history } = useBingoGame()

        // 状態を汚す
        currentNumber.value = 99
        displayText.value = 99
        history.value = [1, 2, 3]

        await resetGame()

        expect(bingoApi.resetGame).toHaveBeenCalled()
        expect(currentNumber.value).toBeNull()
        expect(displayText.value).toBe('Merry Christmas!')
        expect(history.value).toEqual([])
    })
})
