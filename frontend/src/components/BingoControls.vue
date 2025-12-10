<script setup lang="ts">
defineProps<{
    isSpinning: boolean
    isSoundEnabled: boolean
}>()

defineEmits<{
    (e: 'reset'): void
    (e: 'update:isSoundEnabled', value: boolean): void
}>()
</script>

<template>
    <div class="controls">

        <button @click="$emit('update:isSoundEnabled', !isSoundEnabled)" class="control-btn"
            :style="{ opacity: isSoundEnabled ? 1 : 0.6 }">
            Sound: {{ isSoundEnabled ? 'ON' : 'OFF' }}
        </button>
        <button @click="$emit('reset')" :disabled="isSpinning" class="reset-btn">
            RESET
        </button>
    </div>
</template>

<style scoped>
.controls {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 60px;
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


.reset-btn,
.control-btn {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 2px solid white;
}
</style>
