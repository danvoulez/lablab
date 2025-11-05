// lib/apiClient.ts
/**
 * API Client for LogLine Discovery Lab Backend
 * 
 * Centralizes all HTTP calls to the Rust backend Director API
 */

import type { SessionData, ManifestoData } from './types'

// Get API base URL from environment or default to localhost
const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:3001'

/**
 * Backend response types matching Rust structs
 */
export interface SimulateProteinRequest {
  sequence: string
  mutation?: string
}

export interface SimulateProteinResponse {
  session_id: string
  pdb: string
  plddt: number[]
  confidence: {
    overall: number
  }
  structure_hash: string
  audit_trail: string[]
  manifest: {
    session_id: string
    timestamp: string
    participants: string[]
    scientific_question: string
    methodology: string[]
    findings: Array<{
      title: string
      description: string
      evidence?: string
    }>
    conclusions: string[]
    digital_signature: string
    audit_trail: string[]
  }
  started_at: string
}

export interface ChatRequest {
  message: string
  actor?: string
  include_context?: boolean
}

export interface ChatResponse {
  session_id: string
  message: string
  classification: {
    flow: string
    priority: string
    confidence: number
  }
  timestamp: string
}

/**
 * Error class for API errors
 */
export class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public details?: unknown
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

/**
 * Convert backend response to frontend SessionData format
 */
function convertToSessionData(response: SimulateProteinResponse): SessionData {
  return {
    sessionId: response.session_id,
    startedAt: response.started_at,
    structureHash: response.structure_hash,
    confidence: { overall: response.confidence.overall },
    plddt: response.plddt,
    auditTrail: response.audit_trail,
    models: [
      {
        name: 'Model-1',
        format: 'pdb' as const,
        content: response.pdb,
      },
    ],
    manifesto: {
      sessionId: response.manifest.session_id,
      timestamp: response.manifest.timestamp,
      participants: response.manifest.participants,
      scientificQuestion: response.manifest.scientific_question,
      methodology: response.manifest.methodology,
      findings: response.manifest.findings.map((f) => ({
        title: f.title,
        description: f.description,
        evidence: f.evidence,
      })),
      conclusions: response.manifest.conclusions,
      digitalSignature: response.manifest.digital_signature,
      auditTrail: response.manifest.audit_trail,
    },
  }
}

/**
 * Simulate a protein structure
 */
export async function simulateProtein(
  sequence: string,
  mutation?: string
): Promise<SessionData> {
  try {
    const response = await fetch(`${API_BASE_URL}/api/simulate_protein`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        sequence,
        mutation,
      } as SimulateProteinRequest),
    })

    if (!response.ok) {
      const errorText = await response.text()
      throw new ApiError(
        `Simulation failed: ${response.statusText}`,
        response.status,
        errorText
      )
    }

    const data: SimulateProteinResponse = await response.json()
    return convertToSessionData(data)
  } catch (error) {
    if (error instanceof ApiError) {
      throw error
    }
    throw new ApiError(
      `Network error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      undefined,
      error
    )
  }
}

/**
 * Send a chat message to the agent
 */
export async function sendChatMessage(
  message: string,
  includeContext: boolean = true
): Promise<ChatResponse> {
  try {
    const response = await fetch(`${API_BASE_URL}/api/chat`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        message,
        include_context: includeContext,
      } as ChatRequest),
    })

    if (!response.ok) {
      const errorText = await response.text()
      throw new ApiError(
        `Chat request failed: ${response.statusText}`,
        response.status,
        errorText
      )
    }

    const data: ChatResponse = await response.json()
    return data
  } catch (error) {
    if (error instanceof ApiError) {
      throw error
    }
    throw new ApiError(
      `Network error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      undefined,
      error
    )
  }
}

/**
 * Check backend health
 */
export async function checkHealth(): Promise<{
  status: string
  version: string
  uptime_seconds: number
}> {
  try {
    const response = await fetch(`${API_BASE_URL}/health`)

    if (!response.ok) {
      throw new ApiError(`Health check failed: ${response.statusText}`, response.status)
    }

    return await response.json()
  } catch (error) {
    if (error instanceof ApiError) {
      throw error
    }
    throw new ApiError(
      `Network error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      undefined,
      error
    )
  }
}
