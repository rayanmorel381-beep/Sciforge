#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchraderCoreMaterial {
    Brass,
    NickelPlated,
    Stainless,
}

#[derive(Debug, Clone)]
pub struct SchraderValve {
    pub core_material: SchraderCoreMaterial,
    pub thread_diameter_mm: f64,
    pub valve_length_mm: u8,
    pub max_pressure_bar: f64,
    pub metal_body: bool,
    pub removable_core: bool,
}

impl SchraderValve {
    pub fn standard() -> Self {
        Self {
            core_material: SchraderCoreMaterial::Brass,
            thread_diameter_mm: 8.0,
            valve_length_mm: 29,
            max_pressure_bar: 6.0,
            metal_body: false,
            removable_core: true,
        }
    }

    pub fn metal() -> Self {
        Self {
            core_material: SchraderCoreMaterial::NickelPlated,
            thread_diameter_mm: 8.0,
            valve_length_mm: 29,
            max_pressure_bar: 10.0,
            metal_body: true,
            removable_core: true,
        }
    }

    pub fn heavy_duty() -> Self {
        Self {
            core_material: SchraderCoreMaterial::Stainless,
            thread_diameter_mm: 8.0,
            valve_length_mm: 40,
            max_pressure_bar: 15.0,
            metal_body: true,
            removable_core: true,
        }
    }
}
