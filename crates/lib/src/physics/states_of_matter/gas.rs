use crate::constants::R_GAS;
use crate::constants::chemistry::molecules::Molecule;
use crate::constants::chemistry::thermo::heat_capacity::{self, HeatCapacity};
use crate::constants::physics::transport::sutherland::{self, SutherlandCoeffs};
use crate::constants::physics::{P_STD, T_STD};

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

    pub fn reynolds_number(
        &self,
        velocity_m_s: f64,
        length_m: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let rho = self.density_ideal_kg_m3(pressure_pa, temperature_k);
        let mu = self.dynamic_viscosity_pa_s(temperature_k);
        rho * velocity_m_s * length_m / mu
    }

    pub fn from_lib_data(
        molecule: &Molecule,
        cp: &HeatCapacity,
        sutherland: &SutherlandCoeffs,
        family: GasFamily,
    ) -> Self {
        let molar_mass_kg = molecule.molar_mass * 1.0e-3;
        let cp_mass = cp.cp_j_molk_298 / molar_mass_kg;
        let cv_mass = cp.cv_j_molk_298 / molar_mass_kg;
        let t_ref = T_STD;
        let p_ref = P_STD;
        let r_specific = R_GAS / molar_mass_kg;
        let density = p_ref / (r_specific * t_ref);
        Gas {
            name: molecule.name,
            formula: molecule.formula,
            family,
            molar_mass_kg_per_mol: molar_mass_kg,
            cp_j_kgk_ref: cp_mass,
            cv_j_kgk_ref: cv_mass,
            density_kg_m3_ref: density,
            temperature_ref_k: t_ref,
            pressure_ref_pa: p_ref,
            sutherland_ref_viscosity_pa_s: sutherland.mu_ref_pa_s,
            sutherland_ref_temp_k: sutherland.t_ref_k,
            sutherland_constant_k: sutherland.s_k,
        }
    }

    pub fn from_molecule(molecule: &Molecule, family: GasFamily) -> Option<Self> {
        let cp = heat_capacity::by_formula_phase(molecule.formula, "gas")?;
        let suth = sutherland::by_formula(molecule.formula)?;
        Some(Self::from_lib_data(molecule, cp, suth, family))
    }
}

fn preset(formula: &str, family: GasFamily) -> Gas {
    let mol = crate::constants::chemistry::molecules::by_formula(formula)
        .expect("preset molecule must exist in molecules table");
    Gas::from_molecule(mol, family).expect("preset must have heat_capacity + sutherland entries")
}

pub fn dry_air() -> Gas {
    preset("AIR", GasFamily::Atmosphere)
}

pub fn r134a() -> Gas {
    preset("CH2FCF3", GasFamily::Refrigerant)
}

pub fn r32() -> Gas {
    preset("CH2F2", GasFamily::Refrigerant)
}

pub fn r125() -> Gas {
    preset("C2HF5", GasFamily::Refrigerant)
}

pub fn r1234yf() -> Gas {
    preset("C3H2F4", GasFamily::Refrigerant)
}

pub fn r410a() -> Gas {
    preset("R410A", GasFamily::Refrigerant)
}
