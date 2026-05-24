#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystType {
    ThreeWay,
    DieselOxidation,
    Scr,
    Gpf,
}

#[derive(Debug, Clone)]
pub struct CatalyticConverter {
    pub catalyst_type: CatalystType,
    pub substrate_cells_per_inch: u16,
    pub precious_metal_g: f64,
    pub lightoff_temp_c: f64,
}

impl CatalyticConverter {
    pub fn three_way() -> Self {
        Self { catalyst_type: CatalystType::ThreeWay, substrate_cells_per_inch: 400, precious_metal_g: 3.5, lightoff_temp_c: 300.0 }
    }

    pub fn diesel_oxidation() -> Self {
        Self { catalyst_type: CatalystType::DieselOxidation, substrate_cells_per_inch: 300, precious_metal_g: 2.0, lightoff_temp_c: 180.0 }
    }

    pub fn scr() -> Self {
        Self { catalyst_type: CatalystType::Scr, substrate_cells_per_inch: 400, precious_metal_g: 0.0, lightoff_temp_c: 200.0 }
    }
}
