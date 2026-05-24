#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiatorMaterial {
    Aluminum,
    CopperBrass,
    Composite,
}

#[derive(Debug, Clone)]
pub struct Radiator {
    pub material: RadiatorMaterial,
    pub core_rows: u8,
    pub area_m2: f64,
    pub electric_fan: bool,
    pub viscous_fan: bool,
}

impl Radiator {
    pub fn standard(material: RadiatorMaterial, area_m2: f64) -> Self {
        Self { material, core_rows: 2, area_m2, electric_fan: false, viscous_fan: true }
    }

    pub fn electric_fan(material: RadiatorMaterial, area_m2: f64) -> Self {
        Self { material, core_rows: 2, area_m2, electric_fan: true, viscous_fan: false }
    }

    pub fn performance(material: RadiatorMaterial, area_m2: f64) -> Self {
        Self { material, core_rows: 3, area_m2, electric_fan: true, viscous_fan: false }
    }
}
