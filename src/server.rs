use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};
use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    mcp_server::{HyperServerOptions, hyper_server, server_runtime},
};

use crate::handler::FileSystemHandler;
use crate::{
    cli::{CommandArguments, TransportMode},
    error::ServiceResult,
};

pub fn server_details() -> InitializeResult {
    InitializeResult {
        server_info: Implementation {
            name: "rust-mcp-filesystem".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title:Some("Filesystem MCP Server: fast and efficient tools for managing filesystem operations.".to_string())
        },
        capabilities: ServerCapabilities {
            experimental: None,
            logging: None,
            prompts: None,
            resources: None,
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            completions: None,
        },
        instructions: None,
        meta: None,
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    }
}

/// Start the MCP server with stdio transport
async fn start_stdio_server(args: CommandArguments) -> ServiceResult<()> {
    let transport = StdioTransport::new(TransportOptions::default())?;

    let handler = FileSystemHandler::new(&args)?;
    let server = server_runtime::create_server(server_details(), transport, handler);

    eprintln!("Starting MCP Filesystem server with stdio transport");
    server.start().await?;

    Ok(())
}

/// Start the MCP server with HTTP/SSE transport
async fn start_http_server(args: CommandArguments) -> ServiceResult<()> {
    let handler = FileSystemHandler::new(&args)?;

    let options = HyperServerOptions {
        host: args.host.clone(),
        port: args.port,
        enable_ssl: args.enable_ssl,
        sse_support: true, // Enable Server-Sent Events
        ..Default::default()
    };

    let server = hyper_server::create_server(server_details(), handler, options);

    eprintln!(
        "Starting MCP Filesystem server with HTTP transport at {}://{}:{}",
        if args.enable_ssl { "https" } else { "http" },
        args.host,
        args.port
    );

    server.start().await?;

    Ok(())
}

/// Start the MCP server with the selected transport mode
pub async fn start_server(args: CommandArguments) -> ServiceResult<()> {
    match args.transport {
        TransportMode::Stdio => start_stdio_server(args).await,
        TransportMode::Http => start_http_server(args).await,
    }
}
