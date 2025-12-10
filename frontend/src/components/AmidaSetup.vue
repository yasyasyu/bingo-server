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

const getLabel = (index: number) => String.fromCharCode(65 + index) // A, B, C, ...
</script>

<template>
    <div class="setup-panel">
        <h2>Enter Guest Names</h2>
        <p class="description">誰がどのプレゼントを手にするのでしょうか？</p>
        <div class="inputs-container-grid">
            <div v-for="(item, index) in inputItems" :key="index" class="input-group">
                <label>{{ getLabel(index) }}</label>
                <input v-model="inputItems[index]" placeholder="Name..." @blur="handleBlur" />
            </div>
        </div>
        <button @click="handleStart" :disabled="isLoading" class="start-btn">
            Start Game
        </button>
    </div>
</template>

<style scoped>
h2 {
    font-size: 3rem;
    color: #d4af37;
    margin-bottom: 10px;
}

.setup-panel {
    background: rgba(0, 0, 0, 0.6);
    padding: 60px 80px;
    border-radius: 20px;
    text-align: center;
    max-width: 1600px;
    width: 95%;
}

.description {
    color: rgba(255, 255, 255, 0.8);
    font-size: 2.2rem;
    margin: 20px 0 40px 0;
}

.inputs-container-grid {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 40px;
    margin-bottom: 50px;
    width: 100%;
}

.input-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 15px;
    flex: 0 0 auto;
    width: calc(25% - 40px);
    min-width: 200px;
}

.input-group label {
    font-weight: bold;
    font-size: 2.5rem;
    color: #d4af37;
}

.input-group input {
    padding: 20px 15px;
    border-radius: 10px;
    border: none;
    width: 100%;
    max-width: 250px;
    text-align: center;
    font-size: 1.8rem;
}

.start-btn {
    padding: 20px 60px;
    font-size: 3rem;
    background: #d4af37;
    border: none;
    border-radius: 35px;
    cursor: pointer;
    font-weight: bold;
    color: #1a472a;
    transition: transform 0.2s;
}

.start-btn:hover {
    transform: scale(1.05);
}

.start-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}
</style>
