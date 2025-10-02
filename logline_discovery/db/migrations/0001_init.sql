-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Dedicated schema for discovery lab artifacts
CREATE SCHEMA IF NOT EXISTS discovery;

-- Generic helper to enforce append-only semantics
CREATE OR REPLACE FUNCTION discovery.prevent_mutation()
RETURNS trigger AS $$
BEGIN
    RAISE EXCEPTION 'Table % is append-only', TG_TABLE_NAME;
END;
$$ LANGUAGE plpgsql;

-- Subjects (protein, patient sample, etc.)
CREATE TABLE IF NOT EXISTS discovery.runs_subjects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    subject_type TEXT NOT NULL,
    subject_identifier TEXT NOT NULL,
    intent TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_subjects_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_subjects
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Protocol configurations
CREATE TABLE IF NOT EXISTS discovery.runs_protocols (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    subject_id UUID NOT NULL REFERENCES discovery.runs_subjects(id),
    recipe_name TEXT NOT NULL,
    parameters JSONB NOT NULL,
    checksum TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_protocols_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_protocols
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Execution runs linking subject + protocol
CREATE TABLE IF NOT EXISTS discovery.runs_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    subject_id UUID NOT NULL REFERENCES discovery.runs_subjects(id),
    protocol_id UUID REFERENCES discovery.runs_protocols(id),
    status TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_executions_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_executions
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Input acquisitions (structures, sequences, etc.)
CREATE TABLE IF NOT EXISTS discovery.runs_acquisitions (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    source TEXT NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_reference TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_acquisitions_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_acquisitions
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Simulation frames (raw trajectories)
CREATE TABLE IF NOT EXISTS discovery.runs_frames (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    frame_index INTEGER NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_frames_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_frames
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Metrics (latency, energy, etc.)
CREATE TABLE IF NOT EXISTS discovery.runs_metrics (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    metric_name TEXT NOT NULL,
    value DOUBLE PRECISION,
    unit TEXT,
    recorded_at TIMESTAMPTZ NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_metrics_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_metrics
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Analysis summaries (folding, similarity, causal)
CREATE TABLE IF NOT EXISTS discovery.runs_analysis (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    analysis_type TEXT NOT NULL,
    summary JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_analysis_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_analysis
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Review checkpoints
CREATE TABLE IF NOT EXISTS discovery.runs_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    reviewer TEXT NOT NULL,
    verdict TEXT NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_reviews_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_reviews
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Manuscript & artifact records
CREATE TABLE IF NOT EXISTS discovery.runs_manuscripts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    title TEXT NOT NULL,
    format TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    checksum TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_manuscripts_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_manuscripts
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Artifact manifest for bundles (NDJSON, SQL dump, contracts)
CREATE TABLE IF NOT EXISTS discovery.runs_artifacts (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES discovery.runs_executions(id),
    artifact_kind TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    checksum TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER runs_artifacts_no_update
BEFORE UPDATE OR DELETE ON discovery.runs_artifacts
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();


-- Twin observations linking physical/digital metrics
CREATE TABLE IF NOT EXISTS discovery.twin_observations (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID REFERENCES discovery.runs_executions(id),
    cycle_id TEXT NOT NULL,
    side TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    metrics JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER twin_observations_no_update
BEFORE UPDATE OR DELETE ON discovery.twin_observations
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Twin divergences detected during comparisons
CREATE TABLE IF NOT EXISTS discovery.twin_divergences (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    execution_id UUID REFERENCES discovery.runs_executions(id),
    cycle_id TEXT NOT NULL,
    metric TEXT NOT NULL,
    severity TEXT NOT NULL,
    absolute_delta DOUBLE PRECISION NOT NULL,
    percent_delta DOUBLE PRECISION NOT NULL,
    detected_at TIMESTAMPTZ NOT NULL,
    physical_span TEXT,
    digital_span TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER twin_divergences_no_update
BEFORE UPDATE OR DELETE ON discovery.twin_divergences
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();

-- Raw span mirror for generic ingestion/audit
CREATE TABLE IF NOT EXISTS discovery.raw_spans (
    id BIGSERIAL PRIMARY KEY,
    span_id TEXT NOT NULL UNIQUE,
    flow TEXT NOT NULL,
    workflow TEXT NOT NULL,
    payload JSONB NOT NULL,
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER raw_spans_no_update
BEFORE UPDATE OR DELETE ON discovery.raw_spans
FOR EACH ROW EXECUTE FUNCTION discovery.prevent_mutation();
