//! Domain dispatch layer.
//!
//! Routes a `(domain, function, params)` triplet to the matching
//! scientific function and returns a [`RunOutput`].

pub mod astronomy;
pub mod biology;
pub mod chemistry;
pub mod crossdomain;
pub mod geology;
pub mod maths;
pub mod meteorology;
pub mod params;
pub mod physics;

use crate::hub::domain::common::errors::HubResult;
use crate::hub::engine::experience::experiment::{DomainType, Experiment};
use crate::hub::engine::experience::runner::RunOutput;

/// Routes an [`Experiment`] to the appropriate domain dispatcher based on its [`DomainType`].
pub fn dispatch(experiment: &Experiment) -> HubResult<RunOutput> {
    let p = &experiment.parameters;
    let func = &experiment.function_name;
    match experiment.domain {
        DomainType::Maths => maths::dispatch(func, p),
        DomainType::Physics => physics::dispatch(func, p),
        DomainType::Chemistry => chemistry::dispatch(func, p),
        DomainType::Biology => biology::dispatch(func, p),
        DomainType::Astronomy => astronomy::dispatch(func, p),
        DomainType::Geology => geology::dispatch(func, p),
        DomainType::Meteorology => meteorology::dispatch(func, p),
        DomainType::Astrochemistry => crossdomain::astrochemistry::dispatch(func, p),
        DomainType::Geophysics => crossdomain::geophysics::dispatch(func, p),
        DomainType::Astrophysics => crossdomain::astrophysics::dispatch(func, p),
        DomainType::Biochemistry => crossdomain::biochemistry::dispatch(func, p),
        DomainType::Biophysics => crossdomain::biophysics::dispatch(func, p),
        DomainType::Geochemistry => crossdomain::geochemistry::dispatch(func, p),
        DomainType::Astrobiology => crossdomain::astrobiology::dispatch(func, p),
        DomainType::AtmosphericChemistry => crossdomain::atmospheric_chemistry::dispatch(func, p),
        DomainType::AtmosphericPhysics => crossdomain::atmospheric_physics::dispatch(func, p),
        DomainType::PlanetaryGeology => crossdomain::planetary_geology::dispatch(func, p),
        DomainType::Biomathematics => crossdomain::biomathematics::dispatch(func, p),
        DomainType::MathematicalPhysics => crossdomain::mathematical_physics::dispatch(func, p),
    }
}
