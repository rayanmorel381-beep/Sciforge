use sciforge_core::materials::Material;
use sciforge_core::materials::irons::cast_iron::CAST_IRON_GREY;
use sciforge_core::materials::irons::stainless::STAINLESS_304;
use sciforge_core::materials::irons::steels::STEEL_1018;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[allow(non_upper_case_globals)]
pub struct ExhaustMaterial;

#[allow(non_upper_case_globals)]
impl ExhaustMaterial {
    pub const MildSteel: &'static std::sync::LazyLock<Material> = &STEEL_1018;
    pub const StainlessSteel: &'static std::sync::LazyLock<Material> = &STAINLESS_304;
    pub const Titanium: &'static std::sync::LazyLock<Material> = &TI6AL4V_GR5;
    pub const CastIron: &'static std::sync::LazyLock<Material> = &CAST_IRON_GREY;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderType {
    Shorty,
    LongTube,
    EqualLength,
    TriY,
    XPipe,
    YPipe,
    HPipe,
}

#[derive(Debug, Clone)]
pub struct ExhaustSystem {
    pub header_type: HeaderType,
    pub material: &'static std::sync::LazyLock<Material>,
    pub header_primary_mm: f64,
    pub collector_diameter_mm: f64,
    pub catalyst_count: u8,
    pub back_pressure_kpa: f64,
    pub ceramic_coated: bool,
}

impl ExhaustSystem {
    pub fn stock(cylinders: u8) -> Self {
        Self {
            header_type: HeaderType::Shorty,
            material: ExhaustMaterial::StainlessSteel,
            header_primary_mm: 38.0,
            collector_diameter_mm: 57.0,
            catalyst_count: if cylinders <= 4 { 1 } else { 2 },
            back_pressure_kpa: 18.0,
            ceramic_coated: false,
        }
    }

    pub fn performance(cylinders: u8) -> Self {
        Self {
            header_type: HeaderType::EqualLength,
            material: ExhaustMaterial::StainlessSteel,
            header_primary_mm: 42.0,
            collector_diameter_mm: 63.5,
            catalyst_count: if cylinders <= 4 { 1 } else { 2 },
            back_pressure_kpa: 10.0,
            ceramic_coated: true,
        }
    }

    pub fn race(cylinders: u8) -> Self {
        Self {
            header_type: HeaderType::EqualLength,
            material: ExhaustMaterial::Titanium,
            header_primary_mm: 40.0 + cylinders as f64,
            collector_diameter_mm: 57.0 + cylinders as f64 * 2.5,
            catalyst_count: 0,
            back_pressure_kpa: 4.0,
            ceramic_coated: true,
        }
    }

    pub fn silencer_count(&self) -> u8 {
        if self.back_pressure_kpa > 12.0 { 2 } else if self.back_pressure_kpa > 6.0 { 1 } else { 0 }
    }

    pub fn silencer_chamber_volume_l(&self) -> f64 {
        (self.back_pressure_kpa * 0.4).max(0.0)
    }

    pub fn silencer_chamber_length_mm(&self) -> f64 {
        if self.silencer_count() == 0 { 0.0 } else { 250.0 + self.back_pressure_kpa * 8.0 }
    }

    pub fn tail_pipe_diameter_mm(&self) -> f64 {
        self.collector_diameter_mm * 1.05
    }

    pub fn transmission_loss_db(&self, freq_hz: f64, gas_temp_k: f64) -> f64 {
        use crate::sounds::acoustics as ac;
        let mut total = 0.0;
        let n = self.silencer_count();
        if n > 0 {
            let chamber_l_m = self.silencer_chamber_length_mm() / 1000.0;
            let chamber_vol_m3 = self.silencer_chamber_volume_l() * 1.0e-3;
            let chamber_area_m2 = chamber_vol_m3 / chamber_l_m.max(0.05);
            let pipe_d_m = self.collector_diameter_mm / 1000.0;
            let pipe_area_m2 = std::f64::consts::PI * pipe_d_m.powi(2) / 4.0;
            let area_ratio = chamber_area_m2 / pipe_area_m2.max(1.0e-6);
            let per_chamber = ac::expansion_chamber_tl_db(freq_hz, area_ratio, chamber_l_m, gas_temp_k);
            total += per_chamber * n as f64;
        }
        total += self.catalyst_count as f64 * 4.0;
        if self.ceramic_coated { total += 1.5; }
        match self.header_type {
            HeaderType::LongTube | HeaderType::EqualLength | HeaderType::TriY => total += 0.5,
            HeaderType::HPipe | HeaderType::XPipe | HeaderType::YPipe => total += 1.0,
            HeaderType::Shorty => total += 2.0,
        }
        total
    }

    pub fn tail_pipe_loudness_db(
        &self,
        source_spl_db: f64,
        rpm: u32,
        cylinders: u8,
        gas_temp_k: f64,
    ) -> f64 {
        use crate::sounds::acoustics as ac;
        let f0 = ac::fundamental_hz(rpm, cylinders);
        let tl = self.transmission_loss_db(f0, gas_temp_k);
        source_spl_db - tl
    }
}
