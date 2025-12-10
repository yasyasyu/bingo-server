import { ref } from 'vue'
import { amidaApi } from '../services/amidaApi'

// Global State
const items = ref<string[]>(new Array(8).fill(''))
const prizeCount = ref(8)
const isConfigured = ref(false)
const isLoading = ref(false)
const error = ref<string | null>(null)
const seed = ref<number | null>(null)

export function useAmida() {
    const fetchAmida = async () => {
        isLoading.value = true
        try {
            const data = await amidaApi.fetchSettings()
            prizeCount.value = data.prize_count

            let fetchedItems = data.items
            // Resize to match prizeCount
            if (fetchedItems.length < prizeCount.value) {
                fetchedItems = [...fetchedItems, ...new Array(prizeCount.value - fetchedItems.length).fill('')]
            } else if (fetchedItems.length > prizeCount.value) {
                fetchedItems = fetchedItems.slice(0, prizeCount.value)
            }

            // Check if items are actually set (not all empty)
            const hasContent = fetchedItems.some(item => item.trim() !== '')
            if (hasContent) {
                items.value = fetchedItems
                isConfigured.value = true
            } else {
                items.value = new Array(prizeCount.value).fill('')
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
            prizeCount.value = data.prize_count
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
        prizeCount,
        isConfigured,
        isLoading,
        error,
        seed,
        fetchAmida,
        setupAmida,
        fetchResults
    }
}
