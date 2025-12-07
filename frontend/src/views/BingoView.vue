<script setup lang="ts">
import { useBingoGame } from '../composables/useBingoGame'
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
    resetGame
} = useBingoGame()
</script>

<template>
    <div class="bingo-container">
        <h1 class="title">🎄 Christmas Bingo 🎄</h1>

        <BingoDisplay :display-text="displayText" :current-number="currentNumber" :is-spinning="isSpinning"
            @spin="spin" />

        <BingoHistory :history="history" />

        <BingoControls :is-spinning="isSpinning" @reset="resetGame" />

        <div v-if="seed" class="seed-display">Seed: {{ seed }}</div>
    </div>
</template>

<style scoped>
.bingo-container {
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
    /* Gold */
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
    margin-bottom: 20px;
    font-family: cursive;
    margin-block-start: 0;
}

.seed-display {
    position: absolute;
    bottom: 10px;
    right: 10px;
    font-size: 1rem;
    color: rgba(255, 255, 255, 0.5);
    font-family: monospace;
}
</style>
