<template>
  <div class="home-view">
    <!-- Loading State -->
    <LoadingSpinner v-if="isLoading" text="Loading Twin Data..." size="lg" />

    <template v-else>
      <!-- Dashboard Header -->
      <div class="dashboard-header">
        <div class="header-title">
          <h2>Digital Twin Monitor</h2>
          <p class="header-subtitle">Real-time HIV drug discovery pipeline synchronization</p>
        </div>
        <div class="status-indicators">
          <BaseBadge
            :variant="hasObservations ? 'info' : 'default'"
            :dot="hasObservations"
            size="md"
            :glow="hasObservations"
          >
            {{ observations.length }} Observations
          </BaseBadge>
          <BaseBadge
            :variant="hasDivergences ? 'warning' : 'success'"
            :dot="hasDivergences"
            size="md"
            :glow="hasDivergences"
          >
            {{ divergences.length }} Divergences
          </BaseBadge>
          <div class="refresh-indicator">
            <span class="refresh-text">Last refresh: {{ lastRefresh }}</span>
          </div>
        </div>
      </div>

      <!-- Stats Overview -->
      <div class="stats-grid">
        <BaseCard variant="glass" :hoverable="true" :glow-effect="true" glow-color="cyan" padding="md">
          <div class="stat-card">
            <div class="stat-icon cyan">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                <path d="M2 17l10 5 10-5"/>
                <path d="M2 12l10 5 10-5"/>
              </svg>
            </div>
            <div class="stat-content">
              <p class="stat-label">Total Observations</p>
              <h3 class="stat-value">{{ observations.length }}</h3>
              <p class="stat-change positive">+{{ observations.length }} this cycle</p>
            </div>
          </div>
        </BaseCard>

        <BaseCard variant="glass" :hoverable="true" :glow-effect="true" glow-color="purple" padding="md">
          <div class="stat-card">
            <div class="stat-icon purple">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <path d="M12 6v6l4 2"/>
              </svg>
            </div>
            <div class="stat-content">
              <p class="stat-label">Sync Status</p>
              <h3 class="stat-value">{{ hasDivergences ? 'Diverged' : 'Synchronized' }}</h3>
              <p class="stat-change" :class="hasDivergences ? 'negative' : 'positive'">
                {{ hasDivergences ? 'Requires attention' : 'Operating normally' }}
              </p>
            </div>
          </div>
        </BaseCard>

        <BaseCard variant="glass" :hoverable="true" :glow-effect="true" glow-color="success" padding="md">
          <div class="stat-card">
            <div class="stat-icon success">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/>
              </svg>
            </div>
            <div class="stat-content">
              <p class="stat-label">Critical Alerts</p>
              <h3 class="stat-value">{{ criticalDivergences }}</h3>
              <p class="stat-change" :class="criticalDivergences > 0 ? 'negative' : 'neutral'">
                {{ criticalDivergences > 0 ? 'Immediate review needed' : 'All systems nominal' }}
              </p>
            </div>
          </div>
        </BaseCard>
      </div>

      <!-- Twin Sections -->
      <div class="twin-sections">
        <!-- Twin Observations Section -->
        <div class="twin-section">
          <div class="section-header">
            <h3>
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                <path d="M2 17l10 5 10-5"/>
                <path d="M2 12l10 5 10-5"/>
              </svg>
              Twin Observations
            </h3>
            <BaseBadge variant="default" size="sm">Live</BaseBadge>
          </div>

          <div class="observations-grid">
            <BaseCard
              v-for="observation in observations"
              :key="observation.span_id"
              variant="glass"
              :hoverable="true"
              :glow-effect="true"
              :glow-color="observation.side === 'digital' ? 'cyan' : 'warning'"
              padding="lg"
            >
              <template #header>
                <div class="observation-header">
                  <div class="twin-badge" :class="observation.side">
                    {{ observation.side.toUpperCase() }} TWIN
                  </div>
                  <BaseBadge variant="default" size="sm" outline>
                    {{ observation.cycle_id }}
                  </BaseBadge>
                </div>
              </template>

              <div class="metrics-display">
                <div
                  v-for="(value, metric) in observation.metrics"
                  :key="metric"
                  class="metric-item"
                >
                  <span class="metric-name">{{ formatMetricName(metric) }}</span>
                  <span class="metric-value">{{ formatMetricValue(value, metric) }}</span>
                </div>
              </div>

              <template #footer>
                <div class="observation-footer">
                  <span class="timestamp">{{ formatTimestamp(observation.recorded_at) }}</span>
                </div>
              </template>
            </BaseCard>
          </div>

          <div v-if="observations.length === 0" class="no-data">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            <p>No observations available</p>
          </div>
        </div>

        <!-- Twin Divergences Section -->
        <div class="twin-section">
          <div class="section-header">
            <h3>
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
                <line x1="12" y1="9" x2="12" y2="13"/>
                <line x1="12" y1="17" x2="12.01" y2="17"/>
              </svg>
              Twin Divergences
            </h3>
            <BaseBadge
              :variant="hasDivergences ? 'warning' : 'success'"
              size="sm"
              :glow="hasDivergences"
            >
              {{ hasDivergences ? 'Active' : 'None' }}
            </BaseBadge>
          </div>

          <div class="divergences-list">
            <BaseCard
              v-for="divergence in divergences"
              :key="divergence.span_id"
              variant="glass"
              :hoverable="true"
              :glow-effect="true"
              :glow-color="getSeverityColor(divergence.severity)"
              padding="lg"
            >
              <template #header>
                <div class="divergence-header">
                  <h4>{{ divergence.metric }} Divergence</h4>
                  <BaseBadge
                    :variant="getSeverityVariant(divergence.severity)"
                    size="md"
                    :glow="divergence.severity === 'CRITICAL'"
                  >
                    {{ divergence.severity }}
                  </BaseBadge>
                </div>
              </template>

              <div class="divergence-metrics">
                <div class="metric-row">
                  <span class="metric-label">Absolute Δ:</span>
                  <span class="delta-value">{{ formatNumber(divergence.absolute_delta) }}</span>
                </div>
                <div class="metric-row">
                  <span class="metric-label">Percent Δ:</span>
                  <span class="delta-value">{{ formatPercent(divergence.percent_delta) }}</span>
                </div>
              </div>

              <template #footer>
                <div class="divergence-footer">
                  <BaseBadge variant="default" size="sm" outline>
                    Cycle: {{ divergence.cycle_id }}
                  </BaseBadge>
                  <span class="timestamp">{{ formatTimestamp(divergence.detected_at) }}</span>
                </div>
              </template>
            </BaseCard>
          </div>

          <div v-if="divergences.length === 0" class="no-data success">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 11.08V12a10 10 0 11-5.93-9.14"/>
              <polyline points="22 4 12 14.01 9 11.01"/>
            </svg>
            <p>No divergences detected</p>
            <span class="no-data-subtitle">Twins are synchronized</span>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { twinService, type TwinObservation, type TwinDivergence } from '@/services/twinService'
