<script setup lang="ts">
defineProps<{
  history: number[]
}>()
</script>

<template>
  <div class="history">
    <h3>History</h3>
    <div class="history-list">
      <span v-for="num in history" :key="num" class="history-item">{{ num }}</span>
    </div>
  </div>
</template>

<style scoped>
.history {
  background: rgba(0, 0, 0, 0.3);
  padding: 20px;
  border-radius: 15px;
  width: 80%;
  max-width: 600px;
}

.history h3 {
  margin-top: 0;
  text-align: center;
  color: #d4af37;
}

.history-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: center;
}

.history-item {
  background: white;
  color: #1a472a;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}
</style>
