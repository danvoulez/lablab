use logline_common::{
    job::{Job, JobPriority},
    Error, Result,
};
use chrono::Utc;
use clap::{Parser, Subcommand};
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about = "LogLine Job Queue Client", long_about = None)]
struct Cli {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Submit a new job to the queue
    Submit {
        /// Job name
        #[arg(short, long)]
        name: String,

        /// Job type
        #[arg(short, long)]
        job_type: String,

        /// Job priority
        #[arg(short, long, default_value = "normal")]
        priority: String,

        /// JSON payload for the job
        #[arg(short, long)]
        payload: Option<String>,

        /// Span ID to process (for span jobs)
        #[arg(long)]
        span_id: Option<String>,

        /// Execution span ID for manuscript generation
        #[arg(long)]
        execution_span: Option<String>,
    },

    /// List queued jobs
    List {
        /// Status filter (queued, running, completed, failed, cancelled)
        #[arg(short, long)]
        status: Option<String>,

        /// Limit number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },

    /// Show job details
    Details {
        /// Job ID
        job_id: Uuid,
    },

    /// Cancel a queued job
    Cancel {
        /// Job ID
        job_id: Uuid,
    },

    /// Show worker status
    Workers {
        /// Show detailed metrics
        #[arg(short, long)]
        detailed: bool,
    },
}

#[derive(Debug)]
struct JobClient {
    pool: PgPool,
}

impl JobClient {
    fn new(database_url: String) -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()?;
        let pool = runtime.block_on(async {
            PgPool::connect(&database_url).await
        })?;

        Ok(Self { pool })
    }

