// lib/manifest.ts
import { sha256Hash } from './crypto-utils'
import { 
  DEFAULT_PLDDT_LENGTH, 
  PLDDT_BASE, 
  PLDDT_AMPLITUDE, 
  PLDDT_FREQUENCY, 
  PLDDT_NOISE_RANGE, 
  PLDDT_MIN, 
  PLDDT_MAX 
} from './constants'

export async function sha256Base64(payload: string): Promise<string> {
  return sha256Hash(payload)
}

export async function signManifesto(payload: object): Promise<string> {
  // Assinatura simbólica "DV25-Seal-like": HASH(payload) + epoch
  const json = JSON.stringify(payload)
  const hash = await sha256Base64(json)
  return `SIGNED|${Date.now()}|${hash}|PT1`
}

export function niceId(prefix = 'PT'): string {
  // Use timestamp + random for uniqueness
  return `${prefix}-${Math.random().toString(36).slice(2, 6)}-${Date.now().toString(36)}`
}

export function samplePLDDT(len = DEFAULT_PLDDT_LENGTH): number[] {
  // Gera curva pLDDT plausível 60–95 com ruído
  const out: number[] = []
  for (let i = 0; i < len; i++) {
    const base = PLDDT_BASE + PLDDT_AMPLITUDE * Math.sin(i / PLDDT_FREQUENCY)
    const noise = (Math.random() - 0.5) * PLDDT_NOISE_RANGE
    out.push(Math.max(PLDDT_MIN, Math.min(PLDDT_MAX, Math.round(base + noise))))
  }
  return out
}
