#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACE_DIR="${ROOT_DIR}/logline_discovery"
ENV_FILE="${WORKSPACE_DIR}/.env"
LOG_DIR="${WORKSPACE_DIR}/tmp/logs"
WATCH_LOG="${LOG_DIR}/watch.log"
SERVICE_LOG="${LOG_DIR}/service.log"

log() {
  printf '[run_lab] %s\n' "$*"
}

ensure_env_loaded() {
  if [[ -f "${ENV_FILE}" ]]; then
    set -a
    # shellcheck disable=SC1090
    source "${ENV_FILE}"
    set +a
    log "Loaded ${ENV_FILE}"
  else
    log "Warning: ${ENV_FILE} missing. Run scripts/setup.sh first."
  fi
}

start_watch() {
  local span_path="${1:-${HOME}/Library/Mobile Documents/com~apple~CloudDocs/LogLine Fold/Backend/spans}"
  mkdir -p "${LOG_DIR}"
  log "Starting watcher for ${span_path}"
  (
    cd "${WORKSPACE_DIR}" && \
    cargo run -p hiv_discovery_runner -- watch --path "${span_path}" --recursive --interval 30
  ) >>"${WATCH_LOG}" 2>&1 &
  WATCH_PID=$!
  log "Watcher PID ${WATCH_PID} (logs: ${WATCH_LOG})"
}

start_service() {
  mkdir -p "${LOG_DIR}"
  log "Starting service API on ${SERVICE_ADDRESS:-127.0.0.1:4040}"
  (
    cd "${WORKSPACE_DIR}" && \
    cargo run -p hiv_discovery_runner -- serve --address "${SERVICE_ADDRESS:-127.0.0.1:4040}"
  ) >>"${SERVICE_LOG}" 2>&1 &
  SERVICE_PID=$!
  log "Service PID ${SERVICE_PID} (logs: ${SERVICE_LOG})"
}

start_manuscript_timer() {
  log "Triggering manuscript generation for the latest execution"
  (
    cd "${WORKSPACE_DIR}" && \
    cargo run -p hiv_discovery_runner -- manuscript --output manuscripts/latest.md
  )
}

cleanup() {
  log "Stopping services"
  if [[ -n "${WATCH_PID:-}" ]]; then
    kill "${WATCH_PID}" 2>/dev/null || true
  fi
  if [[ -n "${SERVICE_PID:-}" ]]; then
    kill "${SERVICE_PID}" 2>/dev/null || true
  fi
}

main() {
  ensure_env_loaded
  trap cleanup EXIT
  start_watch "$1"
  start_service
  log "Lab is running. Press Ctrl+C to stop."
  wait
}

main "$@"
