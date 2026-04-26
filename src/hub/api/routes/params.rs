//! Raw parameter parser for route inputs.
//!
//! Converts `key=value` string slices into typed
//! [`ParameterValue`] pairs consumable by the experiment runner.

use crate::hub::engine::experience::experiment::ParameterValue;

/// Parses `key=value` slices into typed parameter pairs.
pub fn parse(raw: &[&str]) -> Vec<(String, ParameterValue)> {
    raw.iter()
        .filter_map(|s| {
            let (k, v) = s.split_once('=')?;
            Some((k.to_string(), parse_value(v)))
        })
        .collect()
}

fn parse_value(s: &str) -> ParameterValue {
    if s == "true" {
        return ParameterValue::Boolean(true);
    }
    if s == "false" {
        return ParameterValue::Boolean(false);
    }
    if let Some(inner) = s.strip_prefix("[[").and_then(|r| r.strip_suffix("]]"))
        && let Some(m) = parse_matrix(inner)
    {
        return ParameterValue::Matrix(m);
    }
    if let Some(inner) = s.strip_prefix('[').and_then(|r| r.strip_suffix(']'))
        && let Some(v) = parse_vec(inner)
    {
        return ParameterValue::Vector(v);
    }
    if let Some(inner) = s.strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
        let mut it = inner.splitn(2, ',');
        if let (Some(Ok(re)), Some(Ok(im))) = (
            it.next().map(|x| x.trim().parse::<f64>()),
            it.next().map(|x| x.trim().parse::<f64>()),
        ) {
            return ParameterValue::Complex(re, im);
        }
    }
    if !s.contains('.')
        && let Ok(i) = s.parse::<i64>()
    {
        return ParameterValue::Integer(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return ParameterValue::Scalar(f);
    }
    ParameterValue::Text(s.to_string())
}

fn parse_vec(inner: &str) -> Option<Vec<f64>> {
    if inner.is_empty() {
        return None;
    }
    inner.split(',').map(|x| x.trim().parse().ok()).collect()
}

fn parse_matrix(inner: &str) -> Option<Vec<Vec<f64>>> {
    inner.split("],[").map(parse_vec).collect()
}
