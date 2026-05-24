//! Output formatter for route responses.
//!
//! Converts a [`RunOutput`] variant into a human-readable string
//! suitable for CSV or JSON serialisation.

use crate::engine::experience::runner::RunOutput;

/// Formats a [`RunOutput`] into a display string.
pub fn format(out: &RunOutput) -> String {
    match out {
        RunOutput::Scalar(v) => v.to_string(),
        RunOutput::Integer(i) => i.to_string(),
        RunOutput::Boolean(b) => b.to_string(),
        RunOutput::Text(t) => format!("\"{t}\""),
        RunOutput::Pair(a, b) => format!("[{a},{b}]"),
        RunOutput::Triple(a, b, c) => format!("[{a},{b},{c}]"),
        RunOutput::Vector(v) => fvec(v),
        RunOutput::IntVector(iv) => {
            let s: Vec<String> = iv.iter().map(|x| x.to_string()).collect();
            format!("[{}]", s.join(","))
        }
        RunOutput::PairVec(pv) => {
            let s: Vec<String> = pv.iter().map(|(a, b)| format!("[{a},{b}]")).collect();
            format!("[{}]", s.join(","))
        }
        RunOutput::Matrix(m) => {
            let rows: Vec<String> = m.iter().map(|r| fvec(r)).collect();
            format!("[{}]", rows.join(","))
        }
        RunOutput::Complex(re, im) => format!(r#"{{"re":{re},"im":{im}}}"#),
        RunOutput::ComplexVector(cv) => {
            let s: Vec<String> = cv
                .iter()
                .map(|(re, im)| format!(r#"{{"re":{re},"im":{im}}}"#))
                .collect();
            format!("[{}]", s.join(","))
        }
        RunOutput::ComplexMatrix(cm) => {
            let rows: Vec<String> = cm
                .iter()
                .map(|row| {
                    let s: Vec<String> = row
                        .iter()
                        .map(|(re, im)| format!(r#"{{"re":{re},"im":{im}}}"#))
                        .collect();
                    format!("[{}]", s.join(","))
                })
                .collect();
            format!("[{}]", rows.join(","))
        }
        RunOutput::PolynomialOut(c) => fvec(c),
        RunOutput::TensorOut { data, shape } => {
            let sh: Vec<String> = shape.iter().map(|x| x.to_string()).collect();
            format!(r#"{{"data":{},"shape":[{}]}}"#, fvec(data), sh.join(","))
        }
        RunOutput::SparseOut {
            rows,
            cols,
            row_ptr,
            col_idx,
            values,
        } => {
            let rp: Vec<String> = row_ptr.iter().map(|x| x.to_string()).collect();
            let ci: Vec<String> = col_idx.iter().map(|x| x.to_string()).collect();
            format!(
                r#"{{"rows":{rows},"cols":{cols},"row_ptr":[{}],"col_idx":[{}],"values":{}}}"#,
                rp.join(","),
                ci.join(","),
                fvec(values)
            )
        }
        RunOutput::TimeSeries { times, values } => {
            format!(r#"{{"times":{},"values":{}}}"#, fvec(times), fvec(values))
        }
    }
}

fn fvec(v: &[f64]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(","))
}
