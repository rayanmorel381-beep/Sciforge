pub mod atmosphere;
pub mod combustion;
pub mod fuel;
pub mod inert;
pub mod industrial;
pub mod refrigerant;

pub use atmosphere::*;
pub use combustion::*;
pub use fuel::*;
pub use inert::*;
pub use industrial::*;
pub use refrigerant::*;

use sciforge_hub::prelude::constants::{P_STD, R_GAS, T_STD};
use sciforge_hub::prelude::constants::chemistry::molecules;
use sciforge_hub::prelude::constants::chemistry::thermo::heat_capacity;
use sciforge_hub::prelude::constants::physics::transport::sutherland;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GasFamily {
    Atmosphere,
    Inert,
    Combustion,
    Fuel,
    Process,
    Refrigerant,
    Industrial,
}

#[derive(Debug, Clone, Copy)]
pub struct Gas {
    pub name: &'static str,
    pub formula: &'static str,
    pub family: GasFamily,
    pub molar_mass_kg_per_mol: f64,
    pub cp_j_kgk_ref: f64,
    pub cv_j_kgk_ref: f64,
    pub density_kg_m3_ref: f64,
    pub temperature_ref_k: f64,
    pub pressure_ref_pa: f64,
    pub sutherland_ref_viscosity_pa_s: f64,
    pub sutherland_ref_temp_k: f64,
    pub sutherland_constant_k: f64,
}

impl Gas {
    pub fn from_lib(formula: &'static str, family: GasFamily) -> Gas {
        let mol = molecules::by_formula(formula)
            .unwrap_or_else(|| panic!("molecule '{formula}' not found in sciforge_lib"));
        let hc = heat_capacity::by_formula_phase(formula, "gas")
            .unwrap_or_else(|| panic!("gas heat capacity for '{formula}' not found in sciforge_lib"));
        let suth = sutherland::by_formula(formula)
            .unwrap_or_else(|| panic!("Sutherland coefficients for '{formula}' not found in sciforge_lib"));
        let molar_mass_kg_per_mol = mol.molar_mass / 1000.0;
        let cp_j_kgk_ref = hc.cp_j_molk_298 / molar_mass_kg_per_mol;
        let cv_j_kgk_ref = hc.cv_j_molk_298 / molar_mass_kg_per_mol;
        let density_kg_m3_ref =
            P_STD * molar_mass_kg_per_mol / (R_GAS * T_STD);
        Gas {
            name: mol.name,
            formula: mol.formula,
            family,
            molar_mass_kg_per_mol,
            cp_j_kgk_ref,
            cv_j_kgk_ref,
            density_kg_m3_ref,
            temperature_ref_k: T_STD,
            pressure_ref_pa: P_STD,
            sutherland_ref_viscosity_pa_s: suth.mu_ref_pa_s,
            sutherland_ref_temp_k: suth.t_ref_k,
            sutherland_constant_k: suth.s_k,
        }
    }

    pub fn specific_gas_constant_j_kgk(&self) -> f64 {
        R_GAS / self.molar_mass_kg_per_mol
    }

    pub fn gamma(&self) -> f64 {
        self.cp_j_kgk_ref / self.cv_j_kgk_ref
    }

    pub fn density_ideal_kg_m3(&self, pressure_pa: f64, temperature_k: f64) -> f64 {
        pressure_pa / (self.specific_gas_constant_j_kgk() * temperature_k)
    }

    pub fn speed_of_sound_m_s(&self, temperature_k: f64) -> f64 {
        (self.gamma() * self.specific_gas_constant_j_kgk() * temperature_k).sqrt()
    }

    pub fn dynamic_viscosity_pa_s(&self, temperature_k: f64) -> f64 {
        let t = temperature_k;
        let t0 = self.sutherland_ref_temp_k;
        let c = self.sutherland_constant_k;
        self.sutherland_ref_viscosity_pa_s * (t / t0).powf(1.5) * (t0 + c) / (t + c)
    }

    pub fn reynolds_number(&self, velocity_m_s: f64, length_m: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
        let rho = self.density_ideal_kg_m3(pressure_pa, temperature_k);
        let mu = self.dynamic_viscosity_pa_s(temperature_k);
        rho * velocity_m_s * length_m / mu
    }
}

pub fn all_gases() -> Vec<&'static Gas> {
    let mut v = Vec::new();
    v.extend(atmosphere::all_atmosphere_gases());
    v.extend(inert::all_inert_gases());
    v.extend(combustion::all_combustion_gases());
    v.extend(fuel::all_fuel_gases());
    v.extend(refrigerant::all_refrigerant_gases());
    v.extend(industrial::all_industrial_gases());
    v
}

pub fn gas_by_name(name: &str) -> Option<&'static Gas> {
    all_gases().into_iter().find(|gas| gas.name.eq_ignore_ascii_case(name))
}
