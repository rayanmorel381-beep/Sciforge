//! Ergonomic re-exports of the most commonly used Hub types and functions.
//!
//! Importing `use sciforge_hub::prelude::*;` gives access to requests,
//! responses, domain modules, engine primitives, and utility helpers
//! without deep path qualifications.

pub use crate::api::dto::request::ComputeRequest;
pub use crate::api::dto::response::ComputeResponse;

#[cfg(feature = "astronomy")]
pub use crate::domain::astronomy;
#[cfg(feature = "biology")]
pub use crate::domain::biology;
#[cfg(feature = "chemistry")]
pub use crate::domain::chemistry;
#[cfg(feature = "geology")]
pub use crate::domain::geology;
#[cfg(feature = "maths")]
pub use crate::domain::maths;
#[cfg(feature = "meteorology")]
pub use crate::domain::meteorology;
#[cfg(feature = "physics")]
pub use crate::domain::physics;

pub use crate::domain::common::constants;
pub use crate::domain::common::errors::{HubError, HubResult};
pub use crate::domain::common::units::{
    AngleUnit, EnergyUnit, LengthUnit, MassUnit, PressureUnit, TemperatureUnit, TimeUnit,
    angle_to_radian, convert_length, convert_mass, convert_time, energy_from_si, energy_to_si,
    kelvin_to_temperature, length_from_si, length_to_si, mass_from_si, mass_to_si,
    pressure_from_si, pressure_to_si, radian_to_angle, temperature_to_kelvin, time_from_si,
    time_to_si,
};

pub use crate::engine::campaign::{Campaign, CampaignResult};
pub use crate::engine::experience::experiment::{DomainType, Experiment, ParameterValue};
pub use crate::engine::experience::runner::{ExperimentRunner, RunOutput};
pub use crate::engine::pipeline::flow::{
    Pipeline, filter_positive_stage, normalize_stage, scale_stage,
};
pub use crate::engine::query::{Catalog, FunctionInfo};
pub use crate::engine::simulation::integrator::{
    IntegrationMethod, IntegratorConfig, integrate,
};
pub use crate::engine::simulation::model::{
    DynamicalSystem, HarmonicOscillator, LotkaVolterra, SimpleModel,
};
pub use crate::engine::simulation::result::SimulationResult;

pub use crate::tools::arena::{Arena, ArenaMatrix, ArenaSlice, ScratchPool};
pub use crate::tools::config::Config;
pub use crate::tools::deterministic::{
    ReproducibleContext, Rng, fingerprint, kahan_dot, kahan_sum,
};
pub use crate::tools::logger::Level;
pub use crate::tools::profiler::{
    ProfileEntry, ProfileReport, profile_batch, profile_experiment, quick_profile,
};
pub use crate::tools::utils::{
    approx_equal, format_scientific, format_si, linspace, logspace, relative_error,
};
pub use crate::tools::validation::{
    MonotonicityCheck, MonotonicityResult, NanSafetyCheck, NanSafetyResult, PipelineOutcome,
    ValidationCase, ValidationPipeline, ValidationReport, ValidationThresholds, default_cases,
    default_monotonicity_checks, default_nan_safety_checks, run_validation,
};
pub use crate::tools::visualization::{
    ChartConfig, Series, bar_chart, heatmap, histogram, line_chart, scatter_plot,
};

fn run_output_to_scalar(output: &crate::engine::experience::runner::RunOutput) -> Option<f64> {
    use crate::engine::experience::runner::RunOutput;
    match output {
        RunOutput::Scalar(v) => Some(*v),
        RunOutput::Integer(v) => Some(*v as f64),
        RunOutput::Boolean(v) => Some(if *v { 1.0 } else { 0.0 }),
        RunOutput::Pair(a, b) => Some((a + b) * 0.5),
        RunOutput::Vector(v) => {
            if v.is_empty() {
                None
            } else {
                Some(v.iter().sum::<f64>() / v.len() as f64)
            }
        }
        _ => None,
    }
}

fn scalar_in_domain(
    domain: crate::engine::experience::experiment::DomainType,
    function: &str,
    names: &[&str],
    values: &[f64],
) -> Option<f64> {
    use crate::engine::experience::experiment::{Experiment, ParameterValue};
    use crate::engine::experience::runner::ExperimentRunner;
    if names.len() != values.len() {
        return None;
    }
    let mut experiment = Experiment::new(domain, function);
    for (name, value) in names.iter().zip(values.iter()) {
        experiment = experiment.param(name, ParameterValue::Scalar(*value));
    }
    let output = ExperimentRunner::new().run(&experiment).ok()?;
    run_output_to_scalar(&output)
}

pub fn physics_scalar(function: &str, names: &[&str], values: &[f64]) -> Option<f64> {
    scalar_in_domain(crate::engine::experience::experiment::DomainType::Physics, function, names, values)
}
