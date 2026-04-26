//! Data-transfer objects exchanged between the API layer and the engine.
//!
//! Contains the [`request::ComputeRequest`] and [`response::ComputeResponse`]
//! payloads used by both HTTP and CLI adapters.

/// Inbound computation request payload.
pub mod request;
/// Outbound computation response payload.
pub mod response;
