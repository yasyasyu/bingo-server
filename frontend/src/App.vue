<script setup lang="ts">
import { useBingoGame } from './composables/useBingoGame'
import SnowEffect from './components/SnowEffect.vue'
import BingoDisplay from './components/BingoDisplay.vue'
import BingoControls from './components/BingoControls.vue'
import BingoHistory from './components/BingoHistory.vue'

const {
  currentNumber,
  displayText,
  history,
  isSpinning,
  spin,
  resetGame
} = useBingoGame()
</script>

<template>
  <div class="container">
    <SnowEffect />
    
    <h1 class="title">🎄 Christmas Bingo 🎄</h1>
    
    <BingoDisplay 
      :display-text="displayText" 
      :current-number="currentNumber" 
      :is-spinning="isSpinning" 
    />

    <BingoControls 
      :is-spinning="isSpinning" 
      @spin="spin" 
      @reset="resetGame" 
    />

    <BingoHistory :history="history" />
  </div>
</template>

<style>
/* グローバルスタイル */
body {
  margin: 0;
  background: linear-gradient(135deg, #1a472a, #2d5a27);
  color: white;
  font-family: 'Arial Rounded MT Bold', 'Helvetica Rounded', Arial, sans-serif;
  overflow: hidden;
}
</style>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  position: relative;
  z-index: 1;
}

.title {
  font-size: 3rem;
  color: #d4af37; /* Gold */
  text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
  margin-bottom: 20px;
  font-family: 'Brush Script MT', cursive;
}
</style>
