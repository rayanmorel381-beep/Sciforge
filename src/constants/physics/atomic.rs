pub const PROTON_MASS_KG: f64 = 1.672_621_923_69e-27;
pub const NEUTRON_MASS_KG: f64 = 1.674_927_498_04e-27;
pub const ELECTRON_MASS_KG: f64 = 9.109_383_701_5e-31;
pub const PROTON_MASS_AMU: f64 = 1.007_276_466_621;
pub const NEUTRON_MASS_AMU: f64 = 1.008_664_915_95;
pub const ELECTRON_MASS_AMU: f64 = 0.000_548_579_909_065;
pub const AMU_TO_KG: f64 = 1.660_539_066_60e-27;
pub const AMU_TO_MEV: f64 = 931.494_102_42;
pub const ELECTRON_REST_MASS_MEV: f64 =
    ELECTRON_MASS_KG * super::fundamental::C * super::fundamental::C
        / (super::units::EV_TO_JOULE * 1e6);
pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11;
pub const RYDBERG_ENERGY: f64 = 13.605_693_122_994;
pub const BOHR_MAGNETON: f64 = 9.274_010_078_3e-24;
pub const NUCLEAR_MAGNETON: f64 = 5.050_783_746_1e-27;
pub const COMPTON_WAVELENGTH: f64 =
    super::fundamental::H / (ELECTRON_MASS_KG * super::fundamental::C);
pub const MUON_MASS: f64 = 1.883_531_627e-28;
pub const TAU_MASS: f64 = 3.167_47e-27;
pub const NEUTRINO_MASS_UPPER: f64 = 2.2e-36;
pub const R_INF: f64 = 10_973_731.568_16;
