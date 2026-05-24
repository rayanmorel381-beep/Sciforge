use sciforge_hub::prelude::constants::elements::Element;

pub mod alus;
pub mod ceramics;
pub mod composites;
pub mod coppers;
pub mod glasses;
pub mod irons;
pub mod magnesiums;
pub mod nickels;
pub mod plastics;
pub mod titaniums;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialFamily {
    Aluminum,
    Iron,
    Titanium,
    Nickel,
    Magnesium,
    Copper,
    Composite,
    Plastic,
    Ceramic,
    Glass,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub element: Option<&'static Element>,
    pub name: &'static str,
    pub formula: &'static str,
    pub family: MaterialFamily,
    pub density_kg_m3: f64,
    pub young_modulus_pa: f64,
    pub poisson_ratio: f64,
    pub yield_strength_pa: f64,
    pub ultimate_strength_pa: f64,
    pub thermal_conductivity_w_mk: f64,
    pub thermal_expansion_per_k: f64,
    pub specific_heat_j_kgk: f64,
    pub max_service_temp_k: f64,
    pub fatigue_strength_coeff_pa: f64,
    pub fatigue_strength_exponent: f64,
    pub hardness_hv: f64,
    pub cost_eur_kg: f64,
}

impl Material {
    pub fn density(&self) -> f64 {
        self.density_kg_m3
    }
}
