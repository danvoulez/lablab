# Span Mapping Reference

This note captures how the HIV Discovery Runner classifies spans and writes them into the Postgres audit tables after the raw mirror is populated.

## Classification
- `subject_init` → parsed into `SubjectMetadata` (`subject_type`, `subject_identifier`, optional `intent`).
- `protocol_config` → parsed into `ProtocolMetadata` (`recipe_name`, `parameters`, optional `checksum`, optional `subject_span`).
- `execution_run` → parsed into `ExecutionMetadata` (status, start/finish timestamps, optional `subject_span`, optional `protocol_span`).
- All other flows currently bypass structured mapping and remain in `discovery.raw_spans` for future handlers.

## Mapping Rules
1. **Subjects**
   - Required fields come from the metadata block; a missing metadata entry keeps the span raw-only.
   - Stored in `discovery.runs_subjects` with `metadata` JSONB mirroring the full metadata payload.
2. **Protocols**
   - Resolves the subject reference via `metadata.subject_span` or the span’s parent ID.
   - If the subject span does not exist in `runs_subjects`, the protocol insert is skipped with a warning.
   - `checksum` uses the provided value or an MD5 hash of the `parameters` JSON payload.
   - Inserts into `discovery.runs_protocols` with a JSONB copy of metadata plus parameters.
3. **Executions**
4. **Acquisitions**
   - Require an execution reference (`metadata.execution_span`, parent span, or related span fallback).
   - Persisted into `discovery.runs_acquisitions` with source/type/reference metadata.
5. **Frames**
   - Capture per-frame payloads (`frame_index`, optional timestamp) stored in `discovery.runs_frames`.
6. **Metrics**
   - Metric name/value/unit plus optional metadata written to `discovery.runs_metrics`.
7. **Analyses**
   - Analysis summaries appended to `discovery.runs_analysis` as JSON.
8. **Reviews**
   - Reviewer verdicts recorded in `discovery.runs_reviews` with optional notes.
9. **Manuscripts**
   - Title, format, storage path, and checksum archived in `discovery.runs_manuscripts`.
10. **Artifacts**
    - Artifact manifests linked to executions in `discovery.runs_artifacts`.
   - Requires a subject reference (`metadata.subject_span` or span parent). Missing references abort the insert.
   - Optional protocol reference comes from `metadata.protocol_span` or the first related span ID.
   - Defaults `status` to `pending` when unspecified; `started_at` falls back to the span’s start time.
   - Inserts into `discovery.runs_executions`, linking to the resolved subject and optional protocol.

## Error Handling
- All structured inserts use `ON CONFLICT (span_id) DO NOTHING` to maintain append-only semantics.
- Missing dependencies or unresolved references emit `tracing::warn!` logs and leave the span in the raw mirror for later reconciliation.

## Next Steps
- Extend classification for `acquisition`, `analysis`, `manuscript`, etc., mapping them into their respective domain tables.
- Add automated tests covering the mapping paths once fixture spans are available.
- Consider deterministic checksum strategy shared with Warp contracts for cross-system verification.

See also: [Ingestion Walkthrough](ingestion_walkthrough.md).
