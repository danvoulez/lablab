// lib/crypto-utils.ts
/**
 * Generate a random UUID with fallback for older browsers
 */
export function generateUUID(): string {
  // Use native crypto.randomUUID if available
  if (typeof crypto !== 'undefined' && crypto.randomUUID) {
    return crypto.randomUUID()
  }
  
  // Fallback to manual UUID v4 generation
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0
    const v = c === 'x' ? r : (r & 0x3 | 0x8)
    return v.toString(16)
  })
}

/**
 * SHA-256 hash with fallback for older browsers
 */
export async function sha256Hash(payload: string): Promise<string> {
  // Check if crypto.subtle is available
  if (typeof crypto !== 'undefined' && crypto.subtle && crypto.subtle.digest) {
    try {
      const enc = new TextEncoder().encode(payload)
      const buf = await crypto.subtle.digest('SHA-256', enc)
      const bytes = new Uint8Array(buf)
      let binary = ''
      for (let i = 0; i < bytes.byteLength; i++) {
        binary += String.fromCharCode(bytes[i])
      }
      return btoa(binary)
    } catch (error) {
      console.warn('crypto.subtle.digest failed, using fallback:', error)
      return fallbackHash(payload)
    }
  }
  
  // Fallback to simple hash for older browsers
  return fallbackHash(payload)
}

/**
 * Simple fallback hash function (NOT cryptographically secure)
 * Only used when Web Crypto API is not available
 * 
 * WARNING: This is a simple hash for data integrity checking only.
 * It should NOT be used for security-critical operations.
 * The hash includes a timestamp to ensure uniqueness but is still predictable.
 */
function fallbackHash(str: string): string {
  let hash = 0
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash // Convert to 32bit integer
  }
  // Convert to base64-like string with timestamp for uniqueness
  const timestamp = Date.now().toString(36)
  const hashStr = Math.abs(hash).toString(36)
  return btoa(`fallback-${hashStr}-${timestamp}`)
}
