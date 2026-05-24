use super::nuclide::binding_energy_per_nucleon_semf;
use super::processes::ProcessType;
use crate::constants::{
    C, ELECTRON_MASS_KG, G, H, K_B, PROTON_MASS_KG, SIGMA_SB, SOLAR_LUMINOSITY, SOLAR_MASS,
    SOLAR_RADIUS, TROPICAL_YEAR,
};

pub struct StellarCore {
    pub temperature_k: f64,
    pub density_kg_m3: f64,
    pub hydrogen_fraction: f64,
    pub helium_fraction: f64,
    pub metal_fraction: f64,
    pub mass_solar: f64,
}

impl StellarCore {
    pub fn new(mass_solar: f64, temperature_k: f64, density_kg_m3: f64) -> Self {
        Self {
            temperature_k,
            density_kg_m3,
            hydrogen_fraction: 0.70,
            helium_fraction: 0.28,
            metal_fraction: 0.02,
            mass_solar,
        }
    }

    pub fn luminosity_solar(&self) -> f64 {
        self.mass_solar.powf(3.5)
    }

    pub fn main_sequence_lifetime_years(&self) -> f64 {
        1e10 / self.mass_solar.powf(2.5)
    }

    pub fn active_processes(&self) -> Vec<ProcessType> {
        super::processes::active_processes(self.temperature_k)
    }

    pub fn dominant_process(&self) -> Option<ProcessType> {
        if self.temperature_k >= 2.7e9 {
            Some(ProcessType::SiliconBurning)
        } else if self.temperature_k >= 1.5e9 {
            Some(ProcessType::OxygenBurning)
        } else if self.temperature_k >= 1.2e9 {
            Some(ProcessType::NeonBurning)
        } else if self.temperature_k >= 5e8 {
            Some(ProcessType::CarbonBurning)
        } else if self.temperature_k >= 1e8 {
            Some(ProcessType::TripleAlpha)
        } else if self.temperature_k >= 15e6 {
            Some(ProcessType::CNOCycle)
        } else if self.temperature_k >= 4e6 {
            Some(ProcessType::PPChain)
        } else {
            None
        }
    }

    pub fn evolve_step(&mut self, dt_years: f64) {
        let rate = match self.dominant_process() {
            Some(ProcessType::PPChain) => 1e-11,
            Some(ProcessType::CNOCycle) => 5e-11,
            Some(ProcessType::TripleAlpha) => 1e-9,
            Some(ProcessType::CarbonBurning) => 1e-7,
            _ => 0.0,
        };
        let dh = rate * dt_years;
        if self.hydrogen_fraction > dh {
            self.hydrogen_fraction -= dh;
            self.helium_fraction += dh * 0.99;
            self.metal_fraction += dh * 0.01;
        } else if self.helium_fraction > dh {
            self.helium_fraction -= dh;
            self.metal_fraction += dh;
            self.temperature_k *= 1.0 + 1e-12 * dt_years;
        }
    }
}

pub fn chandrasekhar_limit() -> f64 {
    1.4
}
pub fn tolman_oppenheimer_volkoff_limit() -> f64 {
    2.17
}
pub fn neutron_drip_density() -> f64 {
    4e14
}

pub fn iron_peak_binding_energy() -> f64 {
    binding_energy_per_nucleon_semf(26, 56)
}

pub fn eddington_luminosity_solar(mass_solar: f64) -> f64 {
    3.2e4 * mass_solar
}

pub fn kelvin_helmholtz_timescale_years(
    mass_solar: f64,
    radius_solar: f64,
    luminosity_solar: f64,
) -> f64 {
    let m = mass_solar * SOLAR_MASS;
    let r = radius_solar * SOLAR_RADIUS;
    let l = luminosity_solar * SOLAR_LUMINOSITY;
    G * m * m / (2.0 * r * l) / (365.25 * 86400.0)
}

pub fn jeans_mass_solar(temperature_k: f64, density_kg_m3: f64) -> f64 {
    let cs = (K_B * temperature_k / PROTON_MASS_KG).sqrt();
    let mj = (std::f64::consts::PI / 6.0) * cs.powi(3) / (G.powf(1.5) * density_kg_m3.sqrt());
    mj / SOLAR_MASS
}

pub fn schwarzschild_radius_km(mass_solar: f64) -> f64 {
    2.953 * mass_solar
}

pub fn nuclear_timescale_years(mass_solar: f64, luminosity_solar: f64, efficiency: f64) -> f64 {
    let e = efficiency * mass_solar * SOLAR_MASS * C * C;
    let l = luminosity_solar * SOLAR_LUMINOSITY;
    e / l / TROPICAL_YEAR
}

pub fn core_collapse_min_mass_solar() -> f64 {
    8.0
}

pub fn white_dwarf_radius_km(mass_solar: f64) -> f64 {
    let r_earth_km = 6371.0;
    r_earth_km * (chandrasekhar_limit() / mass_solar).powf(1.0 / 3.0)
}

pub fn electron_degeneracy_pressure(density_kg_m3: f64) -> f64 {
    let n_e = density_kg_m3 / PROTON_MASS_KG;
    (H * H / (5.0 * ELECTRON_MASS_KG))
        * (3.0 / (8.0 * std::f64::consts::PI)).powf(2.0 / 3.0)
        * n_e.powf(5.0 / 3.0)
}

pub fn neutron_star_radius_km(mass_solar: f64) -> f64 {
    10.0 * (1.4 / mass_solar).powf(1.0 / 3.0)
}

pub fn luminosity_radius_temperature(radius_solar: f64, temperature_k: f64) -> f64 {
    let r = radius_solar * SOLAR_RADIUS;
    4.0 * std::f64::consts::PI * r * r * SIGMA_SB * temperature_k.powi(4) / SOLAR_LUMINOSITY
}

pub fn stellar_wind_mass_loss(luminosity_solar: f64, escape_velocity_km_s: f64) -> f64 {
    let l = luminosity_solar * SOLAR_LUMINOSITY;
    let v = escape_velocity_km_s * 1e3;
    let m_sun_per_yr = SOLAR_MASS / TROPICAL_YEAR;
    l / (v * v) / m_sun_per_yr
}