    async fn submit_job(
        &self,
        name: String,
        job_type: String,
        priority: JobPriority,
        payload: Value,
    ) -> Result<Uuid> {
        let job = Job::new(name, payload, priority);

        sqlx::query(
            r#"
            INSERT INTO jobs (id, name, status, priority, payload, created_at, retry_count, max_retries)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(&job.id)
        .bind(&job.name)
        .bind(&job.status)
        .bind(&job.priority)
        .bind(&job.payload)
        .bind(&job.created_at)
        .bind(&job.retry_count)
        .bind(&job.max_retries)
        .execute(&self.pool)
        .await?;

        // Notify scheduler that a job is available
        sqlx::query("NOTIFY job_available")
            .execute(&self.pool)
            .await?;

        info!("Submitted job {} ({})", job.id, job.name);
        Ok(job.id)
    }

    async fn list_jobs(&self, status: Option<String>, limit: u32) -> Result<()> {
        let status_filter = status.as_ref().map(|s| format!("AND status = '{}'", s))
            .unwrap_or_default();

        let jobs: Vec<Job> = sqlx::query_as(&format!(
            r#"
            SELECT id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                   payload, created_at, started_at, completed_at, worker_id, error_message,
                   retry_count, max_retries
            FROM jobs
            {}
            ORDER BY created_at DESC
            LIMIT {}
            "#,
            status_filter, limit
        ))
        .fetch_all(&self.pool)
        .await?;

        if jobs.is_empty() {
            println!("No jobs found");
            return Ok(());
        }

        println!("{:<36} {:<20} {:<10} {:<20} {}", "JOB ID", "NAME", "STATUS", "PRIORITY", "CREATED");
        println!("{}", "-".repeat(100));

        for job in jobs {
            println!("{:<36} {:<20} {:<10} {:<20} {}",
                job.id,
                job.name.chars().take(20).collect::<String>(),
                format!("{:?}", job.status),
                format!("{:?}", job.priority),
                job.created_at.format("%Y-%m-%d %H:%M:%S")
            );
        }

        Ok(())
    }

    async fn show_job_details(&self, job_id: Uuid) -> Result<()> {
        let job: Option<Job> = sqlx::query_as(
            r#"
            SELECT id, name, status as "status: JobStatus", priority as "priority: JobPriority",
                   payload, created_at, started_at, completed_at, worker_id, error_message,
                   retry_count, max_retries
            FROM jobs WHERE id = $1
            "#
        )
        .bind(&job_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(job) = job {
            println!("Job Details:");
            println!("  ID: {}", job.id);
            println!("  Name: {}", job.name);
            println!("  Status: {:?}", job.status);
            println!("  Priority: {:?}", job.priority);
            println!("  Created: {}", job.created_at.format("%Y-%m-%d %H:%M:%S"));
            if let Some(started) = job.started_at {
                println!("  Started: {}", started.format("%Y-%m-%d %H:%M:%S"));
            }
            if let Some(completed) = job.completed_at {
                println!("  Completed: {}", completed.format("%Y-%m-%d %H:%M:%S"));
            }
            if let Some(worker) = &job.worker_id {
                println!("  Worker: {}", worker);
            }
            if let Some(error) = &job.error_message {
                println!("  Error: {}", error);
            }
            println!("  Retries: {}/{}", job.retry_count, job.max_retries);
            println!("  Payload: {}", serde_json::to_string_pretty(&job.payload)?);
        } else {
            println!("Job {} not found", job_id);
        }

        Ok(())
    }

    async fn cancel_job(&self, job_id: Uuid) -> Result<()> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE jobs
            SET status = 'cancelled', completed_at = NOW()
            WHERE id = $1 AND status = 'queued'
            "#
        )
        .bind(&job_id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected > 0 {
            println!("Cancelled job {}", job_id);
        } else {
            println!("Job {} not found or not in queued status", job_id);
        }

        Ok(())
    }

    async fn show_worker_status(&self, detailed: bool) -> Result<()> {
        let workers: Vec<(String, i64, i64, i64, Option<String>)> = sqlx::query_as(
            r#"
            SELECT
                worker_id,
                COUNT(*) as total_jobs,
                COUNT(*) FILTER (WHERE success = true) as successful_jobs,
                COUNT(*) FILTER (WHERE success = false) as failed_jobs,
                MAX(completed_at) as last_activity
            FROM job_executions
            WHERE completed_at > NOW() - INTERVAL '1 hour'
            GROUP BY worker_id
            ORDER BY last_activity DESC NULLS LAST
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        if workers.is_empty() {
            println!("No active workers found");
            return Ok(());
        }

        if detailed {
            println!("{:<15} {:<10} {:<10} {:<10} {}", "WORKER", "TOTAL", "SUCCESS", "FAILED", "LAST ACTIVITY");
            println!("{}", "-".repeat(70));

            for (worker_id, total, success, failed, last_activity) in workers {
                println!("{:<15} {:<10} {:<10} {:<10} {}",
                    worker_id.chars().take(15).collect::<String>(),
                    total, success, failed,
                    last_activity.map(|t| t.format("%H:%M:%S").to_string()).unwrap_or_else(|| "Never".to_string())
                );
            }
        } else {
            println!("Active workers:");
            for (worker_id, _, _, _, _) in workers {
                println!("  • {}", worker_id);
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let client = JobClient::new(cli.database_url)?;

    match cli.command {
        Command::Submit { name, job_type, priority, payload, span_id, execution_span } => {
            let priority = match priority.as_str() {
                "low" => JobPriority::Low,
                "normal" => JobPriority::Normal,
                "high" => JobPriority::High,
                "critical" => JobPriority::Critical,
                _ => {
                    warn!("Invalid priority '{}', using 'normal'", priority);
                    JobPriority::Normal
                }
            };

            let mut job_payload = json!({
                "type": job_type,
            });

            if let Some(payload_str) = payload {
                let custom_payload: Value = serde_json::from_str(&payload_str)?;
                job_payload = custom_payload;
            } else if let Some(span_id) = span_id {
                job_payload = json!({
                    "type": "span_processing",
                    "span_id": span_id,
                });
            } else if let Some(exec_span) = execution_span {
                job_payload = json!({
                    "type": "manuscript_generation",
                    "execution_span": exec_span,
                });
            }

            let job_id = client.submit_job(name, job_type, priority, job_payload).await?;
            println!("Submitted job: {}", job_id);
        }

        Command::List { status, limit } => {
            client.list_jobs(status, limit).await?;
        }

        Command::Details { job_id } => {
            client.show_job_details(job_id).await?;
        }

        Command::Cancel { job_id } => {
            client.cancel_job(job_id).await?;
        }

        Command::Workers { detailed } => {
            client.show_worker_status(detailed).await?;
        }
    }

    Ok(())
}
