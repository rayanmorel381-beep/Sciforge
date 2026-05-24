//! Outbound computation response payload.
//!
//! [`ComputeResponse`] wraps the computation result (or error) together
//! with timing metadata returned to the caller.

/// Outbound computation response.
#[derive(Debug, Clone)]
pub struct ComputeResponse {
    /// Whether the computation succeeded.
    pub success: bool,
    /// Computation result, if successful.
    pub result: Option<ResultData>,
    /// Error message, if the computation failed.
    pub error: Option<String>,
    /// Wall-clock time in milliseconds.
    pub elapsed_ms: f64,
}

/// Typed result payload.
#[derive(Debug, Clone)]
pub enum ResultData {
    Scalar(f64),
    Pair(f64, f64),
    Triple(f64, f64, f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
    TimeSeries {
        times: Vec<f64>,
        values: Vec<Vec<f64>>,
    },
    Text(String),
    Boolean(bool),
}

impl ComputeResponse {
    /// Builds a successful scalar response.
    pub fn ok_scalar(value: f64, elapsed_ms: f64) -> Self {
        Self {
            success: true,
            result: Some(ResultData::Scalar(value)),
            error: None,
            elapsed_ms,
        }
    }

    /// Builds a successful vector response.
    pub fn ok_vector(values: Vec<f64>, elapsed_ms: f64) -> Self {
        Self {
            success: true,
            result: Some(ResultData::Vector(values)),
            error: None,
            elapsed_ms,
        }
    }

    /// Builds a successful time-series response.
    pub fn ok_time_series(times: Vec<f64>, values: Vec<Vec<f64>>, elapsed_ms: f64) -> Self {
        Self {
            success: true,
            result: Some(ResultData::TimeSeries { times, values }),
            error: None,
            elapsed_ms,
        }
    }

    /// Builds a successful text response.
    pub fn ok_text(text: String, elapsed_ms: f64) -> Self {
        Self {
            success: true,
            result: Some(ResultData::Text(text)),
            error: None,
            elapsed_ms,
        }
    }

    /// Builds a failure response with the given error message.
    pub fn fail(msg: &str, elapsed_ms: f64) -> Self {
        Self {
            success: false,
            result: None,
            error: Some(msg.to_string()),
            elapsed_ms,
        }
    }

    /// Serializes the response to a JSON string.
    pub fn to_json(&self) -> String {
        let result_str = match &self.result {
            Some(ResultData::Scalar(v)) => format!(r#""result":{v}"#),
            Some(ResultData::Pair(a, b)) => format!(r#""result":[{a},{b}]"#),
            Some(ResultData::Triple(a, b, c)) => format!(r#""result":[{a},{b},{c}]"#),
            Some(ResultData::Vector(v)) => format!(r#""result":{v:?}"#),
            Some(ResultData::Text(t)) => format!(r#""result":"{t}""#),
            Some(ResultData::Boolean(b)) => format!(r#""result":{b}"#),
            Some(ResultData::Matrix(_)) => r#""result":"<matrix>""#.to_string(),
            Some(ResultData::TimeSeries { .. }) => r#""result":"<time_series>""#.to_string(),
            None => r#""result":null"#.to_string(),
        };
        let err_str = match &self.error {
            Some(e) => format!(r#""error":"{e}""#),
            None => r#""error":null"#.to_string(),
        };
        format!(
            r#"{{"success":{},{result_str},{err_str},"elapsed_ms":{}}}"#,
            self.success, self.elapsed_ms
        )
    }
}
