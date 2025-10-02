-- Adds the 'pending' job status for two-phase scheduling and updates defaults.
ALTER TYPE job_status ADD VALUE IF NOT EXISTS 'pending';

ALTER TABLE jobs ALTER COLUMN status SET DEFAULT 'pending';
