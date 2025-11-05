// lib/sanitize.ts
import { MAX_INPUT_LENGTH } from './constants'

/**
 * Simple HTML sanitization for markdown-like formatting
 * Allows only bold (**text**) conversion and escapes all other HTML
 */
export function sanitizeMarkdown(text: string): string {
  // First, escape all HTML special characters
  const escaped = text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
  
  // Then convert **bold** markdown to <strong> tags
  return escaped.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
}

/**
 * Validate and sanitize user input
 * @throws Error if input exceeds maximum length
 */
export function sanitizeUserInput(input: string): string {
  // Trim whitespace
  const trimmed = input.trim()
  
  // Limit length to prevent abuse
  if (trimmed.length > MAX_INPUT_LENGTH) {
    throw new Error(`Input exceeds maximum length of ${MAX_INPUT_LENGTH} characters`)
  }
  
  return trimmed
}
