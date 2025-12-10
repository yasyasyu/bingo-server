import { ref } from 'vue'
import { amidaApi } from '../services/amidaApi'

// Global State
const items = ref<string[]>(new Array(8).fill(''))
const isConfigured = ref(false)
const isLoading = ref(false)
const error = ref<string | null>(null)
const seed = ref<number | null>(null)

export function useAmida() {
    const fetchAmida = async () => {
        isLoading.value = true
        try {
            const data = await amidaApi.fetchSettings()

            // Check if items are actually set (not all empty)
            const hasContent = data.items.some(item => item.trim() !== '')
            if (hasContent) {
                items.value = data.items
                isConfigured.value = true
            } else {
                isConfigured.value = false
            }
            seed.value = data.seed
        } catch (e) {
            error.value = e instanceof Error ? e.message : 'Unknown error'
        } finally {
            isLoading.value = false
        }
    }

    const setupAmida = async (newItems: string[]) => {
        isLoading.value = true
        try {
            console.log('Setting up Amida with items:', newItems)
            const data = await amidaApi.updateSettings(newItems)
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
            const data = await amidaApi.fetchResults()
            console.log('Fetched Amida results:', data.items)
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
        seed,
        fetchAmida,
        setupAmida,
        fetchResults
    }
}
