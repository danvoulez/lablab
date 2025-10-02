import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 10000
})

// Types for our data structures
export interface TwinObservation {
  span_id: string
  side: string
  cycle_id: string
  metrics: Record<string, number>
  recorded_at: string
  execution_id?: string
}

export interface TwinDivergence {
  span_id: string
  cycle_id: string
  metric: string
  severity: string
  absolute_delta: number
  percent_delta: number
  detected_at: string
  physical_span?: string
  digital_span?: string
  execution_id?: string
}

class TwinService {
  async getTwinObservations(): Promise<TwinObservation[]> {
    try {
      const response = await api.get<TwinObservation[]>('/twin-observations')
      return response.data
    } catch (error) {
      console.error('Failed to fetch twin observations:', error)
      return []
    }
  }

  async getTwinDivergences(): Promise<TwinDivergence[]> {
    try {
      const response = await api.get<TwinDivergence[]>('/twin-divergences')
      return response.data
    } catch (error) {
      console.error('Failed to fetch twin divergences:', error)
      return []
    }
  }

  async getExecutionTwinData(executionId: string): Promise<{
    observations: TwinObservation[]
    divergences: TwinDivergence[]
  }> {
    try {
      const [observations, divergences] = await Promise.all([
        api.get<TwinObservation[]>(`/executions/${executionId}/twin-observations`),
        api.get<TwinDivergence[]>(`/executions/${executionId}/twin-divergences`)
      ])

      return {
        observations: observations.data,
        divergences: divergences.data
      }
    } catch (error) {
      console.error(`Failed to fetch twin data for execution ${executionId}:`, error)
      return { observations: [], divergences: [] }
    }
  }
}

export const twinService = new TwinService()
