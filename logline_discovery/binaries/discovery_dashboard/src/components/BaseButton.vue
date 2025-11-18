<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    :type="type"
    @click="handleClick"
  >
    <span v-if="loading" class="btn-spinner"></span>
    <span class="btn-content" :class="{ 'btn-content-loading': loading }">
      <slot />
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'success'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
  type?: 'button' | 'submit' | 'reset'
  fullWidth?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
  type: 'button',
  fullWidth: false
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClasses = computed(() => [
  'btn',
  `btn-${props.variant}`,
  `btn-${props.size}`,
  {
    'btn-disabled': props.disabled,
    'btn-loading': props.loading,
    'btn-full-width': props.fullWidth
  }
])

const handleClick = (event: MouseEvent) => {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>

<style scoped>
.btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-family-base);
  font-weight: var(--font-weight-semibold);
  border: none;
  cursor: pointer;
  transition: var(--transition-base);
  outline: none;
  white-space: nowrap;
  user-select: none;
  overflow: hidden;
}

.btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, transparent 100%);
  opacity: 0;
  transition: opacity var(--duration-base) var(--ease-in-out);
}

.btn:hover::before {
  opacity: 1;
}

.btn:active {
  transform: scale(0.98);
}

/* Sizes */
.btn-sm {
  padding: var(--space-2) var(--space-4);
  font-size: var(--font-size-sm);
  border-radius: var(--radius-base);
  gap: var(--space-2);
}

.btn-md {
  padding: var(--space-3) var(--space-6);
  font-size: var(--font-size-base);
  border-radius: var(--radius-md);
  gap: var(--space-2);
}

.btn-lg {
  padding: var(--space-4) var(--space-8);
  font-size: var(--font-size-lg);
  border-radius: var(--radius-lg);
  gap: var(--space-3);
}

/* Variants */
.btn-primary {
  background: var(--gradient-cyan-glow);
  color: var(--color-text-primary);
  box-shadow: var(--shadow-base), var(--shadow-glow-cyan);
}

.btn-primary:hover {
  box-shadow: var(--shadow-md), var(--shadow-glow-cyan);
  transform: translateY(-2px);
}

.btn-secondary {
  background: var(--color-surface-base);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-base);
  box-shadow: var(--shadow-sm);
}

.btn-secondary:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-border-strong);
  box-shadow: var(--shadow-base);
}

.btn-ghost {
  background: transparent;
  color: var(--color-cyan-500);
  border: 1px solid var(--color-cyan-500);
}

.btn-ghost:hover {
  background: rgba(0, 212, 255, 0.1);
  box-shadow: var(--shadow-glow-cyan);
}

.btn-danger {
  background: var(--gradient-error);
  color: var(--color-text-primary);
  box-shadow: var(--shadow-base), var(--shadow-glow-error);
}

.btn-danger:hover {
  box-shadow: var(--shadow-md), var(--shadow-glow-error);
  transform: translateY(-2px);
}

.btn-success {
  background: var(--gradient-success);
  color: var(--color-text-primary);
  box-shadow: var(--shadow-base), var(--shadow-glow-success);
}

.btn-success:hover {
  box-shadow: var(--shadow-md), var(--shadow-glow-success);
  transform: translateY(-2px);
}

/* States */
.btn-disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.btn-loading {
  cursor: wait;
}

.btn-full-width {
  width: 100%;
}

/* Content & Spinner */
.btn-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: inherit;
  transition: opacity var(--duration-base) var(--ease-in-out);
}

.btn-content-loading {
  opacity: 0;
}

.btn-spinner {
  position: absolute;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: var(--color-text-primary);
  border-radius: 50%;
  animation: spin var(--duration-slowest) linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
