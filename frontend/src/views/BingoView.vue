<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useBingoGame } from '../composables/useBingoGame'
import { useAmida } from '../composables/useAmida'
import BingoDisplay from '../components/BingoDisplay.vue'
import BingoControls from '../components/BingoControls.vue'
import BingoHistory from '../components/BingoHistory.vue'

const {
    currentNumber,
    displayText,
    history,
    isSpinning,
    seed,
    spin,
    resetGame,
    isSoundEnabled
} = useBingoGame()

const { items, fetchAmida, setupAmida } = useAmida()
const showAmidaSettings = ref(true)

onMounted(() => {
    fetchAmida()
})

const updateItem = async (index: number, value: string) => {
    const newItems = [...items.value]
    newItems[index] = value
    await setupAmida(newItems)
}

const getLabel = (index: number) => String.fromCharCode(65 + index)
</script>

<template>
    <div class="bingo-view">
        <button class="settings-toggle" @click="showAmidaSettings = !showAmidaSettings">
            {{ showAmidaSettings ? 'Hide Settings' : 'Show Settings' }}
        </button>
        <h1 class="title">🎄 Christmas Bingo 🎄</h1>

        <div class="main-section">
            <!-- Left Panel (A-D) -->
            <div class="side-panel left" v-if="showAmidaSettings">
                <div v-for="i in 4" :key="i - 1" class="amida-item">
                    <label>{{ getLabel(i - 1) }}</label>
                    <input :value="items[i - 1]"
                        @change="(e) => updateItem(i - 1, (e.target as HTMLInputElement).value)"
                        placeholder="Name..." />
                </div>
            </div>

            <div class="center-display">
                <BingoDisplay :display-text="displayText" :current-number="currentNumber" :is-spinning="isSpinning"
                    @spin="spin" />
            </div>

            <!-- Right Panel (E-H) -->
            <div class="side-panel right" v-if="showAmidaSettings">
                <div v-for="i in 4" :key="i + 3" class="amida-item">
                    <label>{{ getLabel(i + 3) }}</label>
                    <input :value="items[i + 3]"
                        @change="(e) => updateItem(i + 3, (e.target as HTMLInputElement).value)"
                        placeholder="Name..." />
                </div>
            </div>
        </div>

        <div class="bottom-section">
            <BingoHistory :history="history" />
            <BingoControls :is-spinning="isSpinning" v-model:is-sound-enabled="isSoundEnabled" @reset="resetGame" />
        </div>

        <div v-if="seed" class="seed-display">Seed: {{ seed }}</div>
    </div>
</template>

<style scoped>
.bingo-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    width: 100%;
    position: relative;
    z-index: 1;
    padding-top: 20px;
}

.settings-toggle {
    position: absolute;
    top: 20px;
    right: 20px;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(212, 175, 55, 0.5);
    color: #d4af37;
    padding: 8px 15px;
    border-radius: 5px;
    cursor: pointer;
    z-index: 10;
    font-family: inherit;
    transition: all 0.3s ease;
}

.settings-toggle:hover {
    background: rgba(0, 0, 0, 0.6);
    border-color: #d4af37;
}

.title {
    font-size: 3rem;
    color: #d4af37;
    /* Gold */
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
    margin-bottom: 20px;
    font-family: cursive;
    margin-block-start: 0;
}

.main-section {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    gap: 160px;
}

.side-panel {
    width: 310px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 25px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 15px;
}

.center-display {
    display: flex;
    justify-content: center;
    align-items: center;
}

.bottom-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    gap: 10px;
}

.amida-item {
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
}

.amida-item:last-child {
    margin-bottom: 0;
}

.amida-item label {
    font-weight: bold;
    font-size: 2rem;
    color: #d4af37;
    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.amida-item input {
    padding: 12px;
    border-radius: 8px;
    border: 2px solid rgba(212, 175, 55, 0.3);
    background: rgba(255, 255, 255, 0.9);
    width: 100%;
    text-align: center;
    font-size: 1.4rem;
    font-weight: bold;
    color: #333;
}

.amida-item input:focus {
    outline: none;
    border-color: #d4af37;
    box-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.seed-display {
    position: absolute;
    bottom: 35px;
    right: 10px;
    font-size: 1.5rem;
    color: rgba(255, 255, 255, 0.5);
    font-family: monospace;
}
</style>
