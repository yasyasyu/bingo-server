<script setup lang="ts">
defineProps<{
  displayText: string | number
  currentNumber: number | null
  isSpinning: boolean
}>()
</script>

<template>
  <div class="display-area">
    <div class="number-display" :class="{ pop: !isSpinning && currentNumber }">
      {{ displayText }}
    </div>
  </div>
</template>

<style scoped>
.display-area {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  width: 300px;
  height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 10px solid #c41e3a; /* Christmas Red */
  box-shadow: 0 0 30px rgba(196, 30, 58, 0.5);
  margin-bottom: 30px;
  background-image: radial-gradient(circle, #fff 0%, #eee 100%);
}

.number-display {
  font-size: 8rem;
  font-weight: bold;
  color: #c41e3a;
  text-shadow: 2px 2px 0px #fff;
}

.pop {
  animation: pop 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes pop {
  0% {
    transform: scale(0.5);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}
</style>
