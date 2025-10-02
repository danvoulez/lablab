use logline_common::{
    config::Config,
    job::{Job, JobStatus, JobExecution},
    Error, Result,
};
use chrono::{DateTime, Utc};
use clap::Parser;
use sqlx::{PgPool, PgListener, PgConnection};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::{timeout, sleep};
use tracing::{info, warn, error};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about = "LogLine Job Queue Worker", long_about = None)]
struct Cli {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// Worker ID (unique identifier for this worker instance)
    #[arg(long, default_value = "worker-1")]
    worker_id: String,

    /// Maximum job processing time in minutes
    #[arg(long, default_value = "60")]
    job_timeout_minutes: u64,

    /// Poll interval when no jobs are available (seconds)
    #[arg(long, default_value = "10")]
    poll_interval_seconds: u64,
}

#[derive(Debug)]
struct Worker {
    pool: PgPool,
    worker_id: String,
    config: WorkerConfig,
    metrics: WorkerMetrics,
}

#[derive(Debug, Clone)]
struct WorkerConfig {
    job_timeout_minutes: u64,
    poll_interval_seconds: u64,
}

#[derive(Debug, Default)]
struct WorkerMetrics {
    jobs_processed: u64,
    jobs_succeeded: u64,
    jobs_failed: u64,
    total_processing_time_ms: u64,
}

impl Worker {
    fn new(database_url: String, worker_id: String, config: WorkerConfig) -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        let pool = runtime.block_on(async {
            PgPool::connect(&database_url).await
        })?;

