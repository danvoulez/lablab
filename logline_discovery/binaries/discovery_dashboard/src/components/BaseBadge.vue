<template>
  <span :class="badgeClasses">
    <span v-if="dot" class="badge-dot" :class="`badge-dot-${variant}`"></span>
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'error' | 'info' | 'purple'
  size?: 'sm' | 'md' | 'lg'
  dot?: boolean
  outline?: boolean
  glow?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  size: 'md',
  dot: false,
  outline: false,
  glow: false
})

const badgeClasses = computed(() => [
  'badge',
  `badge-${props.variant}`,
  `badge-${props.size}`,
  {
    'badge-outline': props.outline,
    'badge-glow': props.glow,
    'badge-with-dot': props.dot
  }
])
</script>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: var(--font-weight-medium);
  border-radius: var(--radius-full);
  white-space: nowrap;
  transition: var(--transition-base);
}

/* Sizes */
.badge-sm {
  padding: var(--space-1) var(--space-3);
  font-size: var(--font-size-xs);
}

.badge-md {
  padding: var(--space-2) var(--space-4);
  font-size: var(--font-size-sm);
}

.badge-lg {
  padding: var(--space-3) var(--space-5);
  font-size: var(--font-size-base);
}

/* Dot Indicator */
.badge-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
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

/* Color Variants - Solid */
.badge-default {
  background: var(--color-surface-base);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border-base);
}

.badge-primary {
  background: var(--color-primary-600);
  color: var(--color-text-primary);
}

.badge-success {
  background: var(--color-success-dark);
  color: var(--color-text-primary);
}

.badge-warning {
  background: var(--color-warning-dark);
  color: var(--color-background-dark);
}

.badge-error {
  background: var(--color-error);
  color: var(--color-text-primary);
}

.badge-info {
  background: var(--color-info);
  color: var(--color-text-primary);
}

.badge-purple {
  background: var(--color-purple-600);
  color: var(--color-text-primary);
}

/* Outline Variants */
.badge-outline {
  background: transparent;
}

.badge-default.badge-outline {
  border-color: var(--color-text-tertiary);
  color: var(--color-text-secondary);
}

.badge-primary.badge-outline {
  border: 1px solid var(--color-primary-500);
  color: var(--color-primary-400);
}

.badge-success.badge-outline {
  border: 1px solid var(--color-success);
  color: var(--color-success-light);
}

.badge-warning.badge-outline {
  border: 1px solid var(--color-warning);
  color: var(--color-warning-light);
}

.badge-error.badge-outline {
  border: 1px solid var(--color-error);
  color: var(--color-error-light);
}

.badge-info.badge-outline {
  border: 1px solid var(--color-info);
  color: var(--color-info-light);
}

.badge-purple.badge-outline {
  border: 1px solid var(--color-purple-500);
  color: var(--color-purple-400);
}

/* Glow Effect */
.badge-glow.badge-primary {
  box-shadow: 0 0 12px rgba(0, 102, 255, 0.5);
}

.badge-glow.badge-success {
  box-shadow: 0 0 12px rgba(0, 217, 168, 0.5);
}

.badge-glow.badge-warning {
  box-shadow: 0 0 12px rgba(255, 193, 7, 0.5);
}

.badge-glow.badge-error {
  box-shadow: 0 0 12px rgba(220, 53, 69, 0.5);
}

.badge-glow.badge-info {
  box-shadow: 0 0 12px rgba(0, 212, 255, 0.5);
}

.badge-glow.badge-purple {
  box-shadow: 0 0 12px rgba(128, 0, 255, 0.5);
}

/* Dot Colors */
.badge-dot-default {
  background: var(--color-text-tertiary);
}

.badge-dot-primary {
  background: var(--color-primary-400);
}

.badge-dot-success {
  background: var(--color-success-light);
}

.badge-dot-warning {
  background: var(--color-warning-light);
}

.badge-dot-error {
  background: var(--color-error-light);
}

.badge-dot-info {
  background: var(--color-info-light);
}

.badge-dot-purple {
  background: var(--color-purple-400);
}
</style>
