#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrestaValveMaterial {
    AluminumAlloy,
    BrassCore,
    Titanium,
}

#[derive(Debug, Clone)]
pub struct PrestaValve {
    pub material: PrestaValveMaterial,
    pub thread_diameter_mm: f64,
    pub valve_length_mm: u8,
    pub max_pressure_bar: f64,
    pub locknut: bool,
    pub removable_core: bool,
}

impl PrestaValve {
    pub fn standard() -> Self {
        Self {
            material: PrestaValveMaterial::AluminumAlloy,
            thread_diameter_mm: 6.0,
            valve_length_mm: 32,
            max_pressure_bar: 15.0,
            locknut: true,
            removable_core: false,
        }
    }

    pub fn extended() -> Self {
        Self {
            material: PrestaValveMaterial::AluminumAlloy,
            thread_diameter_mm: 6.0,
            valve_length_mm: 60,
            max_pressure_bar: 15.0,
            locknut: true,
            removable_core: true,
        }
    }

    pub fn titanium() -> Self {
        Self {
            material: PrestaValveMaterial::Titanium,
            thread_diameter_mm: 6.0,
            valve_length_mm: 42,
            max_pressure_bar: 20.0,
            locknut: false,
            removable_core: true,
        }
    }
}
