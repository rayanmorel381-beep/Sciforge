//! Inbound computation request payload.
//!
//! [`ComputeRequest`] carries the domain, function name, and parameter
//! map submitted by the caller.

use std::collections::HashMap;

/// Inbound computation request.
#[derive(Debug, Clone)]
pub struct ComputeRequest {
    /// Target scientific domain.
    pub domain: String,
    /// Function to compute.
    pub function: String,
    /// Named parameters.
    pub params: HashMap<String, ParamValue>,
}

/// Typed parameter value.
#[derive(Debug, Clone)]
pub enum ParamValue {
    Scalar(f64),
    Integer(i64),
    Text(String),
    Boolean(bool),
    Array(Vec<f64>),
}

impl ComputeRequest {
    /// Creates an empty request for the given domain and function.
    pub fn new(domain: &str, function: &str) -> Self {
        Self {
            domain: domain.to_string(),
            function: function.to_string(),
            params: HashMap::new(),
        }
    }

    /// Adds a scalar parameter.
    pub fn with_scalar(mut self, key: &str, value: f64) -> Self {
        self.params
            .insert(key.to_string(), ParamValue::Scalar(value));
        self
    }

    /// Adds an integer parameter.
    pub fn with_integer(mut self, key: &str, value: i64) -> Self {
        self.params
            .insert(key.to_string(), ParamValue::Integer(value));
        self
    }

    /// Adds a text parameter.
    pub fn with_text(mut self, key: &str, value: &str) -> Self {
        self.params
            .insert(key.to_string(), ParamValue::Text(value.to_string()));
        self
    }

    /// Adds a boolean parameter.
    pub fn with_bool(mut self, key: &str, value: bool) -> Self {
        self.params
            .insert(key.to_string(), ParamValue::Boolean(value));
        self
    }

    /// Adds an array parameter.
    pub fn with_array(mut self, key: &str, value: Vec<f64>) -> Self {
        self.params
            .insert(key.to_string(), ParamValue::Array(value));
        self
    }

    /// Returns the scalar value for `key`, if present.
    pub fn get_scalar(&self, key: &str) -> Option<f64> {
        match self.params.get(key) {
            Some(ParamValue::Scalar(v)) => Some(*v),
            _ => None,
        }
    }

    /// Returns the integer value for `key`, if present.
    pub fn get_integer(&self, key: &str) -> Option<i64> {
        match self.params.get(key) {
            Some(ParamValue::Integer(v)) => Some(*v),
            _ => None,
        }
    }

    /// Returns the array value for `key`, if present.
    pub fn get_array(&self, key: &str) -> Option<&[f64]> {
        match self.params.get(key) {
            Some(ParamValue::Array(v)) => Some(v),
            _ => None,
        }
    }

    /// Attempts to parse a [`ComputeRequest`] from a JSON string.
    pub fn from_json_str(json: &str) -> Option<Self> {
        let mut domain = String::new();
        let mut function = String::new();
        for line in json.lines() {
            let trimmed = line.trim().trim_matches(',');
            if let Some(val) = trimmed
                .strip_prefix(r#""domain""#)
                .and_then(extract_json_string)
            {
                domain = val;
            } else if let Some(val) = trimmed
                .strip_prefix(r#""function""#)
                .and_then(extract_json_string)
            {
                function = val;
            }
        }
        if domain.is_empty() || function.is_empty() {
            return None;
        }
        Some(Self::new(&domain, &function))
    }
}

fn extract_json_string(s: &str) -> Option<String> {
    let s = s.trim().trim_start_matches(':').trim();
    let s = s.trim_matches('"').trim_matches(',');
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
