use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ServiceExt,
};
use serde::Deserialize;
use std::process::Command;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AuditArgs {
    path: Option<String>,
}

pub struct CargoAuditService {
    tool_router: ToolRouter<CargoAuditService>,
}

#[tool_router]
impl CargoAuditService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Run cargo audit to check Rust dependencies for known vulnerabilities")]
    fn audit(
        &self,
        Parameters(args): Parameters<AuditArgs>,
    ) -> Result<CallToolResult, McpError> {
        let project_path = args.path.unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string())
        });

        let output = Command::new("cargo")
            .args(["audit"])
            .current_dir(&project_path)
            .output()
            .map_err(|e| {
                McpError::internal_error(format!("Failed to run cargo audit: {}", e), None)
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(CallToolResult::success(vec![Content::text(stdout)]))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let combined = format!("{}{}", stdout, stderr);
            if stderr.contains("no advisory database") {
                Err(McpError::internal_error(
                    "No advisory database found. Run `cargo audit init` first.",
                    None,
                ))
            } else {
                Ok(CallToolResult::success(vec![Content::text(combined)]))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for CargoAuditService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(Implementation::new("mcp-cargo-audit", "0.1.0"))
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
        .with_instructions(
            "Run cargo audit to check Rust dependencies for known vulnerabilities.".to_string(),
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let service = CargoAuditService::new();
    let server = service.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
