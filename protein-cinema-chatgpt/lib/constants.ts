// lib/constants.ts

/**
 * Application-wide constants
 */

// Animation and Timing
export const CHAT_SHOW_DELAY_MS = 2000
export const THINKING_ANIMATION_INTERVAL_MS = 500
export const SESSION_START_DELAY_MS = 1500
export const ASSISTANT_RESPONSE_DELAY_MS = 1000
export const SIMULATION_DELAY_MS = 1500

// Canvas and Visualization
export const MAX_STARS = 250
export const STARS_PER_WIDTH_UNIT = 6
export const MIN_STAR_SIZE = 0.3
export const MAX_STAR_SIZE = 1.8
export const PARALLAX_MIN = 0.5
export const PARALLAX_MAX = 1.0

// User Input Constraints
export const MAX_INPUT_LENGTH = 5000

// Touch Gesture Thresholds
export const MIN_SWIPE_DISTANCE = 60

// pLDDT (Protein Confidence) Configuration
export const DEFAULT_PLDDT_LENGTH = 120
export const PLDDT_BASE = 78
export const PLDDT_AMPLITUDE = 15
export const PLDDT_FREQUENCY = 13
export const PLDDT_NOISE_RANGE = 8
export const PLDDT_MIN = 40
export const PLDDT_MAX = 98

// Protein Theater Controls
export const ZOOM_STEP = 1.15
export const ZOOM_WHEEL_STEP = 1.1

// Color Thresholds for pLDDT
export const PLDDT_BINS = [0, 50, 70, 90, 100] as const
export const PLDDT_LABELS = ['<50', '50–69', '70–89', '90–100'] as const
