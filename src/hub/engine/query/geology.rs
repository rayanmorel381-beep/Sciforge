//! Catalog entries for geology functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Geology;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Geology,
        "p_wave_velocity",
        &["k", "mu", "rho"],
        "P-wave velocity from elastic moduli",
    );
    reg(
        e,
        Geology,
        "s_wave_velocity",
        &["mu", "rho"],
        "S-wave velocity",
    );
    reg(
        e,
        Geology,
        "richter_magnitude",
        &["amplitude", "distance"],
        "Richter local magnitude",
    );
    reg(
        e,
        Geology,
        "moment_magnitude",
        &["seismic_moment"],
        "Moment magnitude from seismic moment",
    );
    reg(
        e,
        Geology,
        "seismic_moment",
        &["mu", "area", "displacement"],
        "Seismic moment M₀",
    );
    reg(
        e,
        Geology,
        "epicenter_distance",
        &["tp", "ts", "vp", "vs"],
        "Epicenter distance from P/S arrival times",
    );
    reg(
        e,
        Geology,
        "radioactive_decay",
        &["n0", "lambda", "t"],
        "Radioactive decay N(t)",
    );
    reg(
        e,
        Geology,
        "half_life_to_lambda",
        &["half_life"],
        "Decay constant from half-life",
    );
    reg(
        e,
        Geology,
        "age_from_ratio",
        &["ratio", "lambda"],
        "Radiometric age from parent/daughter ratio",
    );
    reg(
        e,
        Geology,
        "mohs_to_vickers",
        &["mohs"],
        "Mohs to Vickers hardness",
    );
    reg(
        e,
        Geology,
        "plate_velocity",
        &["distance", "time"],
        "Tectonic plate velocity",
    );
    reg(
        e,
        Geology,
        "isostatic_depth",
        &["h_crust", "rho_crust", "rho_mantle"],
        "Isostatic root depth",
    );
}
