#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelMaterial {
    Steel,
    Aluminum,
    CarbonFibre,
    Smc,
}

#[derive(Debug, Clone)]
pub struct Hood {
    pub material: PanelMaterial,
    pub vented: bool,
    pub scoop: bool,
    pub weight_kg: f64,
}

impl Hood {
    pub fn steel(weight_kg: f64) -> Self {
        Self { material: PanelMaterial::Steel, vented: false, scoop: false, weight_kg }
    }

    pub fn aluminum(weight_kg: f64) -> Self {
        Self { material: PanelMaterial::Aluminum, vented: false, scoop: false, weight_kg }
    }

    pub fn vented(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { material, vented: true, scoop: false, weight_kg }
    }

    pub fn with_scoop(material: PanelMaterial, weight_kg: f64) -> Self {
        Self { material, vented: true, scoop: true, weight_kg }
    }
}
