pub mod coolants;
pub mod fuels;
pub mod hydraulics;
pub mod oils;
pub mod water;

pub use coolants::*;
pub use fuels::*;
pub use hydraulics::*;
pub use oils::*;
pub use water::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidFamily {
    Water,
    Oil,
    Coolant,
    Fuel,
    Hydraulic,
}

#[derive(Debug, Clone, Copy)]
pub struct Liquid {
    pub name: &'static str,
    pub formula: &'static str,
    pub family: LiquidFamily,
    pub density_kg_m3_ref: f64,
    pub dynamic_viscosity_pa_s_ref: f64,
    pub bulk_modulus_pa: f64,
    pub specific_heat_j_kgk: f64,
    pub thermal_conductivity_w_mk: f64,
    pub surface_tension_n_m: f64,
    pub vapor_pressure_pa_ref: f64,
    pub temperature_ref_k: f64,
    pub viscosity_index: f64,
    pub pour_point_k: f64,
    pub flash_point_k: f64,
    pub shear_stability_index: f64,
    pub friction_coefficient: f64,
}

impl Liquid {
    pub fn kinematic_viscosity_m2_s(&self) -> f64 {
        self.dynamic_viscosity_pa_s_ref / self.density_kg_m3_ref
    }

    pub fn speed_of_sound_m_s(&self) -> f64 {
        (self.bulk_modulus_pa / self.density_kg_m3_ref).sqrt()
    }

    pub fn reynolds_number(&self, velocity_m_s: f64, length_m: f64) -> f64 {
        self.density_kg_m3_ref * velocity_m_s * length_m / self.dynamic_viscosity_pa_s_ref
    }

    pub fn prandtl_number(&self) -> f64 {
        self.dynamic_viscosity_pa_s_ref * self.specific_heat_j_kgk / self.thermal_conductivity_w_mk
    }
}

pub fn all_liquids() -> Vec<&'static Liquid> {
    let mut liquids = Vec::new();
    liquids.extend(water::all_water());
    liquids.extend(coolants::all_coolants());
    liquids.extend(oils::all_oils());
    liquids.extend(hydraulics::all_hydraulics());
    liquids.extend(fuels::all_fuels());
    liquids
}

pub fn liquid_by_name(name: &str) -> Option<&'static Liquid> {
    all_liquids()
        .into_iter()
        .find(|liquid| liquid.name.eq_ignore_ascii_case(name))
}
