//! Main Director agent implementation

use crate::contracts::{Contract, Flow};
use crate::parser::Intent;
use crate::policy::Policy;
use crate::tools::*;
use crate::timeline::record_span;
use tracing::{info, warn};

pub struct Director;

impl Director {
    pub async fn handle(user_input: &str, actor: &str, actor_role: &str) -> String {
        info!("Processing request from {} ({})", actor, actor_role);

        let contract = Intent::parse(user_input, actor).await;

        if !Policy::allowed(actor_role, &contract) {
            warn!("Permission denied for {} to execute {:?}", actor, contract.flow);
            return "⛔ Insufficient permissions for this operation.".into();
        }

        if contract.requires_approval {
            // Record and return awaiting approval
            record_span("awaiting_approval", &contract);
            return "🟡 Operation created and awaiting approval.".into();
        }

        record_span("contract_received", &contract);

        let tool_res = match contract.flow {
            Flow::SubmitJob => SubmitJob.run(&contract).await,
            Flow::Diagnose => Diagnose.run(&contract).await,
            Flow::Monitor => Monitor.run(&contract).await,
            Flow::HotReload => HotReload.run(&contract).await,
            Flow::RequeueStuck => RequeueStuck.run(&contract).await,
            Flow::ScaleWorkers => ScaleWorkers.run(&contract).await,
            Flow::RotateLogs => RotateLogs.run(&contract).await,
            Flow::VacuumDb => VacuumDb.run(&contract).await,
            Flow::BackupSnap => BackupSnap.run(&contract).await,
            Flow::DatasetRegister => DatasetRegister.run(&contract).await,
            Flow::LabHealthcheck => LabHealthcheck.run(&contract).await,
            Flow::Unknown => ToolResult {
                ok: false,
                msg: "❓ I don't understand. Can you rephrase?".into(),
                extra: None,
            },
        };

        if tool_res.ok {
            record_span("success", &contract);
        } else {
            record_span("error", &contract);
        }

        tool_res.msg
    }
}