import BaseCard from '@/components/BaseCard.vue'
import BaseBadge from '@/components/BaseBadge.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'

const observations = ref<TwinObservation[]>([])
const divergences = ref<TwinDivergence[]>([])
const isLoading = ref(true)
const lastRefresh = ref('')

const hasObservations = computed(() => observations.value.length > 0)
const hasDivergences = computed(() => divergences.value.length > 0)
const criticalDivergences = computed(() =>
  divergences.value.filter(d => d.severity === 'CRITICAL').length
)

const loadTwinData = async () => {
  try {
    const [obsData, divData] = await Promise.all([
      twinService.getTwinObservations(),
      twinService.getTwinDivergences()
    ])

    observations.value = obsData
    divergences.value = divData
    lastRefresh.value = new Date().toLocaleTimeString()
    isLoading.value = false
  } catch (error) {
    console.error('Failed to load twin data:', error)
    isLoading.value = false
  }
}

const getSeverityVariant = (severity: string): 'error' | 'warning' | 'info' => {
  switch (severity.toUpperCase()) {
    case 'CRITICAL':
      return 'error'
    case 'WARNING':
      return 'warning'
    default:
      return 'info'
  }
}

const getSeverityColor = (severity: string): 'error' | 'warning' | 'cyan' => {
  switch (severity.toUpperCase()) {
    case 'CRITICAL':
      return 'error'
    case 'WARNING':
      return 'warning'
    default:
      return 'cyan'
  }
}

