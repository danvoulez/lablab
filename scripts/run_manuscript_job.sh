#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
RUNNER_DIR="${PROJECT_ROOT}/logline_discovery"
OUTPUT_DIR="${RUNNER_DIR}/manuscripts"

mkdir -p "${OUTPUT_DIR}"
TIMESTAMP=$(date -u +"%Y%m%dT%H%M%SZ")
OUTPUT_FILE="${OUTPUT_DIR}/manuscript_${TIMESTAMP}.md"

cd "${RUNNER_DIR}"

cargo run -p hiv_discovery_runner -- manuscript --output "${OUTPUT_FILE}"

echo "Manuscript bundle written to ${OUTPUT_FILE}"
echo "Structured bundle sidecar: ${OUTPUT_FILE%.md}.json"
