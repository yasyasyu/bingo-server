<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAmida } from '../composables/useAmida'
import { useAmidaGame } from '../composables/useAmidaGame'
import AmidaSetup from '../components/AmidaSetup.vue'
import AmidaBoard from '../components/AmidaBoard.vue'

const router = useRouter()
const route = useRoute()
const { items, isConfigured, isLoading, seed, fetchAmida, setupAmida, fetchResults } = useAmida()
const {
    horizontalLines,
    bottomPrizes,
    generateAmida,
    calculatePrizes
} = useAmidaGame()

// Setup Mode State
const inputItems = ref<string[]>(new Array(8).fill(''))

onMounted(async () => {
    await fetchAmida()
    if (isConfigured.value) {
        inputItems.value = [...items.value]
        if (route.path === '/amida/result') {
            generateAmida()
            await updatePrizes()
        }
    } else if (route.path === '/amida/result') {
        router.replace('/amida')
    }
})

// 画面遷移時の処理
watch(() => route.path, async (newPath) => {
    if (newPath === '/amida/result') {
        if (!isConfigured.value) {
            router.replace('/amida')
            return
        }
        if (horizontalLines.value.length === 0) {
            generateAmida()
        }
        await updatePrizes()
    }
})

const saveInput = async () => {
    await setupAmida(inputItems.value)
}

const handleSubmit = async () => {
    await saveInput()
    const results = await fetchResults()
    if (results && results.length > 0) {
        router.push('/amida/result')
    } else {
        alert("Please enter all items before starting.")
    }
}

const updatePrizes = async () => {
    const results = await fetchResults()
    if (results) {
        calculatePrizes(results)
    }
}

</script>

<template>
    <div class="amida-container">
        <h1 class="title" v-if="route.path === '/amida'">🎅 Amidakuji 🎅</h1>

        <!-- Setup Mode -->
        <AmidaSetup v-if="route.path === '/amida'" v-model:items="inputItems" :is-loading="isLoading" @save="saveInput"
            @start="handleSubmit" />

        <!-- Game Mode -->
        <AmidaBoard v-else :horizontal-lines="horizontalLines" :bottom-prizes="bottomPrizes" />

        <div v-if="seed" class="seed-display">Seed: {{ seed }}</div>
    </div>
</template>

<style scoped>
.amida-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    position: relative;
    z-index: 1;
}

.title {
    font-size: 3rem;
    color: #d4af37;
    position: absolute;
    top: 0;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
    margin-bottom: 20px;
    font-family: cursive;
    margin-block-start: 0;
}

.seed-display {
    position: absolute;
    bottom: 15px;
    right: 10px;
    font-size: 1.5rem;
    color: rgba(255, 255, 255, 0.5);
    font-family: monospace;
}
</style>
