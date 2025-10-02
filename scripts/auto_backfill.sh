#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 /path/to/spans.ndjson" >&2
  exit 1
fi

LEDGER_SOURCE="$1"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKSPACE_DIR="${ROOT_DIR}/logline_discovery"

if [[ ! -f "${LEDGER_SOURCE}" ]]; then
  echo "Source file not found: ${LEDGER_SOURCE}" >&2
  exit 1
fi

echo "Replaying ${LEDGER_SOURCE} into local ledger"
(
  cd "${WORKSPACE_DIR}" && \
  cargo run -p hiv_discovery_runner -- sync-ledger --path "${LEDGER_SOURCE}"
)
