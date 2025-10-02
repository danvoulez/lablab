#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

SPAN_FILE="${PROJECT_ROOT}/logline_discovery/ledger/spans/folding/gp41_folding.json"

if [[ ! -f "${SPAN_FILE}" ]]; then
  echo "Span file not found: ${SPAN_FILE}" >&2
  exit 1
fi

cd "${PROJECT_ROOT}/logline_discovery"

cargo run -p hiv_discovery_runner -- ingest "${SPAN_FILE}" --flow protein_folding --workflow hiv_reservoir_mapping
