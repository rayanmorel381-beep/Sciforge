use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatorType {
    InletGuideVane,
    CompressorStator,
    TurbineNozzleGuideVane,
    OutletGuideVane,
}

#[derive(Debug, Clone)]
pub struct StatorVane {
    pub stator_type: StatorType,
    pub vane_count: u8,
    pub chord_mm: f64,
    pub aspect_ratio: f64,
    pub variable_geometry: bool,
    pub cooling: bool,
    pub material_density_kg_m3: f64,
}

impl StatorVane {
    pub fn inlet_guide_vane(vane_count: u8, variable_geometry: bool) -> Self {
        Self {
            stator_type: StatorType::InletGuideVane,
            vane_count,
            chord_mm: 60.0,
            aspect_ratio: 2.8,
            variable_geometry,
            cooling: false,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
        }
    }

    pub fn compressor_stator(vane_count: u8, variable_geometry: bool) -> Self {
        Self {
            stator_type: StatorType::CompressorStator,
            vane_count,
            chord_mm: 45.0,
            aspect_ratio: 2.2,
            variable_geometry,
            cooling: false,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
        }
    }

    pub fn nozzle_guide_vane(vane_count: u8, turbine_entry_temp_k: f64) -> Self {
        let cooling = turbine_entry_temp_k > 1_500.0;
        Self {
            stator_type: StatorType::TurbineNozzleGuideVane,
            vane_count,
            chord_mm: 55.0,
            aspect_ratio: 1.4,
            variable_geometry: false,
            cooling,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn outlet_guide_vane(vane_count: u8) -> Self {
        Self {
            stator_type: StatorType::OutletGuideVane,
            vane_count,
            chord_mm: 70.0,
            aspect_ratio: 3.0,
            variable_geometry: false,
            cooling: false,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
        }
    }
}
