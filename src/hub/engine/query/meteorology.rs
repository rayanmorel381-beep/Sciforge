//! Catalog entries for meteorology functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Meteorology;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Meteorology,
        "barometric_formula",
        &["p0", "h", "t", "m", "g"],
        "Barometric height formula",
    );
    reg(
        e,
        Meteorology,
        "scale_height",
        &["t", "m", "g"],
        "Atmospheric scale height",
    );
    reg(
        e,
        Meteorology,
        "lapse_rate_dry",
        &["g", "cp"],
        "Dry adiabatic lapse rate",
    );
    reg(
        e,
        Meteorology,
        "potential_temperature",
        &["t", "p", "p0"],
        "Potential temperature θ",
    );
    reg(
        e,
        Meteorology,
        "saturation_vapor_pressure",
        &["t"],
        "Saturation vapor pressure (Magnus)",
    );
    reg(
        e,
        Meteorology,
        "relative_humidity",
        &["t", "t_d"],
        "Relative humidity from temperature and dew point",
    );
    reg(
        e,
        Meteorology,
        "dew_point",
        &["t", "rh"],
        "Dew point temperature",
    );
    reg(
        e,
        Meteorology,
        "wind_chill",
        &["t", "v"],
        "Wind chill index",
    );
    reg(e, Meteorology, "heat_index", &["t", "rh"], "Heat index");
    reg(
        e,
        Meteorology,
        "coriolis_parameter",
        &["latitude"],
        "Coriolis parameter f",
    );
    reg(
        e,
        Meteorology,
        "stefan_boltzmann_flux",
        &["t"],
        "Stefan-Boltzmann radiative flux",
    );
    reg(
        e,
        Meteorology,
        "albedo_flux",
        &["solar", "albedo"],
        "Reflected flux",
    );
}
