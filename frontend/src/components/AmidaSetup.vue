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
        <div class="inputs-row">
            <div v-for="(_item, index) in inputItems" :key="index" class="input-group">
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
.setup-panel {
    background: rgba(0, 0, 0, 0.6);
    padding: 40px 50px;
    border-radius: 15px;
    text-align: center;
    max-width: 1800px;
    width: 90%;
}

.description {
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.95rem;
    margin: 10px 0 20px 0;
}

.inputs-row {
    display: flex;
    justify-content: space-between;
    gap: 15px;
    margin-bottom: 30px;
    width: 100%;
}

.input-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    flex: 1;
}

.input-group label {
    font-weight: bold;
    font-size: 1.5rem;
    color: #d4af37;
}

.input-group input {
    padding: 15px 10px;
    border-radius: 5px;
    border: none;
    width: 100%;
    max-width: 150px;
    text-align: center;
    font-size: 1rem;
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

.start-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}
</style>
