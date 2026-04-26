//! Ergonomic re-exports of the most commonly used Hub types and functions.
//!
//! Importing `use sciforge::hub::prelude::*;` gives access to requests,
//! responses, domain modules, engine primitives, and utility helpers
//! without deep path qualifications.

pub use crate::hub::api::dto::request::ComputeRequest;
pub use crate::hub::api::dto::response::ComputeResponse;

pub use crate::hub::domain::astronomy;
pub use crate::hub::domain::biology;
pub use crate::hub::domain::chemistry;
pub use crate::hub::domain::geology;
pub use crate::hub::domain::maths;
pub use crate::hub::domain::meteorology;
pub use crate::hub::domain::physics;

pub use crate::hub::domain::common::constants;
pub use crate::hub::domain::common::errors::{HubError, HubResult};
pub use crate::hub::domain::common::units::{
    AngleUnit, EnergyUnit, LengthUnit, MassUnit, PressureUnit, TemperatureUnit, TimeUnit,
    angle_to_radian, convert_length, convert_mass, convert_time, energy_from_si, energy_to_si,
    kelvin_to_temperature, length_from_si, length_to_si, mass_from_si, mass_to_si,
    pressure_from_si, pressure_to_si, radian_to_angle, temperature_to_kelvin, time_from_si,
    time_to_si,
};

pub use crate::hub::engine::campaign::{Campaign, CampaignResult};
pub use crate::hub::engine::experience::experiment::{DomainType, Experiment, ParameterValue};
pub use crate::hub::engine::experience::runner::{ExperimentRunner, RunOutput};
pub use crate::hub::engine::pipeline::flow::{
    Pipeline, filter_positive_stage, normalize_stage, scale_stage,
};
pub use crate::hub::engine::query::{Catalog, FunctionInfo};
pub use crate::hub::engine::simulation::integrator::{
    IntegrationMethod, IntegratorConfig, integrate,
};
pub use crate::hub::engine::simulation::model::{
    DynamicalSystem, HarmonicOscillator, LotkaVolterra, SimpleModel,
};
pub use crate::hub::engine::simulation::result::SimulationResult;

pub use crate::hub::tools::arena::{Arena, ArenaMatrix, ArenaSlice, ScratchPool};
pub use crate::hub::tools::config::Config;
pub use crate::hub::tools::deterministic::{
    ReproducibleContext, Rng, fingerprint, kahan_dot, kahan_sum,
};
pub use crate::hub::tools::logger::Level;
pub use crate::hub::tools::profiler::{
    ProfileEntry, ProfileReport, profile_batch, profile_experiment, quick_profile,
};
pub use crate::hub::tools::utils::{
    approx_equal, format_scientific, format_si, linspace, logspace, relative_error,
};
pub use crate::hub::tools::validation::{
    MonotonicityCheck, MonotonicityResult, NanSafetyCheck, NanSafetyResult, PipelineOutcome,
    ValidationCase, ValidationPipeline, ValidationReport, ValidationThresholds, default_cases,
    default_monotonicity_checks, default_nan_safety_checks, run_validation,
};
pub use crate::hub::tools::visualization::{
    ChartConfig, Series, bar_chart, heatmap, histogram, line_chart, scatter_plot,
};
