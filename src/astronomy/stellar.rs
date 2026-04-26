use crate::constants::{
    ADIABATIC_INDEX_IDEAL, ADIABATIC_INDEX_RADIATION, AMU_TO_KG, BOLOMETRIC_CORRECTION_CENTER,
    BOLOMETRIC_CORRECTION_OFFSET, BOLOMETRIC_CORRECTION_QUAD, C, CHANDRASEKHAR_LIMIT_SOLAR,
    CNO_CYCLE_ENERGY, G, H_BURNING_EFFICIENCY, HELIUM_FRACTION_SOLAR, HELIUM4_MASS,
    HYDROGEN_FRACTION_SOLAR, K_B, MAGNETAR_B_TYPICAL, MAIN_SEQUENCE_LIFETIME_SCALE_YR,
    METAL_FRACTION_SOLAR, ML_HIGH_COEFF, ML_HIGH_EXPONENT, ML_HIGH_THRESHOLD, ML_LOW_COEFF,
    ML_LOW_EXPONENT, ML_LOW_THRESHOLD, ML_MID_EXPONENT, ML_MID_THRESHOLD, ML_VERY_HIGH_COEFF, MU_0,
    NS_MASS_TYPICAL, NS_RADIUS_TYPICAL, OPACITY_ELECTRON_SCATTERING, PP_CHAIN_ENERGY,
    PROTON_MASS_KG, PULSAR_B_TYPICAL, SIGMA_SB, SOLAR_ABS_MAGNITUDE, SOLAR_MASS, SOLAR_METALLICITY,
    SOLAR_MS_LIFETIME, SOLAR_TEFF, SPECTRAL_INDEX_OFFSET, SPECTRAL_TEMP_SCALE, TOV_LIMIT,
    WD_RADIUS_TYPICAL, WIEN_DISPLACEMENT,
};

pub fn luminosity_from_radius_temp(r: f64, t: f64) -> f64 {
    4.0 * std::f64::consts::PI * r * r * SIGMA_SB * t.powi(4)
}

pub fn absolute_magnitude(apparent_mag: f64, distance_pc: f64) -> f64 {
    apparent_mag - 5.0 * (distance_pc / 10.0).log10()
}

pub fn distance_modulus(apparent_mag: f64, absolute_mag: f64) -> f64 {
    10.0_f64.powf((apparent_mag - absolute_mag + 5.0) / 5.0)
}

pub fn stellar_flux(luminosity: f64, distance: f64) -> f64 {
    luminosity / (4.0 * std::f64::consts::PI * distance * distance)
}

pub fn wien_peak_wavelength(temperature: f64) -> f64 {
    WIEN_DISPLACEMENT / temperature
}

pub fn main_sequence_lifetime(mass_solar: f64, luminosity_solar: f64) -> f64 {
    MAIN_SEQUENCE_LIFETIME_SCALE_YR * mass_solar / luminosity_solar
}

pub fn mass_luminosity_relation(mass_solar: f64) -> f64 {
    if mass_solar < ML_LOW_THRESHOLD {
        ML_LOW_COEFF * mass_solar.powf(ML_LOW_EXPONENT)
    } else if mass_solar < ML_MID_THRESHOLD {
        mass_solar.powf(ML_MID_EXPONENT)
    } else if mass_solar < ML_HIGH_THRESHOLD {
        ML_HIGH_COEFF * mass_solar.powf(ML_HIGH_EXPONENT)
    } else {
        ML_VERY_HIGH_COEFF * mass_solar
    }
}

pub fn schwarzschild_radius(mass: f64) -> f64 {
    2.0 * G * mass / (C * C)
}

pub fn chandrasekhar_limit() -> f64 {
    CHANDRASEKHAR_LIMIT_SOLAR * SOLAR_MASS
}

pub fn chandrasekhar_limit_kg() -> f64 {
    crate::constants::CHANDRASEKHAR_LIMIT
}

pub fn is_above_chandrasekhar(mass_kg: f64) -> bool {
    mass_kg > crate::constants::CHANDRASEKHAR_LIMIT
}

pub fn eddington_luminosity(mass: f64) -> f64 {
    4.0 * std::f64::consts::PI * G * mass * C / OPACITY_ELECTRON_SCATTERING
}

pub fn spectral_type_temperature(spectral_index: f64) -> f64 {
    SPECTRAL_TEMP_SCALE / (spectral_index + SPECTRAL_INDEX_OFFSET)
}

pub fn bolometric_correction(t_eff: f64) -> f64 {
    let log_t = t_eff.log10();
    BOLOMETRIC_CORRECTION_QUAD * (log_t - BOLOMETRIC_CORRECTION_CENTER).powi(2)
        + BOLOMETRIC_CORRECTION_OFFSET
}

pub fn tov_limit() -> f64 {
    TOV_LIMIT
}

pub fn neutron_star_surface_gravity() -> f64 {
    G * NS_MASS_TYPICAL / (NS_RADIUS_TYPICAL * NS_RADIUS_TYPICAL)
}

pub fn neutron_star_mean_density() -> f64 {
    3.0 * NS_MASS_TYPICAL / (4.0 * std::f64::consts::PI * NS_RADIUS_TYPICAL.powi(3))
}

pub fn neutron_star_escape_velocity() -> f64 {
    (2.0 * G * NS_MASS_TYPICAL / NS_RADIUS_TYPICAL).sqrt()
}

pub fn pulsar_spindown_luminosity(period: f64, period_dot: f64) -> f64 {
    let inertia = 0.4 * NS_MASS_TYPICAL * NS_RADIUS_TYPICAL * NS_RADIUS_TYPICAL;
    4.0 * std::f64::consts::PI * std::f64::consts::PI * inertia * period_dot
        / (period * period * period)
}

