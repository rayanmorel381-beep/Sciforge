//! Error types for the Hub computation pipeline.
//!
//! [`HubError`] covers invalid inputs, out-of-range values, and
//! computation failures. [`HubResult`] is the corresponding alias.

/// Error types produced by hub computations.
#[derive(Debug, Clone)]
pub enum HubError {
    InvalidInput(String),
    ComputationFailed(String),
    OutOfRange {
        name: String,
        min: f64,
        max: f64,
        got: f64,
    },
    DimensionMismatch {
        expected: usize,
        got: usize,
    },
    NotConverged {
        iterations: usize,
    },
    EmptyData,
    Overflow,
    Underflow,
}

impl core::fmt::Display for HubError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::ComputationFailed(msg) => write!(f, "computation failed: {msg}"),
            Self::OutOfRange {
                name,
                min,
                max,
                got,
            } => write!(f, "{name} out of range [{min}, {max}]: {got}"),
            Self::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected}, got {got}")
            }
            Self::NotConverged { iterations } => {
                write!(f, "not converged after {iterations} iterations")
            }
            Self::EmptyData => write!(f, "empty data"),
            Self::Overflow => write!(f, "numeric overflow"),
            Self::Underflow => write!(f, "numeric underflow"),
        }
    }
}

impl std::error::Error for HubError {}

/// Convenience alias for hub computation results.
pub type HubResult<T> = Result<T, HubError>;
