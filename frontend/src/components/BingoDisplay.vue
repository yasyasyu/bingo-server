<script setup lang="ts">
const props = defineProps<{
    displayText: string | number
    currentNumber: number | null
    isSpinning: boolean
}>()

defineEmits<{
    (e: 'spin'): void
}>()

const isNumber = () => typeof props.displayText === 'number'
const shouldAnimate = () => !props.isSpinning && props.currentNumber !== null
</script>

<template>
    <button class="display-area" @click="$emit('spin')" :disabled="isSpinning">
        <div :class="{
            'number-display': isNumber(),
            'text-display': !isNumber(),
            'pop': shouldAnimate()
        }">
            {{ displayText }}
        </div>
    </button>
</template>

<style scoped>
.display-area {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 50%;
    width: 550px;
    height: 550px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 10px solid #c41e3a;
    /* Christmas Red */
    box-shadow: 0 0 30px rgba(196, 30, 58, 0.5);
    margin-bottom: 30px;
    background-image: radial-gradient(circle, #fff 0%, #eee 100%);
}

.number-display {
    font-size: 26rem;
    font-weight: bold;
    color: #c41e3a;
}

.text-display {
    font-size: 6rem;
    font-weight: bold;
    color: #c41e3a;
    text-shadow: 2px 2px 2px #fff;
}

.pop {
    animation: pop 0.75s cubic-bezier(.05, .81, .41, 1);
}

@keyframes pop {
    0% {
        transform: scale(0.5);
        opacity: 0;
    }

    50% {
        transform: scale(1.4);
        opacity: 1;
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

button:disabled {
    /* opacity: 0.6; */
    cursor: not-allowed;
}
</style>
