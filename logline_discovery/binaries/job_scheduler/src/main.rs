use logline_common::{
    config::Config,
    job::{Job, JobStatus, JobPriority},
    Error, Result,
};
use chrono::{DateTime, Utc};
use clap::Parser;
use sqlx::{PgPool, Row};
use std::time::Duration;
use tokio::time::{interval, sleep};
use tracing::{info, warn, error};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about = "LogLine Job Queue Scheduler", long_about = None)]
struct Cli {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    /// Scheduler interval in seconds
    #[arg(long, default_value = "30")]
    interval_seconds: u64,

    /// Maximum number of jobs to process per cycle
    #[arg(long, default_value = "10")]
    batch_size: usize,

    /// Worker timeout in minutes
    #[arg(long, default_value = "60")]
    worker_timeout_minutes: u64,
}

#[derive(Debug)]
struct Scheduler {
    pool: PgPool,
    config: SchedulerConfig,
}

#[derive(Debug, Clone)]
struct SchedulerConfig {
    interval_seconds: u64,
    batch_size: usize,
    worker_timeout_minutes: u64,
}

impl Scheduler {
    fn new(database_url: String, config: SchedulerConfig) -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        let pool = runtime.block_on(async {
            PgPool::connect(&database_url).await
        })?;

        Ok(Self { pool, config })
    }

    async fn run(&self) -> Result<()> {
        info!("Starting job scheduler (interval: {}s, batch: {})",
              self.config.interval_seconds, self.config.batch_size);

        let mut interval = interval(Duration::from_secs(self.config.interval_seconds));

        loop {
            interval.tick().await;

            if let Err(e) = self.process_jobs().await {
                error!("Error processing jobs: {}", e);
            }

            if let Err(e) = self.cleanup_stuck_jobs().await {
                error!("Error cleaning up stuck jobs: {}", e);
            }
        }
    }

    async fn process_jobs(&self) -> Result<()> {
        // Use SELECT FOR UPDATE SKIP LOCKED to atomically get and lock jobs
        let jobs: Vec<Job> = sqlx::query_as(
            r#"
            SELECT id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                   payload, created_at, started_at, completed_at, worker_id, error_message,
                   retry_count, max_retries
            FROM jobs
            WHERE status = 'queued'
            ORDER BY priority DESC, created_at ASC
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#
        )
        .bind(self.config.batch_size as i32)
        .fetch_all(&self.pool)
        .await?;

        for mut job in jobs {

            // Mark job as running
            job.status = JobStatus::Running;
            job.started_at = Some(Utc::now());

            // Record job execution
        let execution = JobExecution {
            job_id: job.id,
            worker_id: job.worker_id,
            started_at: job.started_at,
            completed_at: None,
            success: None,
            error_message: None,
            metrics: None,
        };

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

            // Notify workers that a job is available
            sqlx::query("NOTIFY job_available")
                .execute(&self.pool)
                .await?;
{{ ... }}
            info!("Job {} marked as running", job.id);
        }

        if !jobs.is_empty() {
            info!("Processed {} jobs in this cycle", jobs.len());
        }

        Ok(())
    }

    async fn cleanup_stuck_jobs(&self) -> Result<()> {
        let timeout_threshold = Utc::now() - chrono::Duration::minutes(self.config.worker_timeout_minutes as i64);

        // Find stuck jobs (running but past timeout)
        let stuck_jobs: Vec<Job> = sqlx::query_as!(
            Job,
            r#"
            SELECT id, name, payload, status as "status: JobStatus", 
                   created_at, started_at, completed_at, worker_id
            FROM jobs
            WHERE status = 'running'
            AND started_at < $1
            FOR UPDATE
            "#,
            timeout_threshold
        )
        .fetch_all(&self.pool)
        .await?;

        for mut job in stuck_jobs {
            info!("Cleaning up stuck job {} ({})", job.id, job.name);

            // Mark job as pending to retry
            job.status = JobStatus::Pending;
            job.started_at = None;
            job.worker_id = None;

            // Update job in database
            sqlx::query!(
                r#"
                UPDATE jobs
                SET status = $1, started_at = $2, worker_id = $3
                WHERE id = $4
                "#,
                job.status as JobStatus,
                job.started_at,
                job.worker_id,
                job.id
            )
            .execute(&self.pool)
            .await?;

            info!("Job {} reset to pending", job.id);
        }

        if !stuck_jobs.is_empty() {
            info!("Cleaned up {} stuck jobs", stuck_jobs.len());
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = SchedulerConfig {
        interval_seconds: cli.interval_seconds,
        batch_size: cli.batch_size,
        worker_timeout_minutes: cli.worker_timeout_minutes,
    };

    let scheduler = Scheduler::new(cli.database_url, config)?;
    scheduler.run().await
}
