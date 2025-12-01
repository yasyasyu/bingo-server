<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  items: string[]
  isLoading: boolean
}>()

const emit = defineEmits<{
  (e: 'update:items', items: string[]): void
  (e: 'save'): void
  (e: 'start'): void
}>()

const inputItems = ref<string[]>([...props.items])

watch(() => props.items, (newItems) => {
  inputItems.value = [...newItems]
})

const handleBlur = () => {
  emit('update:items', inputItems.value)
  emit('save')
}

const handleStart = () => {
  emit('update:items', inputItems.value)
  emit('start')
}
</script>

<template>
  <div class="setup-panel">
    <h2>Enter 10 Items</h2>
    <div class="inputs-grid">
      <div v-for="(_item, index) in inputItems" :key="index" class="input-group">
        <label>{{ index + 1 }}</label>
        <input v-model="inputItems[index]" placeholder="Item name..." @blur="handleBlur" />
      </div>
    </div>
    <button @click="handleStart" :disabled="isLoading" class="start-btn">
      Start Game
    </button>
  </div>
</template>

<style scoped>
.setup-panel {
  background: rgba(0, 0, 0, 0.6);
  padding: 30px;
  border-radius: 15px;
  text-align: center;
}

.inputs-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  margin-bottom: 20px;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.input-group label {
  width: 20px;
  font-weight: bold;
}

.input-group input {
  padding: 8px;
  border-radius: 5px;
  border: none;
  width: 200px;
}

.start-btn {
  padding: 10px 30px;
  font-size: 1.2rem;
  background: #d4af37;
  border: none;
  border-radius: 25px;
  cursor: pointer;
  font-weight: bold;
  color: #1a472a;
  transition: transform 0.2s;
}

.start-btn:hover {
  transform: scale(1.05);
}
</style>