pub fn pulsar_magnetic_field(period: f64, period_dot: f64) -> f64 {
    let inertia = 0.4 * NS_MASS_TYPICAL * NS_RADIUS_TYPICAL * NS_RADIUS_TYPICAL;
    let r = NS_RADIUS_TYPICAL;
    (3.0 * C * C * C * inertia * period * period_dot
        / (8.0 * std::f64::consts::PI * std::f64::consts::PI * r.powi(6)))
    .sqrt()
}

pub fn pulsar_characteristic_age(period: f64, period_dot: f64) -> f64 {
    period / (2.0 * period_dot)
}

pub fn pulsar_death_line_period(b_field: f64) -> f64 {
    (b_field / PULSAR_B_TYPICAL).sqrt()
}

pub fn magnetar_energy_reservoir(b_field: f64) -> f64 {
    let volume = 4.0 / 3.0 * std::f64::consts::PI * NS_RADIUS_TYPICAL.powi(3);
    b_field * b_field * volume / (2.0 * MU_0)
}

pub fn magnetar_typical_energy() -> f64 {
    let volume = 4.0 / 3.0 * std::f64::consts::PI * NS_RADIUS_TYPICAL.powi(3);
    MAGNETAR_B_TYPICAL * MAGNETAR_B_TYPICAL * volume / (2.0 * MU_0)
}

pub fn radiation_pressure(temperature: f64) -> f64 {
    ADIABATIC_INDEX_RADIATION * SIGMA_SB * temperature.powi(4) / (3.0 * C)
}

pub fn gas_pressure(rho: f64, temperature: f64, mu: f64) -> f64 {
    rho * K_B * temperature / (mu * AMU_TO_KG)
}

pub fn adiabatic_sound_speed(temperature: f64, mu: f64) -> f64 {
    (ADIABATIC_INDEX_IDEAL * K_B * temperature / (mu * AMU_TO_KG)).sqrt()
}

pub fn pp_chain_luminosity(mass_kg: f64, x_hydrogen: f64) -> f64 {
    let n_protons = x_hydrogen * mass_kg / PROTON_MASS_KG;
    let rate_per_proton = H_BURNING_EFFICIENCY * PP_CHAIN_ENERGY;
    n_protons * rate_per_proton / SOLAR_MS_LIFETIME
}

pub fn cno_cycle_luminosity(mass_kg: f64, x_hydrogen: f64, z_metals: f64) -> f64 {
    let n_protons = x_hydrogen * mass_kg / PROTON_MASS_KG;
    let rate_per_proton = H_BURNING_EFFICIENCY * CNO_CYCLE_ENERGY * z_metals / SOLAR_METALLICITY;
    n_protons * rate_per_proton / SOLAR_MS_LIFETIME
}

pub fn kelvin_helmholtz_timescale(mass: f64, radius: f64, luminosity: f64) -> f64 {
    G * mass * mass / (radius * luminosity)
}

pub fn nuclear_timescale(mass: f64, luminosity: f64) -> f64 {
    H_BURNING_EFFICIENCY * HYDROGEN_FRACTION_SOLAR * mass * C * C / luminosity
}

pub fn white_dwarf_radius_from_mass(mass_kg: f64) -> f64 {
    WD_RADIUS_TYPICAL
        * (CHANDRASEKHAR_LIMIT_SOLAR * SOLAR_MASS / mass_kg).powf(1.0 / 3.0)
        * (1.0 - (mass_kg / (CHANDRASEKHAR_LIMIT_SOLAR * SOLAR_MASS)).powf(4.0 / 3.0)).sqrt()
}

pub fn eddington_luminosity_numerical(mass_solar: f64) -> f64 {
    crate::constants::EDDINGTON_PREFACTOR * mass_solar * crate::constants::SOLAR_LUMINOSITY
}

pub fn helium_mass() -> f64 {
    HELIUM4_MASS
}

pub fn solar_composition() -> (f64, f64, f64) {
    (
        HYDROGEN_FRACTION_SOLAR,
        HELIUM_FRACTION_SOLAR,
        METAL_FRACTION_SOLAR,
    )
}

pub fn solar_effective_temperature() -> f64 {
    SOLAR_TEFF
}

pub fn solar_absolute_magnitude() -> f64 {
    SOLAR_ABS_MAGNITUDE
}

pub fn solar_metallicity() -> f64 {
    SOLAR_METALLICITY
}

pub fn solar_main_sequence_lifetime() -> f64 {
    SOLAR_MS_LIFETIME
}

pub fn hydrogen_burning_energy_per_kg() -> f64 {
    H_BURNING_EFFICIENCY * C * C
}

pub fn eddington_ratio(luminosity: f64, mass: f64) -> f64 {
    luminosity * OPACITY_ELECTRON_SCATTERING * C / (4.0 * std::f64::consts::PI * G * mass * C * C)
}

pub fn sun_core_temperature() -> f64 {
    crate::constants::SUN_CORE_TEMPERATURE
}

pub fn sun_surface_temperature() -> f64 {
    crate::constants::SUN_SURFACE_TEMPERATURE
}

pub fn sun_core_density() -> f64 {
    crate::constants::SUN_CORE_DENSITY
}

pub fn sun_age() -> f64 {
    crate::constants::SUN_AGE
}

pub fn sun_rotation_period() -> f64 {
    crate::constants::SUN_ROTATION_PERIOD
}

pub fn solar_density() -> f64 {
    crate::constants::SOLAR_DENSITY
}

pub fn solar_surface_gravity() -> f64 {
    crate::constants::SOLAR_GRAVITY
}

pub fn sun_luminosity_to_mass_ratio() -> f64 {
    crate::constants::SOLAR_LUMINOSITY / SOLAR_MASS
}
