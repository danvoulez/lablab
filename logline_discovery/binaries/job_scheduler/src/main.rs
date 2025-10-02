use chrono::{Duration, Utc};
use clap::Parser;
use logline_common::{job::Job, Result};
use sqlx::PgPool;
use std::time::Duration as StdDuration;
use tokio::time::interval;
use tracing::{error, info};

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
        let pool = runtime.block_on(async { PgPool::connect(&database_url).await })?;

        Ok(Self { pool, config })
    }

    async fn run(&self) -> Result<()> {
        info!(
            "Starting job scheduler (interval: {}s, batch: {})",
            self.config.interval_seconds, self.config.batch_size
        );

        let mut interval = interval(StdDuration::from_secs(self.config.interval_seconds));

        loop {
            interval.tick().await;

            if let Err(error) = self.process_jobs().await {
                error!(%error, "scheduler_process_jobs_failed");
            }

            if let Err(error) = self.cleanup_stuck_jobs().await {
                error!(%error, "scheduler_cleanup_failed");
            }
        }
    }

    async fn process_jobs(&self) -> Result<()> {
        let pending_jobs: Vec<Job> = sqlx::query_as::<_, Job>(
            r#"
            SELECT id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                   payload, created_at, started_at, completed_at, worker_id, error_message,
                   retry_count, max_retries
            FROM jobs
            WHERE status = 'pending'
            ORDER BY priority DESC, created_at ASC
            LIMIT $1
            "#,
        )
        .bind(self.config.batch_size as i64)
        .fetch_all(&self.pool)
        .await?;

        for job in &pending_jobs {
            sqlx::query(
                r#"
                UPDATE jobs
                SET status = 'queued', started_at = NULL, worker_id = NULL
                WHERE id = $1
                "#,
            )
            .bind(job.id)
            .execute(&self.pool)
            .await?;

            sqlx::query("NOTIFY job_available")
                .execute(&self.pool)
                .await?;

            info!(job_id = %job.id, job_name = %job.name, "job_queued");
        }

        if !pending_jobs.is_empty() {
            info!("Queued {} jobs in this cycle", pending_jobs.len());
        }

        Ok(())
    }

    async fn cleanup_stuck_jobs(&self) -> Result<()> {
        let timeout_threshold =
            Utc::now() - Duration::minutes(self.config.worker_timeout_minutes as i64);

        let stuck_jobs: Vec<Job> = sqlx::query_as::<_, Job>(
            r#"
            SELECT id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                   payload, created_at, started_at, completed_at, worker_id, error_message,
                   retry_count, max_retries
            FROM jobs
            WHERE status = 'running'
              AND started_at < $1
            "#,
        )
        .bind(timeout_threshold)
        .fetch_all(&self.pool)
        .await?;

        for job in &stuck_jobs {
            sqlx::query(
                r#"
                UPDATE jobs
                SET status = 'pending', started_at = NULL, worker_id = NULL
                WHERE id = $1
                "#,
            )
            .bind(job.id)
            .execute(&self.pool)
            .await?;

            info!(job_id = %job.id, job_name = %job.name, "job_reset_to_pending");
        }

        if !stuck_jobs.is_empty() {
            info!("Reset {} stuck jobs", stuck_jobs.len());
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