        Ok(Self {
            pool,
            worker_id,
            config,
            metrics: WorkerMetrics::default(),
        })
    }

    async fn run(&mut self) -> Result<()> {
        info!("Starting job worker {} (timeout: {}min, poll: {}s)",
              self.worker_id, self.config.job_timeout_minutes, self.config.poll_interval_seconds);

        // Set up PostgreSQL listener for job notifications
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen("job_available").await?;

        loop {
            // Try to get a job
            match self.acquire_job().await {
                Ok(Some(mut job)) => {
                    let start_time = std::time::Instant::now();

                    match self.process_job(&mut job).await {
                        Ok(_) => {
                            self.record_job_success(&job, start_time.elapsed()).await?;
                            self.metrics.jobs_succeeded += 1;
                        }
                        Err(e) => {
                            self.record_job_failure(&job, &e.to_string(), start_time.elapsed()).await?;
                            self.metrics.jobs_failed += 1;
                        }
                    }

                    self.metrics.jobs_processed += 1;
                    self.log_metrics().await;
                }
                Ok(None) => {
                    // No jobs available, wait for notification or poll
                    tokio::select! {
                        notification = listener.recv() => {
                            match notification {
                                Ok(_) => info!("Received job notification"),
                                Err(e) => warn!("Listener error: {}", e),
                            }
                        }
                        _ = sleep(Duration::from_secs(self.config.poll_interval_seconds)) => {
                            // Poll timeout, continue loop
                        }
                    }
                }
                Err(e) => {
                    error!("Error acquiring job: {}", e);
                    sleep(Duration::from_secs(self.config.poll_interval_seconds)).await;
                }
            }
        }
    }

    async fn acquire_job(&self) -> Result<Option<Job>> {
        // Try to acquire a job atomically
        let job: Option<Job> = sqlx::query_as(
            r#"
            UPDATE jobs
            SET status = 'running', started_at = NOW(), worker_id = $1
            WHERE id = (
                SELECT id FROM jobs
                WHERE status = 'queued'
                ORDER BY priority DESC, created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                      payload, created_at, started_at, completed_at, worker_id, error_message,
                      retry_count, max_retries
            "#
        )
        .bind(&self.worker_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    async fn process_job(&self, job: &mut Job) -> Result<()> {
        info!("Processing job {}: {}", job.id, job.name);

        // Set a timeout for job processing
        let timeout_duration = Duration::from_secs(self.config.job_timeout_minutes * 60);

        match timeout(timeout_duration, self.execute_job(job)).await {
            Ok(result) => result,
            Err(_) => {
                Err(Error::JobQueue(format!("Job {} timed out after {} minutes",
                    job.id, self.config.job_timeout_minutes)))
            }
        }
    }

    async fn execute_job(&self, job: &mut Job) -> Result<()> {
        // This is where the actual job processing happens
        // For now, we'll simulate different types of jobs

        let payload: serde_json::Value = job.payload.clone();

        match payload.get("type").and_then(|t| t.as_str()) {
            Some("span_processing") => {
                self.process_span_job(job, payload).await
            }
            Some("manuscript_generation") => {
                self.process_manuscript_job(job, payload).await
            }
            Some("twin_analysis") => {
                self.process_twin_analysis_job(job, payload).await
            }
            _ => {
                // Generic job processing
                info!("Processing generic job {}", job.id);
                sleep(Duration::from_secs(2)).await; // Simulate work
                Ok(())
            }
        }
    }

    async fn process_span_job(&self, job: &mut Job, payload: serde_json::Value) -> Result<()> {
        info!("Processing span job {}", job.id);

        // Simulate span processing work
        sleep(Duration::from_secs(1)).await;

        // In a real implementation, this would:
        // 1. Load span data from payload
        // 2. Run triage to determine analysis stages
        // 3. Execute each stage with proper error handling
        // 4. Update job progress

        Ok(())
    }

    async fn process_manuscript_job(&self, job: &mut Job, payload: serde_json::Value) -> Result<()> {
        info!("Processing manuscript job {}", job.id);

        // Simulate manuscript generation
        sleep(Duration::from_secs(3)).await;

        Ok(())
    }

    async fn process_twin_analysis_job(&self, job: &mut Job, payload: serde_json::Value) -> Result<()> {
        info!("Processing twin analysis job {}", job.id);

        // Simulate twin analysis
        sleep(Duration::from_secs(2)).await;

        Ok(())
    }

    async fn record_job_success(&self, job: &Job, duration: Duration) -> Result<()> {
        let execution = JobExecution {
            job_id: job.id,
            worker_id: self.worker_id.clone(),
            started_at: job.started_at.unwrap_or_else(Utc::now),
            completed_at: Some(Utc::now()),
            success: true,
            error_message: None,
            metrics: serde_json::json!({
                "processing_time_ms": duration.as_millis(),
            }),
        };

        // Record job execution
        sqlx::query(
            r#"
            INSERT INTO job_executions (job_id, worker_id, started_at, completed_at, success, metrics)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&execution.job_id)
        .bind(&execution.worker_id)
        .bind(&execution.started_at)
        .bind(&execution.completed_at)
        .bind(&execution.success)
        .bind(&execution.metrics)
        .execute(&self.pool)
        .await?;

        // Mark job as completed
        sqlx::query(
            r#"
            UPDATE jobs
            SET status = 'completed', completed_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(&job.id)
        .execute(&self.pool)
        .await?;

        info!("Job {} completed successfully (took {}ms)", job.id, duration.as_millis());
        Ok(())
    }

    async fn record_job_failure(&self, job: &Job, error_msg: &str, duration: Duration) -> Result<()> {
        let execution = JobExecution {
            job_id: job.id,
            worker_id: self.worker_id.clone(),
            started_at: job.started_at.unwrap_or_else(Utc::now),
            completed_at: Some(Utc::now()),
            success: false,
            error_message: Some(error_msg.to_string()),
            metrics: serde_json::json!({
                "processing_time_ms": duration.as_millis(),
            }),
        };

        // Record job execution
        sqlx::query(
            r#"
            INSERT INTO job_executions (job_id, worker_id, started_at, completed_at, success, error_message, metrics)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&execution.job_id)
        .bind(&execution.worker_id)
        .bind(&execution.started_at)
        .bind(&execution.completed_at)
        .bind(&execution.success)
        .bind(&execution.error_message)
        .bind(&execution.metrics)
        .execute(&self.pool)
        .await?;

        // Handle job failure with retry logic
        if job.can_retry() {
            sqlx::query(
                r#"
                UPDATE jobs
                SET status = 'queued', started_at = NULL, worker_id = NULL,
                    retry_count = retry_count + 1, error_message = $2
                WHERE id = $1
                "#
            )
            .bind(&job.id)
            .bind(error_msg)
            .execute(&self.pool)
            .await?;

            info!("Job {} failed, queued for retry (attempt {})", job.id, job.retry_count + 1);
        } else {
            sqlx::query(
                r#"
                UPDATE jobs
                SET status = 'failed', completed_at = NOW(), error_message = $2
                WHERE id = $1
                "#
            )
            .bind(&job.id)
            .bind(error_msg)
            .execute(&self.pool)
            .await?;

            info!("Job {} failed permanently after {} attempts", job.id, job.max_retries);
        }

        Ok(())
    }

    async fn log_metrics(&self) {
        info!("Worker {} metrics: processed={}, succeeded={}, failed={}, avg_time={}ms",
              self.worker_id,
              self.metrics.jobs_processed,
              self.metrics.jobs_succeeded,
              self.metrics.jobs_failed,
              if self.metrics.jobs_processed > 0 {
                  self.metrics.total_processing_time_ms / self.metrics.jobs_processed
              } else {
                  0
              });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = WorkerConfig {
        job_timeout_minutes: cli.job_timeout_minutes,
        poll_interval_seconds: cli.poll_interval_seconds,
    };

    let mut worker = Worker::new(cli.database_url, cli.worker_id, config)?;
    worker.run().await
}
