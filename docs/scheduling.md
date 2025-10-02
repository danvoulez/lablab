# Scheduling & Automation Guide

LogLine Discovery Lab is designed to run continuously. This guide shows how to keep ingestion and manuscript generation online using either `systemd` timers or classic cron jobs. Adjust paths as needed for your environment.

## systemd Services (recommended)

### Span Watcher (long-running)
1. Copy `ops/systemd/logline-discovery-watcher.service` to `~/.config/systemd/user/` (or `/etc/systemd/system/` for a system instance).
2. Ensure the `.env` file under `logline_discovery/.env` contains `DATABASE_URL` and `LEDGER_PATH`.
3. Enable and start:
   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now logline-discovery-watcher.service
   journalctl --user -u logline-discovery-watcher.service -f   # tail logs
   ```

The watcher uses the new `cargo run -p hiv_discovery_runner -- watch ...` subcommand to automatically ingest Fold spans.

### Manuscript Job (scheduled)
1. Copy `ops/systemd/logline-discovery-manuscript.service` and `.timer` into your user/system `systemd` directory.
2. Enable the timer:
   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now logline-discovery-manuscript.timer
   ```
3. Inspect results in `logline_discovery/manuscripts/` or via logs: `journalctl --user -u logline-discovery-manuscript.service`.

The timer runs hourly (with a small jitter) and calls `scripts/run_manuscript_job.sh`, which in turn executes `cargo run -p hiv_discovery_runner -- manuscript` to produce a new bundle.

## Cron Alternative

If `systemd` isn’t available, you can schedule cron jobs instead:

```cron
# Watcher (restarts on reboot)
@reboot cd "$HOME/LogLine Discovery Lab/logline_discovery" && \ 
  cargo run -p hiv_discovery_runner -- watch --path "$HOME/Library/Mobile Documents/com~apple~CloudDocs/LogLine Fold/Backend/spans" --recursive --interval 60 >> "$HOME/logline_watcher.log" 2>&1 &

# Manuscript job (hourly)
0 * * * * cd "$HOME/LogLine Discovery Lab" && ./scripts/run_manuscript_job.sh >> "$HOME/logline_manuscript.log" 2>&1
```

Cron cannot keep long-running processes alive automatically, so we restart the watcher on boot and rely on the CLI’s internal loop.

## Monitoring & Alerts
- `journalctl` (systemd) or log files (cron) surface errors; hook into a log shipper or alerting system as desired.
- Future tasks include exposing Prometheus metrics and integrating heartbeat alerts for missed jobs (see Phase 2 backlog).

## Next Automation Steps
- Deploy the watcher + timer to your preferred host (local workstation, NAS, or server).
- Integrate analytics/causal/manuscript jobs into the same scheduling infrastructure as they come online.
- Mirror historical Warp NDJSON using a dedicated batch job (upcoming backlog item).
