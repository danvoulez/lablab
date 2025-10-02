<template>
  <div class="home-view">
    <div class="dashboard-header">
      <h2>Twin Bridge Monitor</h2>
      <div class="status-indicators">
        <div class="status-indicator" :class="{ active: hasObservations }">
          <span class="status-dot"></span>
          <span class="status-text">Observations</span>
        </div>
        <div class="status-indicator" :class="{ active: hasDivergences, warning: hasDivergences }">
          <span class="status-dot"></span>
          <span class="status-text">Divergences</span>
        </div>
      </div>
    </div>

    <div class="twin-sections">
      <!-- Twin Observations Section -->
      <div class="twin-section">
        <h3>Twin Observations</h3>
        <div class="observations-grid">
          <div
            v-for="observation in observations"
            :key="observation.span_id"
            class="observation-card"
            :class="`observation-${observation.side}`"
          >
            <div class="card-header">
              <h4>{{ observation.side.toUpperCase() }} Twin</h4>
              <span class="cycle-id">{{ observation.cycle_id }}</span>
            </div>
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
            <div class="timestamp">
              {{ formatTimestamp(observation.recorded_at) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Twin Divergences Section -->
      <div class="twin-section">
        <h3>Twin Divergences</h3>
        <div class="divergences-list">
          <div
            v-for="divergence in divergences"
            :key="divergence.span_id"
            class="divergence-card"
            :class="`severity-${divergence.severity.toLowerCase()}`"
          >
            <div class="divergence-header">
              <h4>{{ divergence.metric }} Divergence</h4>
              <span class="severity-badge" :class="divergence.severity.toLowerCase()">
                {{ divergence.severity }}
              </span>
            </div>
            <div class="divergence-metrics">
              <div class="metric-row">
                <span>Absolute Δ:</span>
                <span class="delta-value">{{ formatNumber(divergence.absolute_delta) }}</span>
              </div>
              <div class="metric-row">
                <span>Percent Δ:</span>
                <span class="delta-value">{{ formatPercent(divergence.percent_delta) }}</span>
              </div>
            </div>
            <div class="divergence-footer">
              <span class="cycle-id">Cycle: {{ divergence.cycle_id }}</span>
              <span class="timestamp">{{ formatTimestamp(divergence.detected_at) }}</span>
            </div>
          </div>
          <div v-if="divergences.length === 0" class="no-data">
            No divergences detected
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { twinService, type TwinObservation, type TwinDivergence } from '@/services/twinService'

const observations = ref<TwinObservation[]>([])
const divergences = ref<TwinDivergence[]>([])

const hasObservations = computed(() => observations.value.length > 0)
const hasDivergences = computed(() => divergences.value.length > 0)

const loadTwinData = async () => {
  try {
    const [obsData, divData] = await Promise.all([
      twinService.getTwinObservations(),
      twinService.getTwinDivergences()
    ])

    observations.value = obsData
    divergences.value = divData
  } catch (error) {
    console.error('Failed to load twin data:', error)
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

onMounted(() => {
  loadTwinData()

  // Set up polling for real-time updates (every 30 seconds)
  const interval = setInterval(loadTwinData, 30000)

  // Cleanup interval on unmount
  return () => clearInterval(interval)
})
</script>

<style scoped>
.home-view {
  max-width: 1400px;
  margin: 0 auto;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid rgba(255, 255, 255, 0.1);
}

.dashboard-header h2 {
  color: #00d4ff;
  margin: 0;
}

.status-indicators {
  display: flex;
  gap: 1rem;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.status-indicator.active {
  background: rgba(0, 212, 255, 0.2);
  border: 1px solid #00d4ff;
}

.status-indicator.warning {
  background: rgba(255, 193, 7, 0.2);
  border: 1px solid #ffc107;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #666;
  transition: all 0.3s ease;
}

.status-indicator.active .status-dot {
  background: #00d4ff;
  box-shadow: 0 0 10px rgba(0, 212, 255, 0.5);
}

.status-indicator.warning .status-dot {
  background: #ffc107;
  box-shadow: 0 0 10px rgba(255, 193, 7, 0.5);
}

.twin-sections {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.twin-section h3 {
  margin-bottom: 1rem;
  color: #00d4ff;
  border-bottom: 1px solid rgba(0, 212, 255, 0.3);
  padding-bottom: 0.5rem;
}

.observations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 1rem;
}

.observation-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.observation-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
}

.observation-digital {
  border-left: 4px solid #00d4ff;
}

.observation-physical {
  border-left: 4px solid #ffc107;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.card-header h4 {
  margin: 0;
  color: #f6f7fb;
}

.cycle-id {
  font-size: 0.8rem;
  opacity: 0.7;
  font-family: monospace;
}

.metrics-display {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.metric-item {
  display: flex;
  justify-content: space-between;
  padding: 0.25rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.metric-name {
  opacity: 0.8;
}

.metric-value {
  font-weight: bold;
  font-family: monospace;
}

.timestamp {
  font-size: 0.8rem;
  opacity: 0.6;
  text-align: right;
}

.divergences-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.divergence-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.divergence-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
}

.severity-critical {
  border-left: 4px solid #dc3545;
}

.severity-warning {
  border-left: 4px solid #ffc107;
}

.severity-info {
  border-left: 4px solid #17a2b8;
}

.divergence-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.divergence-header h4 {
  margin: 0;
  color: #f6f7fb;
}

.severity-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: bold;
  text-transform: uppercase;
}

.severity-badge.critical {
  background: rgba(220, 53, 69, 0.2);
  color: #ff6b7a;
}

.severity-badge.warning {
  background: rgba(255, 193, 7, 0.2);
  color: #ffd93d;
}

.severity-badge.info {
  background: rgba(23, 162, 184, 0.2);
  color: #5bc0de;
}

.divergence-metrics {
  margin-bottom: 1rem;
}

.metric-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.delta-value {
  font-weight: bold;
  font-family: monospace;
  color: #ffc107;
}

.divergence-footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.8rem;
  opacity: 0.7;
}

.no-data {
  text-align: center;
  padding: 2rem;
  color: rgba(255, 255, 255, 0.5);
  font-style: italic;
}

@media (max-width: 1200px) {
  .twin-sections {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .observations-grid {
    grid-template-columns: 1fr;
  }

  .dashboard-header {
    flex-direction: column;
    gap: 1rem;
    text-align: center;
  }
}
</style>