const formatMetricName = (name: string): string => {
  return name.split('_')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const formatMetricValue = (value: number, metric: string): string => {
  if (metric.includes('energy')) {
    return `${value.toFixed(1)} kcal/mol`
  } else if (metric.includes('rmsd')) {
    return `${value.toFixed(2)} Å`
  } else if (metric.includes('gyration') || metric.includes('surface')) {
    return `${value.toFixed(1)} Å²`
  } else if (metric.includes('temperature')) {
    return `${value.toFixed(1)} K`
  } else if (metric.includes('pressure')) {
    return `${value.toFixed(3)} bar`
  }
  return value.toFixed(3)
}

const formatNumber = (num: number): string => {
  return num.toFixed(3)
}

const formatPercent = (num: number): string => {
  return `${(num * 100).toFixed(1)}%`
}

const formatTimestamp = (timestamp: string): string => {
  return new Date(timestamp).toLocaleString()
}

let pollInterval: NodeJS.Timeout | null = null

onMounted(() => {
  loadTwinData()
  // Set up polling for real-time updates (every 30 seconds)
  pollInterval = setInterval(loadTwinData, 30000)
})

onUnmounted(() => {
  if (pollInterval) {
    clearInterval(pollInterval)
  }
})
</script>

<style scoped>
.home-view {
  width: 100%;
  animation: fadeIn var(--duration-moderate) var(--ease-out);
}

/* ============================================
   DASHBOARD HEADER
   ============================================ */
.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border-subtle);
  animation: slideInRight var(--duration-moderate) var(--ease-out);
}

.header-title h2 {
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  background: linear-gradient(135deg, #00d4ff 0%, #8000ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin: 0 0 var(--space-2) 0;
}

.header-subtitle {
  color: var(--color-text-secondary);
  font-size: var(--font-size-base);
  margin: 0;
}

.status-indicators {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
}

.refresh-indicator {
  padding: var(--space-2) var(--space-4);
  background: var(--glass-subtle);
  border-radius: var(--radius-full);
  border: 1px solid var(--color-border-subtle);
}

.refresh-text {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-family-mono);
}

/* ============================================
   STATS GRID
   ============================================ */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--space-6);
  margin-bottom: var(--space-8);
}

.stat-card {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.stat-icon {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-lg);
  transition: var(--transition-transform);
}

.stat-icon.cyan {
  background: var(--gradient-cyan-glow);
  box-shadow: var(--shadow-glow-cyan);
}

.stat-icon.purple {
  background: var(--gradient-purple-glow);
  box-shadow: var(--shadow-glow-purple);
}

.stat-icon.success {
  background: var(--gradient-success);
  box-shadow: var(--shadow-glow-success);
}

.stat-icon svg {
  color: var(--color-text-primary);
}

.stat-content {
  flex: 1;
}

.stat-label {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin: 0 0 var(--space-1) 0;
  font-weight: var(--font-weight-medium);
}

.stat-value {
  font-size: var(--font-size-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0 0 var(--space-1) 0;
  font-family: var(--font-family-mono);
}

.stat-change {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-medium);
  margin: 0;
}

