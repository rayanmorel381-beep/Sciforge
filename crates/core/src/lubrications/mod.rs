pub mod ehl;
pub mod greases;
pub mod hydrodynamic;
pub mod interface;
pub mod properties;

pub use greases::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreaseFamily {
    Lithium,
    LithiumComplex,
    Calcium,
    CalciumSulfonate,
    Molybdenum,
    Polyurea,
    Aluminum,
    Sodium,
    Bentone,
    Silicone,
    Ptfe,
    Copper,
}

#[derive(Debug, Clone, Copy)]
pub struct Grease {
    pub name: &'static str,
    pub thickener: &'static str,
    pub family: GreaseFamily,
    pub nlgi_grade: u8,
    pub base_oil_viscosity_mm2_s: f64,
    pub density_kg_m3: f64,
    pub dropping_point_k: f64,
    pub operating_temp_min_k: f64,
    pub operating_temp_max_k: f64,
    pub penetration_worked_dmm: f64,
    pub friction_coefficient: f64,
    pub weld_point_n: f64,
}

impl Grease {
    pub fn is_suitable_for_temperature(&self, temp_k: f64) -> bool {
        temp_k >= self.operating_temp_min_k && temp_k <= self.operating_temp_max_k
    }

    pub fn is_suitable_for_load(&self, load_n: f64) -> bool {
        load_n <= self.weld_point_n
    }

    pub fn temperature_range_k(&self) -> (f64, f64) {
        (self.operating_temp_min_k, self.operating_temp_max_k)
    }
}
