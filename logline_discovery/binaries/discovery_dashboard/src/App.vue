<template>
  <div id="app">
    <header class="app-header">
      <h1>LogLine Discovery Lab</h1>
      <p class="subtitle">24/7 Automated Scientific Discovery Pipeline</p>
    </header>

    <main class="app-main">
      <router-view />
    </main>

    <footer class="app-footer">
      <p>&copy; 2024 LogLine Discovery Lab. Real-time causal inference and molecular analysis.</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
// App component - main layout wrapper
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: linear-gradient(135deg, #0c1021 0%, #1a1f3a 100%);
  color: #f6f7fb;
  min-height: 100vh;
  line-height: 1.6;
}

#app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-header {
  background: rgba(22, 27, 50, 0.95);
  backdrop-filter: blur(10px);
  padding: 1.5rem 2rem;
  text-align: center;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.app-header h1 {
  font-size: 2.5rem;
  font-weight: 300;
  margin-bottom: 0.5rem;
  background: linear-gradient(45deg, #00d4ff, #090979);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.8;
  font-weight: 300;
}

.app-main {
  flex: 1;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.app-footer {
  background: rgba(22, 27, 50, 0.8);
  padding: 1rem 2rem;
  text-align: center;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  font-size: 0.9rem;
  opacity: 0.7;
}

/* Responsive design */
@media (max-width: 768px) {
  .app-header {
    padding: 1rem;
  }

  .app-header h1 {
    font-size: 2rem;
  }

  .app-main {
    padding: 1rem;
  }
}
</style>
