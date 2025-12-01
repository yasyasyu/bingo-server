const API_BASE = 'http://localhost:3000'

export interface AmidaResponse {
    items: string[]
    message: string
}

export interface AmidaResultResponse {
    items: [string, string][]
    message: string
}

export const amidaApi = {
    async fetchSettings(): Promise<AmidaResponse> {
        const res = await fetch(`${API_BASE}/amida`)
        if (!res.ok) throw new Error('Failed to fetch amida settings')
        return res.json()
    },

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

    async fetchResults(): Promise<AmidaResultResponse> {
        const res = await fetch(`${API_BASE}/amida/result`)
        if (!res.ok) throw new Error('Failed to fetch amida results')
        return res.json()
    }
}
