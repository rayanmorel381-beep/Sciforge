//! Domain-agnostic routing layer.
//!
//! Parses a `(domain, function, params)` triplet, builds an
//! [`Experiment`], runs it through the engine, and formats the output.

mod formatter;
mod params;

use crate::hub::engine::experience::experiment::{DomainType, Experiment};
use crate::hub::tools::benchmark::run_timed;

/// Parses a raw `(domain, function, params)` triplet, builds an [`Experiment`], runs it, and returns a CSV-formatted result string.
pub fn route(domain: &str, function: &str, raw_params: &[&str]) -> String {
    let domain_type = match domain {
        "maths" | "math" => DomainType::Maths,
        "physics" | "phys" => DomainType::Physics,
        "chemistry" | "chem" => DomainType::Chemistry,
        "biology" | "bio" => DomainType::Biology,
        "astronomy" | "astro" => DomainType::Astronomy,
        "geology" | "geo" => DomainType::Geology,
        "meteorology" | "meteo" => DomainType::Meteorology,
        _ => return format!(r#"{{"error":"unknown domain: {domain}"}}"#),
    };
    let experiment = Experiment {
        domain: domain_type,
        function_name: function.to_string(),
        parameters: params::parse(raw_params),
    };
    match run_timed(&experiment) {
        Ok((out, csv)) => {
            let val = formatter::format(&out);
            format!("{csv},result,{val}")
        }
        Err(e) => format!("error,{e}"),
    }
}
