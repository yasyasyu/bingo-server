export interface NumberResponse {
    number: number | null
    history: number[]
    message: string
    seed: number
}

const API_BASE = 'http://localhost:3000'

/**
 * ビンゴゲームに関するAPI呼び出しを行うサービス
 */
export const bingoApi = {
    /**
     * 次のビンゴ番号を取得します
     * 
     * サーバー側で乱数生成を行い、新しい番号とこれまでの履歴を返します。
     * すべての番号が出尽くした場合は null が返る可能性があります。
     */
    async fetchNextNumber(): Promise<NumberResponse | null> {
        try {
            const res = await fetch(`${API_BASE}/next_number`)
            if (!res.ok) throw new Error('Network response was not ok')
            return await res.json()
        } catch (e) {
            console.error('Failed to fetch next number:', e)
            return null
        }
    },

    /**
     * ビンゴゲームをリセットします
     * 
     * 履歴をクリアし、新しいシード値でゲームを再開します。
     */
    async resetGame(): Promise<NumberResponse | null> {
        try {
            const res = await fetch(`${API_BASE}/reset`, { method: 'POST' })
            if (!res.ok) throw new Error('Network response was not ok')
            return await res.json()
        } catch (e) {
            console.error('Failed to reset game:', e)
            throw e
        }
    },
}
