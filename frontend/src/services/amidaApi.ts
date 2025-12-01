const API_BASE = 'http://localhost:3000'

export interface AmidaResponse {
    items: string[]
    message: string
}

export interface AmidaResultResponse {
    items: [string, string][]
    message: string
}

/**
 * あみだくじに関するAPI呼び出しを行うサービス
 */
export const amidaApi = {
    /**
     * 現在のあみだくじの設定（景品リストなど）を取得します
     */
    async fetchSettings(): Promise<AmidaResponse> {
        const res = await fetch(`${API_BASE}/amida`)
        if (!res.ok) throw new Error('Failed to fetch amida settings')
        return res.json()
    },

    /**
     * あみだくじの設定（景品リスト）を更新します
     * @param items - 新しい景品リスト
     */
    async updateSettings(items: string[]): Promise<AmidaResponse> {
        const res = await fetch(`${API_BASE}/amida`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ items })
        })
        if (!res.ok) throw new Error('Failed to update amida settings')
        return res.json()
    },

    /**
     * あみだくじの結果（ゲストと景品のペア）を取得します
     * サーバー側でシャッフルされた結果が返されます
     */
    async fetchResults(): Promise<AmidaResultResponse> {
        const res = await fetch(`${API_BASE}/amida/result`)
        if (!res.ok) throw new Error('Failed to fetch amida results')
        return res.json()
    }
}
