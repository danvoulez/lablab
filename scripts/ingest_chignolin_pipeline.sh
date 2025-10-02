#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
RUNNER_DIR="${PROJECT_ROOT}/logline_discovery"
SAMPLES_DIR="${RUNNER_DIR}/samples/spans"

if [[ ! -d "${SAMPLES_DIR}" ]]; then
  echo "Samples directory not found: ${SAMPLES_DIR}" >&2
  exit 1
fi

cd "${RUNNER_DIR}"

for file in \
  "${SAMPLES_DIR}/subject_chignolin.json" \
  "${SAMPLES_DIR}/protocol_chignolin.json" \
  "${SAMPLES_DIR}/execution_chignolin.json" \
  "${SAMPLES_DIR}/metric_chignolin.json" \
  "${SAMPLES_DIR}/analysis_chignolin.json" \
  "${SAMPLES_DIR}/manuscript_chignolin.json" \
  "${SAMPLES_DIR}/artifact_bundle.json"; do
  echo "Ingesting ${file}"
  cargo run -p hiv_discovery_runner -- ingest "${file}"
  echo
done
