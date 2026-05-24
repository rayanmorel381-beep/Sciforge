//! Domain dispatch layer.
//!
//! Routes a `(domain, function, params)` triplet to the matching
//! scientific function and returns a [`RunOutput`].

#[cfg(feature = "astronomy")]
pub mod astronomy;
#[cfg(feature = "biology")]
pub mod biology;
#[cfg(feature = "chemistry")]
pub mod chemistry;
#[cfg(feature = "cross_domain")]
pub mod crossdomain;
#[cfg(feature = "geology")]
pub mod geology;
#[cfg(feature = "maths")]
pub mod maths;
#[cfg(feature = "meteorology")]
pub mod meteorology;
pub mod params;
#[cfg(feature = "physics")]
pub mod physics;

use crate::domain::common::errors::HubResult;
#[cfg(not(all(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
    feature = "cross_domain",
)))]
use crate::domain::common::errors::HubError;
#[cfg(any(
    feature = "maths",
    feature = "physics",
    feature = "chemistry",
    feature = "biology",
    feature = "astronomy",
    feature = "geology",
    feature = "meteorology",
    feature = "cross_domain",
))]
use crate::engine::experience::experiment::DomainType;
use crate::engine::experience::experiment::Experiment;
use crate::engine::experience::runner::RunOutput;

/// Routes an [`Experiment`] to the appropriate domain dispatcher based on its [`DomainType`].
pub fn dispatch(experiment: &Experiment) -> HubResult<RunOutput> {
    match &experiment.domain {
        #[cfg(feature = "maths")]
        DomainType::Maths => maths::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "physics")]
        DomainType::Physics => physics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "chemistry")]
        DomainType::Chemistry => chemistry::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "biology")]
        DomainType::Biology => biology::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "astronomy")]
        DomainType::Astronomy => astronomy::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "geology")]
        DomainType::Geology => geology::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "meteorology")]
        DomainType::Meteorology => meteorology::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Astrochemistry => crossdomain::astrochemistry::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Geophysics => crossdomain::geophysics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Astrophysics => crossdomain::astrophysics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Biochemistry => crossdomain::biochemistry::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Biophysics => crossdomain::biophysics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Geochemistry => crossdomain::geochemistry::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Astrobiology => crossdomain::astrobiology::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::AtmosphericChemistry => crossdomain::atmospheric_chemistry::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::AtmosphericPhysics => crossdomain::atmospheric_physics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::PlanetaryGeology => crossdomain::planetary_geology::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::Biomathematics => crossdomain::biomathematics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(feature = "cross_domain")]
        DomainType::MathematicalPhysics => crossdomain::mathematical_physics::dispatch(&experiment.function_name, &experiment.parameters),
        #[cfg(not(all(
            feature = "maths",
            feature = "physics",
            feature = "chemistry",
            feature = "biology",
            feature = "astronomy",
            feature = "geology",
            feature = "meteorology",
            feature = "cross_domain",
        )))]
        other => Err(HubError::InvalidInput(format!(
            "domain {other:?} not enabled (missing feature)"
        ))),
    }
}
