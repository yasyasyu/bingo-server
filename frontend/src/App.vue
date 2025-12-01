<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import SnowEffect from './components/SnowEffect.vue'
</script>

<template>
  <div class="app-container">
    <SnowEffect />
    <nav class="nav-bar">
      <RouterLink to="/" class="nav-link">Bingo</RouterLink>
      <RouterLink to="/amida" class="nav-link">Amidakuji</RouterLink>
    </nav>

    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style>
/* グローバルスタイル */
body {
  margin: 0;
  background: linear-gradient(135deg, #1a472a, #2d5a27);
  color: white;
  font-family: 'Arial Rounded MT Bold', 'Helvetica Rounded', Arial, sans-serif;
  overflow: hidden;
}
</style>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.nav-bar {
  display: flex;
  justify-content: center;
  gap: 20px;
  padding: 10px;
  background-color: rgba(0, 0, 0, 0.2);
  z-index: 10;
}

.nav-link {
  color: rgba(255, 255, 255, 0.7);
  text-decoration: none;
  font-size: 1.2rem;
  font-weight: bold;
  padding: 5px 10px;
  border-radius: 5px;
  transition: all 0.3s ease;
}

.nav-link:hover, .nav-link.router-link-active {
  color: #d4af37;
  background-color: rgba(255, 255, 255, 0.1);
}

.main-content {
  flex: 1;
  position: relative;
  overflow: hidden;
}
</style>
