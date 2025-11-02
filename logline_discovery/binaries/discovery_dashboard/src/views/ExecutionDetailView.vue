<template>
  <div class="execution-detail-view">
    <BaseCard variant="glass" padding="lg">
      <template #header>
        <div class="detail-header">
          <h2>Execution Detail</h2>
          <BaseBadge variant="info" :glow="true">
            ID: {{ executionId }}
          </BaseBadge>
        </div>
      </template>

      <div class="detail-content">
        <p class="placeholder-message">
          Detailed execution view coming soon...
        </p>
        <p class="placeholder-description">
          This view will show comprehensive analysis, metrics, and visualizations for execution {{ executionId }}.
        </p>
      </div>

      <template #footer>
        <BaseButton variant="secondary" @click="goBack">
          ← Back to Dashboard
        </BaseButton>
      </template>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import BaseCard from '@/components/BaseCard.vue'
import BaseBadge from '@/components/BaseBadge.vue'
import BaseButton from '@/components/BaseButton.vue'

const route = useRoute()
const router = useRouter()

const executionId = computed(() => route.params.id)

const goBack = () => {
  router.push('/')
}
</script>

<style scoped>
.execution-detail-view {
  width: 100%;
  animation: fadeIn var(--duration-moderate) var(--ease-out);
}

.detail-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.detail-header h2 {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  background: linear-gradient(135deg, #00d4ff 0%, #8000ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin: 0;
}

.detail-content {
  padding: var(--space-12) 0;
  text-align: center;
}

.placeholder-message {
  font-size: var(--font-size-xl);
  color: var(--color-text-primary);
  font-weight: var(--font-weight-semibold);
  margin: 0 0 var(--space-4) 0;
}

.placeholder-description {
  font-size: var(--font-size-base);
  color: var(--color-text-secondary);
  margin: 0;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>
