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
        <p class="description">誰がどのプレゼントを手にするのでしょうか？</p>
        <div class="inputs-container-grid">
            <div v-for="(_, index) in inputItems" :key="index" class="input-group">
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
    padding: 50px 50px;
    border-radius: 15px;
    text-align: center;
    max-width: 1800px;
    width: 90%;
}

.description {
    color: rgba(255, 255, 255, 0.8);
    font-size: 2.2rem;
    margin: 20px 0 40px 0;
}

.inputs-container-grid {
    display: flex;
    overflow-wrap: anywhere;
    gap: 50px;
    margin-bottom: 50px;
    width: 100%;
}

.inputs-row {
    display: flex;
    justify-content: center;
    gap: 50px;
    width: 100%;
}

.input-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 15px;
    flex: 1;
}

.input-group label {
    font-weight: bold;
    font-size: 2.5rem;
    color: #d4af37;
}

.input-group input {
    padding: 10px 10px;
    border-radius: 10px;
    border: none;
    width: 100%;
    max-width: 260px;
    height: 70px;
    text-align: center;
    font-size: 1.5rem;
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
