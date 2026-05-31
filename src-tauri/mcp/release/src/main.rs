use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ServiceExt,
};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BumpVersionArgs {
    version: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GitTagArgs {
    version: String,
}

pub struct FlameReleaseService {
    tool_router: ToolRouter<FlameReleaseService>,
}

#[tool_router]
impl FlameReleaseService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    fn find_project_root(&self) -> Result<String, McpError> {
        let cwd = std::env::current_dir()
            .map_err(|e| McpError::internal_error(format!("Cannot get cwd: {}", e), None))?;
        let mut dir = Some(cwd.as_path());
        while let Some(d) = dir {
            if d.join("package.json").exists() && d.join("src-tauri").exists() {
                return Ok(d.to_string_lossy().to_string());
            }
            dir = d.parent();
        }
        Err(McpError::internal_error(
            "Cannot find project root (no package.json + src-tauri)",
            None,
        ))
    }

    fn read_version_json(&self, path: &str) -> Result<String, McpError> {
        let pkg = fs::read_to_string(Path::new(path).join("package.json"))
            .map_err(|e| McpError::internal_error(format!("Cannot read package.json: {}", e), None))?;
        let json: serde_json::Value = serde_json::from_str(&pkg)
            .map_err(|e| McpError::internal_error(format!("Cannot parse package.json: {}", e), None))?;
        json["version"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| McpError::internal_error("Cannot find version in package.json", None))
    }

    fn read_version_cargo(&self, path: &str) -> Result<String, McpError> {
        let cargo_path = Path::new(path).join("src-tauri").join("Cargo.toml");
        let content = fs::read_to_string(&cargo_path)
            .map_err(|e| McpError::internal_error(format!("Cannot read Cargo.toml: {}", e), None))?;
        let doc: toml::Value = toml::from_str(&content)
            .map_err(|e| McpError::internal_error(format!("Cannot parse Cargo.toml: {}", e), None))?;
        doc["package"]["version"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| McpError::internal_error("Cannot find version in Cargo.toml", None))
    }

    fn write_version_json(&self, path: &str, version: &str) -> Result<(), McpError> {
        let pkg_path = Path::new(path).join("package.json");
        let content = fs::read_to_string(&pkg_path)
            .map_err(|e| McpError::internal_error(format!("Cannot read package.json: {}", e), None))?;
        let mut json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| McpError::internal_error(format!("Cannot parse package.json: {}", e), None))?;
        json["version"] = serde_json::Value::String(version.to_string());
        let updated = serde_json::to_string_pretty(&json)
            .map_err(|e| McpError::internal_error(format!("Cannot serialize: {}", e), None))?;
        fs::write(&pkg_path, updated)
            .map_err(|e| McpError::internal_error(format!("Cannot write package.json: {}", e), None))?;
        Ok(())
    }

    fn write_version_cargo(&self, path: &str, version: &str) -> Result<(), McpError> {
        let cargo_path = Path::new(path).join("src-tauri").join("Cargo.toml");
        let content = fs::read_to_string(&cargo_path)
            .map_err(|e| McpError::internal_error(format!("Cannot read Cargo.toml: {}", e), None))?;
        let re = regex::Regex::new(r#"^version\s*=\s*"[^"]*""#)
            .map_err(|e| McpError::internal_error(format!("Regex error: {}", e), None))?;
        let updated = re
            .replace(&content, format!(r#"version = "{}""#, version))
            .to_string();
        fs::write(&cargo_path, updated)
            .map_err(|e| McpError::internal_error(format!("Cannot write Cargo.toml: {}", e), None))?;
        Ok(())
    }

    #[tool(description = "Show current version from package.json and Cargo.toml")]
    fn current_version(&self) -> Result<CallToolResult, McpError> {
        let root = self.find_project_root()?;
        let json_ver = self.read_version_json(&root)?;
        let cargo_ver = self.read_version_cargo(&root)?;
        let output = format!(
            "Current versions:\n  package.json: {}\n  Cargo.toml:   {}",
            json_ver, cargo_ver
        );
        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Bump version in package.json and Cargo.toml")]
    fn bump_version(
        &self,
        Parameters(args): Parameters<BumpVersionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let root = self.find_project_root()?;
        let old_json = self.read_version_json(&root)?;
        let old_cargo = self.read_version_cargo(&root)?;

        self.write_version_json(&root, &args.version)?;
        self.write_version_cargo(&root, &args.version)?;

        let output = format!(
            "Version bumped:\n  package.json: {} → {}\n  Cargo.toml:   {} → {}",
            old_json, args.version, old_cargo, args.version
        );
        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Run pnpm tauri build for production")]
    fn build(&self) -> Result<CallToolResult, McpError> {
        let root = self.find_project_root()?;
        let status = Command::new("pnpm")
            .args(["tauri", "build"])
            .current_dir(&root)
            .status()
            .map_err(|e| McpError::internal_error(format!("Build failed: {}", e), None))?;

        let msg = if status.success() {
            "Build succeeded!".to_string()
        } else {
            format!("Build failed with exit code: {:?}", status.code())
        };
        Ok(CallToolResult::success(vec![Content::text(msg)]))
    }

    #[tool(description = "Create a git tag for the given version")]
    fn git_tag(
        &self,
        Parameters(args): Parameters<GitTagArgs>,
    ) -> Result<CallToolResult, McpError> {
        let tag = format!("v{}", args.version);
        let output = Command::new("git")
            .args(["tag", "-a", &tag, "-m", &format!("Release {}", tag)])
            .output()
            .map_err(|e| McpError::internal_error(format!("Git tag failed: {}", e), None))?;

        let msg = if output.status.success() {
            format!("Created git tag: {}", tag)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("Failed to create tag: {}", stderr)
        };
        Ok(CallToolResult::success(vec![Content::text(msg)]))
    }
}

#[tool_handler]
impl ServerHandler for FlameReleaseService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(Implementation::new("mcp-flame-release", "0.1.0"))
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
        .with_instructions(
            "Release management tools for Flame ADE V2: version bumping, building, and tagging."
                .to_string(),
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

    let service = FlameReleaseService::new();
    let server = service.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
