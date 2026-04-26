//! REST / CLI interface layer.
//!
//! Provides HTTP and command-line adapters that accept user input,
//! route it through the engine, and return structured results.

/// Command-line interface adapter.
pub mod cli;
/// Data transfer objects (request/response payloads).
pub mod dto;
/// HTTP server adapter.
pub mod http;
/// Domain-agnostic routing: parses domain + function + params, dispatches to the engine..
pub mod routes;
