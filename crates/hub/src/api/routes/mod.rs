//! Domain-agnostic routing layer.
//!
//! Parses a `(domain, function, params)` triplet, builds an
//! [`Experiment`], runs it through the engine, and formats the output.

#[cfg(any(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
))]
mod formatter;
#[cfg(any(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
))]
mod params;

/// Parses a raw `(domain, function, params)` triplet, builds an [`Experiment`], runs it, and returns a CSV-formatted result string.
#[cfg(any(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
))]
pub fn route(domain: &str, function: &str, raw_params: &[&str]) -> String {
    use crate::engine::experience::experiment::{DomainType, Experiment};
    use crate::tools::benchmark::run_timed;
    let domain_type = match domain {
        #[cfg(feature = "maths")]
        "maths" | "math" => DomainType::Maths,
        #[cfg(feature = "physics")]
        "physics" | "phys" => DomainType::Physics,
        #[cfg(feature = "chemistry")]
        "chemistry" | "chem" => DomainType::Chemistry,
        #[cfg(feature = "biology")]
        "biology" | "bio" => DomainType::Biology,
        #[cfg(feature = "astronomy")]
        "astronomy" | "astro" => DomainType::Astronomy,
        #[cfg(feature = "geology")]
        "geology" | "geo" => DomainType::Geology,
        #[cfg(feature = "meteorology")]
        "meteorology" | "meteo" => DomainType::Meteorology,
        _ => return format!(r#"{{"error":"unknown or disabled domain: {domain}"}}"#),
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

/// Fallback when no domain feature is enabled.
#[cfg(not(any(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
)))]
pub fn route(domain: &str, _function: &str, _raw_params: &[&str]) -> String {
    format!(r#"{{"error":"no domain features enabled, cannot route domain: {domain}"}}"#)
}
