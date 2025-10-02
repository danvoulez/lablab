-- LogLine Discovery Lab Job Queue Schema
-- Migration: 001_initial_job_queue

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Job statuses enum
CREATE TYPE job_status AS ENUM ('queued', 'running', 'completed', 'failed', 'cancelled');

-- Job priorities enum
CREATE TYPE job_priority AS ENUM ('low', 'normal', 'high', 'critical');

-- Main jobs table
CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    status job_status NOT NULL DEFAULT 'queued',
    priority job_priority NOT NULL DEFAULT 'normal',
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    worker_id VARCHAR(255),
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,

    -- Indexes for efficient querying
    INDEX idx_jobs_status (status),
    INDEX idx_jobs_priority_status (priority, status),
    INDEX idx_jobs_created_at (created_at),
    INDEX idx_jobs_worker_id (worker_id)
);

-- Job executions table for audit trail
CREATE TABLE IF NOT EXISTS job_executions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    worker_id VARCHAR(255) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    success BOOLEAN,
    error_message TEXT,
    metrics JSONB DEFAULT '{}',

    -- Indexes
    INDEX idx_job_executions_job_id (job_id),
    INDEX idx_job_executions_worker_id (worker_id),
    INDEX idx_job_executions_started_at (started_at)
);

-- Notification channel for job queue events
-- PostgreSQL will use this for LISTEN/NOTIFY pattern