.stat-change.positive {
  color: var(--color-success-light);
}

.stat-change.negative {
  color: var(--color-error-light);
}

.stat-change.neutral {
  color: var(--color-text-tertiary);
}

/* ============================================
   TWIN SECTIONS
   ============================================ */
.twin-sections {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  gap: var(--space-8);
}

.twin-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: var(--space-4);
  border-bottom: 1px solid var(--color-border-subtle);
}

.section-header h3 {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0;
}

.section-header svg {
  color: var(--color-cyan-500);
}

/* ============================================
   OBSERVATIONS
   ============================================ */
.observations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: var(--space-4);
}

.observation-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.twin-badge {
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-base);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-bold);
  letter-spacing: var(--letter-spacing-wide);
}

.twin-badge.digital {
  background: linear-gradient(135deg, rgba(0, 212, 255, 0.2) 0%, rgba(0, 212, 255, 0.1) 100%);
  color: var(--color-cyan-400);
  border: 1px solid var(--color-cyan-500);
}

.twin-badge.physical {
  background: linear-gradient(135deg, rgba(255, 193, 7, 0.2) 0%, rgba(255, 193, 7, 0.1) 100%);
  color: var(--color-warning-light);
  border: 1px solid var(--color-warning);
}

.metrics-display {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-3);
}

.metric-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: var(--space-3);
  background: var(--glass-subtle);
  border-radius: var(--radius-base);
  border: 1px solid var(--color-border-subtle);
  transition: var(--transition-base);
}

.metric-item:hover {
  background: var(--glass-base);
  border-color: var(--color-border-base);
}

.metric-name {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: var(--letter-spacing-wide);
  font-weight: var(--font-weight-medium);
}

.metric-value {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  font-family: var(--font-family-mono);
}

.observation-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  width: 100%;
}

.timestamp {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-family-mono);
}

/* ============================================
   DIVERGENCES
   ============================================ */
.divergences-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.divergence-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.divergence-header h4 {
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0;
}

.divergence-metrics {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.metric-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-3);
  background: var(--glass-subtle);
  border-radius: var(--radius-base);
  border: 1px solid var(--color-border-subtle);
}

.metric-label {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  font-weight: var(--font-weight-medium);
}

.delta-value {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-bold);
  color: var(--color-warning-light);
  font-family: var(--font-family-mono);
}

.divergence-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

/* ============================================
   NO DATA STATE
   ============================================ */
.no-data {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-16);
  text-align: center;
  background: var(--glass-subtle);
  border-radius: var(--radius-xl);
  border: 1px dashed var(--color-border-base);
}

.no-data svg {
  color: var(--color-text-tertiary);
  margin-bottom: var(--space-4);
  opacity: 0.5;
}

.no-data p {
  font-size: var(--font-size-lg);
  color: var(--color-text-secondary);
  margin: 0;
  font-weight: var(--font-weight-medium);
}

.no-data.success svg {
  color: var(--color-success);
  opacity: 0.8;
}

.no-data.success p {
  color: var(--color-success-light);
}

.no-data-subtitle {
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
  margin-top: var(--space-2);
}

/* ============================================
   ANIMATIONS
   ============================================ */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* ============================================
   RESPONSIVE DESIGN
   ============================================ */
@media (max-width: 1200px) {
  .twin-sections {
    grid-template-columns: 1fr;
  }

  .stats-grid {
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  }
}

@media (max-width: 768px) {
  .dashboard-header {
    flex-direction: column;
    gap: var(--space-4);
    align-items: flex-start;
  }

  .status-indicators {
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
  }

  .observations-grid {
    grid-template-columns: 1fr;
  }

  .metrics-display {
    grid-template-columns: 1fr;
  }

  .stats-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 480px) {
  .header-title h2 {
    font-size: var(--font-size-2xl);
  }

  .stat-value {
    font-size: var(--font-size-xl);
  }
}
</style>
