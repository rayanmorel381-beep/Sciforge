#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolantType {
    WaterGlycol,
    DielectricOil,
    DirectImmersion,
    Refrigerant,
}

#[derive(Debug, Clone)]
pub struct CoolingLoop {
    pub coolant_type: CoolantType,
    pub coolant_capacity_l: f64,
    pub pump_flow_l_min: f64,
    pub radiator_area_m2: f64,
    pub max_heat_rejection_kw: f64,
    pub coolant_density_kg_m3: f64,
    pub coolant_specific_heat_j_kg_k: f64,
    pub mass_kg: f64,
}

impl Default for CoolingLoop {
    fn default() -> Self {
        Self::glycol(85.0)
    }
}

fn coolant_specs(kind: CoolantType) -> (f64, f64, f64) {
    match kind {
        CoolantType::WaterGlycol => (1050.0, 3500.0, 25.0),
        CoolantType::DielectricOil => (820.0, 2100.0, 18.0),
        CoolantType::DirectImmersion => (820.0, 2100.0, 35.0),
        CoolantType::Refrigerant => (1200.0, 1450.0, 60.0),
    }
}

fn build(kind: CoolantType, max_heat_rejection_kw: f64) -> CoolingLoop {
    let (density, cp, rejection_density_kw_m2) = coolant_specs(kind);
    let radiator_area_m2 = (max_heat_rejection_kw / rejection_density_kw_m2).max(0.15);
    let coolant_capacity_l = 0.06 * max_heat_rejection_kw + 1.5;
    let delta_t_k = 8.0;
    let mass_flow_kg_s = max_heat_rejection_kw * 1000.0 / (cp * delta_t_k);
    let volumetric_flow_m3_s = mass_flow_kg_s / density;
    let pump_flow_l_min = volumetric_flow_m3_s * 60_000.0;
    let coolant_mass = coolant_capacity_l * density / 1000.0;
    let radiator_mass = radiator_area_m2 * 4.5;
    let pump_mass = 2.5 + max_heat_rejection_kw * 0.04;
    let mass_kg = coolant_mass + radiator_mass + pump_mass;
    CoolingLoop {
        coolant_type: kind,
        coolant_capacity_l: (coolant_capacity_l * 100.0).round() / 100.0,
        pump_flow_l_min: (pump_flow_l_min * 10.0).round() / 10.0,
        radiator_area_m2: (radiator_area_m2 * 100.0).round() / 100.0,
        max_heat_rejection_kw,
        coolant_density_kg_m3: density,
        coolant_specific_heat_j_kg_k: cp,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
    }
}

impl CoolingLoop {
    pub fn glycol(max_heat_rejection_kw: f64) -> Self {
        build(CoolantType::WaterGlycol, max_heat_rejection_kw)
    }
    pub fn dielectric_oil(max_heat_rejection_kw: f64) -> Self {
        build(CoolantType::DielectricOil, max_heat_rejection_kw)
    }
    pub fn direct_immersion(max_heat_rejection_kw: f64) -> Self {
        build(CoolantType::DirectImmersion, max_heat_rejection_kw)
    }
    pub fn refrigerant(max_heat_rejection_kw: f64) -> Self {
        build(CoolantType::Refrigerant, max_heat_rejection_kw)
    }

    pub fn for_losses(losses_kw: f64) -> Self {
        Self::glycol(losses_kw * 1.25)
    }
}
