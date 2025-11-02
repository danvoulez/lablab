<template>
  <div :class="cardClasses" @click="handleClick">
    <!-- Gradient Background Effect -->
    <div v-if="glowEffect" class="card-glow" :class="`card-glow-${glowColor}`"></div>

    <!-- Header -->
    <div v-if="$slots.header || title" class="card-header">
      <slot name="header">
        <h3 class="card-title">{{ title }}</h3>
      </slot>
      <div v-if="$slots.actions" class="card-actions">
        <slot name="actions"></slot>
      </div>
    </div>

    <!-- Body -->
    <div class="card-body">
      <slot></slot>
    </div>

    <!-- Footer -->
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'glass' | 'elevated' | 'outlined' | 'flat'
  title?: string
  hoverable?: boolean
  clickable?: boolean
  glowEffect?: boolean
  glowColor?: 'cyan' | 'purple' | 'success' | 'warning' | 'error'
  padding?: 'none' | 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'glass',
  hoverable: false,
  clickable: false,
  glowEffect: false,
  glowColor: 'cyan',
  padding: 'md'
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const cardClasses = computed(() => [
  'card',
  `card-${props.variant}`,
  `card-padding-${props.padding}`,
  {
    'card-hoverable': props.hoverable,
    'card-clickable': props.clickable,
    'card-with-glow': props.glowEffect
  }
])

const handleClick = (event: MouseEvent) => {
  if (props.clickable) {
    emit('click', event)
  }
}
</script>

<style scoped>
.card {
  position: relative;
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: var(--transition-base);
}

/* Padding Variants */
.card-padding-none .card-body {
  padding: 0;
}

.card-padding-sm .card-body {
  padding: var(--space-4);
}

.card-padding-md .card-body {
  padding: var(--space-6);
}

.card-padding-lg .card-body {
  padding: var(--space-8);
}

/* Card Variants */
.card-glass {
  background: var(--glass-base);
  backdrop-filter: blur(var(--blur-lg));
  -webkit-backdrop-filter: blur(var(--blur-lg));
  border: 1px solid var(--color-border-subtle);
  box-shadow: var(--shadow-base);
}

.card-elevated {
  background: var(--color-background-elevated);
  box-shadow: var(--shadow-lg);
  border: 1px solid var(--color-border-subtle);
}

.card-outlined {
  background: var(--color-background-elevated);
  border: 2px solid var(--color-border-base);
}

.card-flat {
  background: var(--color-background-elevated);
  border: none;
}

/* Glow Background Effect */
.card-glow {
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  opacity: 0.1;
  pointer-events: none;
  transition: opacity var(--duration-moderate) var(--ease-in-out);
}

.card-glow-cyan {
  background: var(--gradient-radial-cyan);
}

.card-glow-purple {
  background: var(--gradient-radial-purple);
}

.card-glow-success {
  background: radial-gradient(circle at 50% 50%, rgba(0, 217, 168, 0.15) 0%, transparent 70%);
}

.card-glow-warning {
  background: radial-gradient(circle at 50% 50%, rgba(255, 193, 7, 0.15) 0%, transparent 70%);
}

.card-glow-error {
  background: radial-gradient(circle at 50% 50%, rgba(220, 53, 69, 0.15) 0%, transparent 70%);
}

.card-with-glow:hover .card-glow {
  opacity: 0.2;
}

/* Hoverable State */
.card-hoverable {
  cursor: default;
}

.card-hoverable:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-xl);
}

.card-glass.card-hoverable:hover {
  border-color: var(--color-border-base);
  background: var(--glass-strong);
}

/* Clickable State */
.card-clickable {
  cursor: pointer;
}

.card-clickable:active {
  transform: scale(0.98);
}

/* Header */
.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-6);
  padding-bottom: var(--space-4);
  border-bottom: 1px solid var(--color-border-subtle);
}

.card-title {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0;
}

.card-actions {
  display: flex;
  gap: var(--space-2);
}

/* Body */
.card-body {
  position: relative;
  z-index: 1;
}

/* Footer */
.card-footer {
  padding: var(--space-4) var(--space-6) var(--space-6);
  border-top: 1px solid var(--color-border-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

/* Animations */
.card {
  animation: fadeInScale var(--duration-moderate) var(--ease-out);
}

@keyframes fadeInScale {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
