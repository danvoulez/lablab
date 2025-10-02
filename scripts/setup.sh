#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACE_DIR="${ROOT_DIR}/logline_discovery"
ENV_FILE="${WORKSPACE_DIR}/.env"
LEDGER_DIR="${WORKSPACE_DIR}/ledger/spans"
LEDGER_FILE="${LEDGER_DIR}/discovery.ndjson"
MIGRATION_FILE="${WORKSPACE_DIR}/db/migrations/0001_init.sql"

log() {
  printf '[setup] %s\n' "$*"
}

ensure_env_file() {
  if [[ -f "${ENV_FILE}" ]]; then
    log ".env already present at ${ENV_FILE}"
    return
  fi

  cat <<'EOC' > "${ENV_FILE}"
# LogLine Discovery Lab defaults
DATABASE_URL=postgres://localhost/logline_discovery
LEDGER_PATH=ledger/spans/discovery.ndjson
EOC
  log "Wrote default .env"
}

ensure_ledger_file() {
  mkdir -p "${LEDGER_DIR}"
  if [[ -f "${LEDGER_FILE}" ]]; then
    log "Ledger already exists at ${LEDGER_FILE}"
    return
  fi
  printf '' > "${LEDGER_FILE}"
  log "Created empty ledger ${LEDGER_FILE}"
}

run_migration() {
  if ! command -v psql >/dev/null 2>&1; then
    log "psql not found; skipping migration. Run manually when Postgres is available."
    return
  fi

  if [[ ! -f "${MIGRATION_FILE}" ]]; then
    log "Migration file missing: ${MIGRATION_FILE}"
    return
  fi

  source "${ENV_FILE}"
  if [[ -z "${DATABASE_URL:-}" ]]; then
    log "DATABASE_URL not set in .env; skipping migration"
    return
  fi

  log "Applying migration to ${DATABASE_URL}"
  psql "${DATABASE_URL}" < "${MIGRATION_FILE}"
}

main() {
  log "Bootstrapping LogLine Discovery Lab"
  ensure_env_file
  ensure_ledger_file
  run_migration
  log "Setup complete"
}

main "$@"
