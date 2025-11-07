// Configuration progressive vers zéro tolérance
// #![deny(warnings)]
// #![deny(clippy::all)]

use clap::Parser;
use rust_mcp_filesystem::{cli, server};

#[tokio::main]
async fn main() {
    let arguments = cli::CommandArguments::parse();
    if let Err(err) = arguments.validate() {
        eprintln!("Error: {err}");
        return;
    };

    // Initialize logger based on verbose flag
    init_logger(arguments.verbose);

    if let Err(error) = server::start_server(arguments).await {
        eprintln!("{error}");
    }
}

/// Initialize the tracing subscriber for logging
fn init_logger(verbose: bool) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = if verbose {
        // Verbose mode: show all logs from this crate
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("rust_mcp_filesystem=debug,rust_mcp_sdk=debug"))
    } else {
        // Normal mode: show info level and above
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("rust_mcp_filesystem=info,rust_mcp_sdk=info"))
    };

    // CRITICAL: Route logs to stderr to avoid corrupting JSON-RPC protocol on stdout
    // ANSI colors are safe on stderr since stdout remains pure JSON
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_line_number(true)
        .with_writer(std::io::stderr) // Logs on stderr, so ANSI colors don't corrupt stdout JSON
        .init();
}
