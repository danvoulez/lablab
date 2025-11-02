<template>
  <div id="app">
    <!-- Animated Background -->
    <div class="background-animation">
      <div class="gradient-orb gradient-orb-1"></div>
      <div class="gradient-orb gradient-orb-2"></div>
      <div class="gradient-orb gradient-orb-3"></div>
    </div>

    <!-- Header with Glassmorphism -->
    <header class="app-header">
      <div class="header-content">
        <div class="logo-container">
          <div class="logo-icon">
            <svg width="40" height="40" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="20" cy="20" r="18" stroke="url(#logo-gradient)" stroke-width="2" />
              <path d="M20 8L20 32M12 20L28 20" stroke="url(#logo-gradient)" stroke-width="2" stroke-linecap="round" />
              <circle cx="20" cy="20" r="4" fill="url(#logo-gradient)" />
              <defs>
                <linearGradient id="logo-gradient" x1="0" y1="0" x2="40" y2="40" gradientUnits="userSpaceOnUse">
                  <stop stop-color="#00d4ff" />
                  <stop offset="1" stop-color="#8000ff" />
                </linearGradient>
              </defs>
            </svg>
          </div>
          <div class="logo-text">
            <h1>LogLine Discovery Lab</h1>
            <p class="subtitle">HIV Drug Discovery • Digital Twin Monitoring</p>
          </div>
        </div>
        <div class="header-actions">
          <div class="status-indicator">
            <span class="status-dot"></span>
            <span class="status-text">Live Monitoring</span>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="app-main">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>

    <!-- Footer -->
    <footer class="app-footer">
      <div class="footer-content">
        <p>&copy; 2024 LogLine Discovery Lab</p>
        <span class="footer-separator">•</span>
        <p>Real-time causal inference and molecular analysis</p>
        <span class="footer-separator">•</span>
        <p>Powered by Digital Twin Technology</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
// App component - main layout wrapper with stunning animations
</script>

<style scoped>
#app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow-x: hidden;
}

/* ============================================
   ANIMATED BACKGROUND
   ============================================ */
.background-animation {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  overflow: hidden;
  pointer-events: none;
}

.gradient-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.15;
  animation: float 20s ease-in-out infinite;
}

.gradient-orb-1 {
  width: 600px;
  height: 600px;
  background: radial-gradient(circle, #00d4ff 0%, transparent 70%);
  top: -200px;
  left: -200px;
  animation-delay: 0s;
}

.gradient-orb-2 {
  width: 500px;
  height: 500px;
  background: radial-gradient(circle, #8000ff 0%, transparent 70%);
  top: 50%;
  right: -150px;
  animation-delay: -7s;
}

.gradient-orb-3 {
  width: 450px;
  height: 450px;
  background: radial-gradient(circle, #00f2c3 0%, transparent 70%);
  bottom: -100px;
  left: 30%;
  animation-delay: -14s;
}

@keyframes float {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(30px, -30px) scale(1.1);
  }
  66% {
    transform: translate(-20px, 20px) scale(0.9);
  }
}

/* ============================================
   HEADER
   ============================================ */
.app-header {
  position: sticky;
  top: 0;
  z-index: var(--z-sticky);
  background: var(--glass-strong);
  backdrop-filter: blur(var(--blur-xl));
  -webkit-backdrop-filter: blur(var(--blur-xl));
  border-bottom: 1px solid var(--color-border-subtle);
  box-shadow: var(--shadow-lg);
  animation: slideDown 0.5s var(--ease-out);
}

@keyframes slideDown {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.header-content {
  max-width: 1600px;
  margin: 0 auto;
  padding: var(--space-4) var(--space-8);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
}

.logo-container {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.logo-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  animation: rotate 20s linear infinite;
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.logo-icon svg {
  filter: drop-shadow(0 0 10px rgba(0, 212, 255, 0.5));
}

.logo-text h1 {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  background: linear-gradient(135deg, #00d4ff 0%, #8000ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin: 0;
  letter-spacing: var(--letter-spacing-tight);
  line-height: 1.2;
}

.subtitle {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  font-weight: var(--font-weight-medium);
  margin: var(--space-1) 0 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  background: var(--glass-subtle);
  border-radius: var(--radius-full);
  border: 1px solid var(--color-border-subtle);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-success-light);
  box-shadow: 0 0 12px var(--color-success);
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.7;
    transform: scale(1.1);
  }
}

.status-text {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-secondary);
}

/* ============================================
   MAIN CONTENT
   ============================================ */
.app-main {
  flex: 1;
  padding: var(--space-8);
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
  position: relative;
  z-index: 1;
}

/* Page Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--duration-moderate) var(--ease-in-out),
              transform var(--duration-moderate) var(--ease-out);
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.fade-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

/* ============================================
   FOOTER
   ============================================ */
.app-footer {
  position: relative;
  z-index: 1;
  background: var(--glass-base);
  backdrop-filter: blur(var(--blur-lg));
  -webkit-backdrop-filter: blur(var(--blur-lg));
  border-top: 1px solid var(--color-border-subtle);
  padding: var(--space-6) var(--space-8);
}

.footer-content {
  max-width: 1600px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
  flex-wrap: wrap;
}

.footer-content p {
  margin: 0;
}

.footer-separator {
  color: var(--color-border-strong);
}

/* ============================================
   RESPONSIVE DESIGN
   ============================================ */
@media (max-width: 1024px) {
  .header-content {
    padding: var(--space-4) var(--space-6);
  }

  .logo-text h1 {
    font-size: var(--font-size-xl);
  }

  .app-main {
    padding: var(--space-6);
  }
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
    padding: var(--space-4);
  }

  .logo-container {
    gap: var(--space-3);
  }

  .logo-icon svg {
    width: 32px;
    height: 32px;
  }

  .logo-text h1 {
    font-size: var(--font-size-lg);
  }

  .subtitle {
    font-size: var(--font-size-xs);
  }

  .header-actions {
    width: 100%;
    justify-content: flex-start;
  }

  .app-main {
    padding: var(--space-4);
  }

  .footer-content {
    flex-direction: column;
    gap: var(--space-2);
    text-align: center;
  }

  .footer-separator {
    display: none;
  }

  .gradient-orb {
    filter: blur(60px);
  }
}

@media (max-width: 480px) {
  .logo-text h1 {
    font-size: var(--font-size-base);
  }

  .app-main {
    padding: var(--space-3);
  }
}
</style>
