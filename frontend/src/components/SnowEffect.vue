<template>
  <div class="snow-container">
    <div
      v-for="n in 50"
      :key="n"
      class="snowflake"
      :style="{
        left: Math.random() * 100 + '%',
        animationDuration: Math.random() * 3 + 2 + 's',
        animationDelay: Math.random() * 5 + 's',
      }"
    >
      ❄
    </div>
  </div>
</template>

<style scoped>
.snow-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
  overflow: hidden;
}

.snowflake {
  position: absolute;
  top: -20px;
  color: white;
  font-size: 1.5em;
  opacity: 0.8;
  animation-name: fall;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
}

@keyframes fall {
  0% {
    transform: translateY(0) rotate(0deg);
  }
  100% {
    transform: translateY(100vh) rotate(360deg);
  }
}
</style>
