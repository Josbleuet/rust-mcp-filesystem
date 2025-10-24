<p align="center">
  <img width="96" src="./docs/_media/rust-mcp-filesystem.png" alt="Rust MCP Filesystem Logo" width="300">
</p>

# Rust MCP Filesystem

Rust MCP Filesystem is a blazingly fast, asynchronous, and lightweight MCP (Model Context Protocol) server designed for efficient handling of various filesystem operations.
This project is a pure Rust rewrite of the JavaScript-based `@modelcontextprotocol/server-filesystem`, offering enhanced capabilities, improved performance, and a robust feature set tailored for modern filesystem interactions.

📝 Refer to the [project documentation](https://rust-mcp-stack.github.io/rust-mcp-filesystem) for installation and configuration instructions.

⭐️ It is also available on [Docker Hub’s MCP Registry](https://hub.docker.com/mcp/server/rust-mcp-filesystem) at: https://hub.docker.com/mcp/server/rust-mcp-filesystem

## Features

- **🔌 Dual Transport Support**: Supports both stdio and HTTP/SSE transports via runtime configuration (stdio by default, HTTP for multi-client scenarios).
- **⚡ High Performance**: Built in Rust for speed and efficiency, leveraging asynchronous I/O to handle filesystem operations seamlessly.
- **🔒 Read-Only by Default**: Starts with no write access, ensuring safety until explicitly configured otherwise.
- **🔍 Advanced Glob Search**: Supports full glob pattern matching allowing precise filtering of files and directories using standard glob syntax.For example, patterns like `*.rs`, `src/**/*.txt`, and `logs/error-???.log` are valid and can be used to match specific file types, recursive directory searches, or patterned filenames.
- **🔄 MCP Roots support**: enabling clients to dynamically modify the list of allowed directories (disabled by default).
- **📦 ZIP Archive Support**: Tools to create ZIP archives from files or directories and extract ZIP files with ease.
- **🪶 Lightweight**: Standalone with no external dependencies (e.g., no Node.js, Python etc required), compiled to a single binary with a minimal resource footprint, ideal for both lightweight and extensive deployment scenarios.

#### 👉 Refer to [capabilities](https://rust-mcp-stack.github.io/rust-mcp-filesystem/#/capabilities) for a full list of tools and other capabilities.

## 🔧 Installation & Configuration

For detailed setup instructions, please visit the [project documentation](https://rust-mcp-stack.github.io/rust-mcp-filesystem).


### Quick installation guide


<!-- x-release-please-start-version -->
- **Shell script**
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.3.6/rust-mcp-filesystem-installer.sh | sh
```

- **PowerShell script**
```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/download/v0.3.6/rust-mcp-filesystem-installer.ps1 | iex"
```

- **Homebrew**
```sh
brew install rust-mcp-stack/tap/rust-mcp-filesystem
```
- **Docker**

  https://hub.docker.com/mcp/server/rust-mcp-filesystem

- **Download Binaries**

  https://github.com/rust-mcp-stack/rust-mcp-filesystem/releases/tag/v0.3.6

<!-- x-release-please-end -->

## 📖 Usage

The server supports **two transport modes**: **stdio** (default) and **HTTP/SSE**.

### Mode stdio (Default)

Ideal for local MCP clients (e.g., Claude Desktop):

```bash
# Read-only mode
rust-mcp-filesystem /path/to/dir1 /path/to/dir2

# With write permissions
rust-mcp-filesystem /path/to/dir1 /path/to/dir2 --allow-write

# With MCP Roots support
rust-mcp-filesystem --enable-roots
```

**MCP Configuration (stdio):**
```json
{
  "mcpServers": {
    "filesystem": {
      "type": "stdio",
      "command": "rust-mcp-filesystem",
      "args": ["/path/to/allowed/dir1", "/path/to/allowed/dir2", "--allow-write"]
    }
  }
}
```

### Mode HTTP/SSE

Ideal for network connections and simultaneous multi-client support:

```bash
# Start HTTP server on localhost:3000
rust-mcp-filesystem /path/to/dir1 /path/to/dir2 --transport http

# Customize host and port
rust-mcp-filesystem /path/to/dir1 --allow-write --transport http --host 0.0.0.0 --port 8080

# Enable SSL (requires certificates)
rust-mcp-filesystem /path/to/dir1 --transport http --enable-ssl
```

**MCP Configuration (HTTP):**
```json
{
  "mcpServers": {
    "filesystem": {
      "type": "http",
      "url": "http://localhost:3000/mcp"
    }
  }
}
```

**Advantages of HTTP mode**:
- ✅ Support for simultaneous connections from multiple clients
- ✅ Accessible over local network
- ✅ SSE (Server-Sent Events) support for streaming
- ✅ Optional SSL for security

### Environment Variables

All CLI arguments can also be set via environment variables:

- `ALLOW_WRITE=true` - Enable write mode
- `ENABLE_ROOTS=true` - Enable MCP Roots support
- `TRANSPORT_MODE=http` - Set transport mode
- `HTTP_HOST=0.0.0.0` - Set HTTP host
- `HTTP_PORT=8080` - Set HTTP port
- `ENABLE_SSL=true` - Enable SSL

## Purpose

This project aims to provide a reliable, secure, and feature-rich MCP server for filesystem management, reimagining the capabilities of @modelcontextprotocol/server-filesystem in a more performant and type-safe language. Whether you’re building tools for file exploration, automation, or system integration, rust-mcp-filesystem offers a solid foundation.

## 🧰 Built With

The project leverages the [rust-mcp-sdk](https://github.com/rust-mcp-stack/rust-mcp-sdk) and [rust-mcp-schema](https://github.com/rust-mcp-stack/rust-mcp-schema) to build this server. check out those repositories if you’re interested in crafting your own Rust-based MCP project or converting existing ones to Rust for enhanced performance and safety.

## License

This project is licensed under the MIT License. see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by `@modelcontextprotocol/server-filesystem` and built with the power of Rust.
