#[derive(Debug, Clone, Copy)]
pub struct LaserMedium {
    pub formula: &'static str,
    pub wavelength_nm: f64,
    pub emission_cross_section_m2: f64,
    pub upper_state_lifetime_s: f64,
    pub host: &'static str,
}

pub const TABLE: &[LaserMedium] = &[
    LaserMedium { formula: "Nd:YAG",     wavelength_nm: 1064.0, emission_cross_section_m2: 2.8e-23, upper_state_lifetime_s: 230e-6,  host: "YAG" },
    LaserMedium { formula: "Nd:YVO4",    wavelength_nm: 1064.0, emission_cross_section_m2: 1.14e-22,upper_state_lifetime_s: 100e-6,  host: "YVO4" },
    LaserMedium { formula: "Nd:Glass",   wavelength_nm: 1054.0, emission_cross_section_m2: 4.0e-24, upper_state_lifetime_s: 350e-6,  host: "phosphate_glass" },
    LaserMedium { formula: "Yb:YAG",     wavelength_nm: 1030.0, emission_cross_section_m2: 2.1e-24, upper_state_lifetime_s: 951e-6,  host: "YAG" },
    LaserMedium { formula: "Er:YAG",     wavelength_nm: 2940.0, emission_cross_section_m2: 8.0e-25, upper_state_lifetime_s: 100e-3,  host: "YAG" },
    LaserMedium { formula: "Er:Glass",   wavelength_nm: 1550.0, emission_cross_section_m2: 8.0e-25, upper_state_lifetime_s: 8e-3,    host: "silica" },
    LaserMedium { formula: "Ti:Sapphire",wavelength_nm: 800.0,  emission_cross_section_m2: 4.1e-23, upper_state_lifetime_s: 3.2e-6,  host: "Al2O3" },
    LaserMedium { formula: "Cr:LiSAF",   wavelength_nm: 850.0,  emission_cross_section_m2: 4.8e-24, upper_state_lifetime_s: 67e-6,   host: "LiSAF" },
    LaserMedium { formula: "Ho:YAG",     wavelength_nm: 2090.0, emission_cross_section_m2: 9.0e-25, upper_state_lifetime_s: 7.4e-3,  host: "YAG" },
    LaserMedium { formula: "Tm:YAG",     wavelength_nm: 2010.0, emission_cross_section_m2: 1.5e-25, upper_state_lifetime_s: 12e-3,   host: "YAG" },
    LaserMedium { formula: "HeNe",       wavelength_nm: 632.8,  emission_cross_section_m2: 3.0e-17, upper_state_lifetime_s: 30e-9,   host: "gas_HeNe" },
    LaserMedium { formula: "CO2",        wavelength_nm: 10600.0,emission_cross_section_m2: 1.5e-22, upper_state_lifetime_s: 4e-3,    host: "gas_CO2" },
    LaserMedium { formula: "Ar+",        wavelength_nm: 514.5,  emission_cross_section_m2: 2.5e-16, upper_state_lifetime_s: 8e-9,    host: "gas_Ar" },
    LaserMedium { formula: "GaAs",       wavelength_nm: 850.0,  emission_cross_section_m2: 4.0e-20, upper_state_lifetime_s: 1e-9,    host: "GaAs_diode" },
    LaserMedium { formula: "InGaAs",     wavelength_nm: 980.0,  emission_cross_section_m2: 5.0e-20, upper_state_lifetime_s: 1e-9,    host: "InGaAs_diode" },
];

pub fn by_formula(formula: &str) -> Option<&'static LaserMedium> {
    TABLE.iter().find(|l| l.formula == formula)
}
