import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { twinService, type TwinObservation, type TwinDivergence } from '@/services/twinService'

export const useTwinStore = defineStore('twin', () => {
  const observations = ref<TwinObservation[]>([])
  const divergences = ref<TwinDivergence[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const hasObservations = computed(() => observations.value.length > 0)
  const hasDivergences = computed(() => divergences.value.length > 0)

  const criticalDivergences = computed(() =>
    divergences.value.filter(d => d.severity.toLowerCase() === 'critical')
  )

  const warningDivergences = computed(() =>
    divergences.value.filter(d => d.severity.toLowerCase() === 'warning')
  )

  async function loadTwinData() {
    loading.value = true
    error.value = null

    try {
      const [obsData, divData] = await Promise.all([
        twinService.getTwinObservations(),
        twinService.getTwinDivergences()
      ])

      observations.value = obsData
      divergences.value = divData
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load twin data'
      console.error('Error loading twin data:', err)
    } finally {
      loading.value = false
    }
  }

  async function refreshData() {
    await loadTwinData()
  }

  return {
    observations,
    divergences,
    loading,
    error,
    hasObservations,
    hasDivergences,
    criticalDivergences,
    warningDivergences,
    loadTwinData,
    refreshData
  }
})
