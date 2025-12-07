<script setup lang="ts">
defineProps<{
    isSpinning: boolean
}>()

defineEmits<{
    (e: 'reset'): void
}>()
</script>

<template>
    <div class="controls">
        <button @click="$emit('reset')" :disabled="isSpinning" class="reset-btn">
            RESET
        </button>
    </div>
</template>

<style scoped>
.controls {
    display: flex;
    margin-bottom: 30px;
    bottom: 10px;
    right: 5px;
    position: absolute;
}

button {
    padding: 5px 15px;
    font-size: 0.8 rem;
    border: none;
    border-radius: 50px;
    cursor: pointer;
    transition: transform 0.1s, box-shadow 0.1s;
    font-weight: bold;
    text-transform: uppercase;
}

button:active {
    transform: scale(0.95);
}

button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.spin-btn {
    background: #d4af37;
    color: #1a472a;
    box-shadow: 0 5px 15px rgba(212, 175, 55, 0.4);
}

.reset-btn {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 2px solid white;
}
</style>
