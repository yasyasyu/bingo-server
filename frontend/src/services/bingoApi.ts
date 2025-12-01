export interface NumberResponse {
    number: number | null
    history: number[]
    message: string
    seed: number
}

const API_BASE = 'http://localhost:3000'

export const bingoApi = {
    async fetchNextNumber(): Promise<NumberResponse | null> {
        try {
            const res = await fetch(`${API_BASE}/next`)
            if (!res.ok) throw new Error('Network response was not ok')
            return await res.json()
        } catch (e) {
            console.error('Failed to fetch next number:', e)
            return null
        }
    },

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
