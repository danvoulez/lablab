<template>
  <div :class="spinnerContainerClasses">
    <div :class="spinnerClasses">
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
    </div>
    <p v-if="text" class="spinner-text">{{ text }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  size?: 'sm' | 'md' | 'lg' | 'xl'
  variant?: 'primary' | 'cyan' | 'purple' | 'white'
  text?: string
  fullscreen?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  variant: 'cyan',
  fullscreen: false
})

const spinnerContainerClasses = computed(() => [
  'spinner-container',
  {
    'spinner-fullscreen': props.fullscreen
  }
])

const spinnerClasses = computed(() => [
  'spinner',
  `spinner-${props.size}`,
  `spinner-${props.variant}`
])
</script>

<style scoped>
.spinner-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
}

.spinner-fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--color-overlay-strong);
  backdrop-filter: blur(var(--blur-base));
  -webkit-backdrop-filter: blur(var(--blur-base));
  z-index: var(--z-modal);
}

.spinner {
  position: relative;
  display: inline-block;
}

/* Sizes */
.spinner-sm {
  width: 24px;
  height: 24px;
}

.spinner-md {
  width: 40px;
  height: 40px;
}

.spinner-lg {
  width: 64px;
  height: 64px;
}

.spinner-xl {
  width: 96px;
  height: 96px;
}

/* Ring Animation */
.spinner-ring {
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  opacity: 0;
  animation: ripple 1.5s cubic-bezier(0, 0.2, 0.8, 1) infinite;
}

.spinner-ring:nth-child(2) {
  animation-delay: -0.5s;
}

.spinner-ring:nth-child(3) {
  animation-delay: -1s;
}

.spinner-ring:nth-child(4) {
  animation-delay: -1.5s;
}

@keyframes ripple {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

/* Color Variants */
.spinner-primary .spinner-ring {
  border: 3px solid var(--color-primary-500);
}

.spinner-cyan .spinner-ring {
  border: 3px solid var(--color-cyan-500);
  box-shadow: 0 0 12px rgba(0, 212, 255, 0.4);
}

.spinner-purple .spinner-ring {
  border: 3px solid var(--color-purple-500);
  box-shadow: 0 0 12px rgba(128, 0, 255, 0.4);
}

.spinner-white .spinner-ring {
  border: 3px solid var(--color-text-primary);
}

/* Text */
.spinner-text {
  color: var(--color-text-secondary);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  margin: 0;
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
