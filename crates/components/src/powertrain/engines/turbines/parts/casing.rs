use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CasingSection {
    FanCasing,
    CompressorCasing,
    CombustionCasing,
    TurbineCasing,
    JetPipe,
}

#[derive(Debug, Clone)]
pub struct Casing {
    pub section: CasingSection,
    pub diameter_m: f64,
    pub thickness_mm: f64,
    pub containment_rated: bool,
    pub abradable_coating: bool,
    pub material_density_kg_m3: f64,
}

impl Casing {
    pub fn fan(diameter_m: f64) -> Self {
        Self {
            section: CasingSection::FanCasing,
            diameter_m,
            thickness_mm: 8.0,
            containment_rated: true,
            abradable_coating: true,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
        }
    }

    pub fn compressor(diameter_m: f64) -> Self {
        Self {
            section: CasingSection::CompressorCasing,
            diameter_m,
            thickness_mm: 5.0,
            containment_rated: false,
            abradable_coating: true,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
        }
    }

    pub fn turbine(diameter_m: f64) -> Self {
        Self {
            section: CasingSection::TurbineCasing,
            diameter_m,
            thickness_mm: 12.0,
            containment_rated: true,
            abradable_coating: true,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn jet_pipe(diameter_m: f64) -> Self {
        Self {
            section: CasingSection::JetPipe,
            diameter_m,
            thickness_mm: 6.0,
            containment_rated: false,
            abradable_coating: false,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
        }
    }

    pub fn mass_per_meter_kg(&self) -> f64 {
        let circumference = std::f64::consts::PI * self.diameter_m;
        circumference * (self.thickness_mm / 1_000.0) * self.material_density_kg_m3
    }
}
