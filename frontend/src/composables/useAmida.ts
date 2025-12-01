import { ref } from 'vue'

const API_BASE = 'http://localhost:3000'

export interface AmidaResponse {
    items: string[]
    message: string
}

// Global State
const items = ref<string[]>(new Array(10).fill(''))
const isConfigured = ref(false)
const isLoading = ref(false)
const error = ref<string | null>(null)

export function useAmida() {
    const fetchAmida = async () => {
        isLoading.value = true
        try {
            const res = await fetch(`${API_BASE}/amida`)
            if (!res.ok) throw new Error('Failed to fetch amida')
            const data: AmidaResponse = await res.json()

            // Check if items are actually set (not all empty)
            const hasContent = data.items.some(item => item.trim() !== '')
            if (hasContent) {
                items.value = data.items
                isConfigured.value = true
            } else {
                isConfigured.value = false
            }
        } catch (e) {
            error.value = e instanceof Error ? e.message : 'Unknown error'
        } finally {
            isLoading.value = false
        }
    }

    const setupAmida = async (newItems: string[]) => {
        isLoading.value = true
        try {
            const res = await fetch(`${API_BASE}/amida`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ items: newItems })
            })
            if (!res.ok) throw new Error('Failed to setup amida')
            const data: AmidaResponse = await res.json()
            items.value = data.items
            isConfigured.value = true
        } catch (e) {
            error.value = e instanceof Error ? e.message : 'Unknown error'
        } finally {
            isLoading.value = false
        }
    }

    const fetchResults = async () => {
        isLoading.value = true
        try {
            const res = await fetch(`${API_BASE}/amida/result`)
            if (!res.ok) throw new Error('Failed to fetch results')
            const data: { items: [string, string][], message: string } = await res.json()
            return data.items
        } catch (e) {
            error.value = e instanceof Error ? e.message : 'Unknown error'
            return null
        } finally {
            isLoading.value = false
        }
    }

    return {
        items,
        isConfigured,
        isLoading,
        error,
        fetchAmida,
        setupAmida,
        fetchResults
    }
}
