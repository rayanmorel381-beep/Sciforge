use sciforge_core::materials::Material;
use sciforge_core::materials::coppers::pure::COPPER_C110;
use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::irons::steels::STEEL_4140;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparkPlugTip {
    Copper,
    Platinum,
    Iridium,
}

impl SparkPlugTip {
    pub fn electrode_diameter_mm(self) -> f64 {
        match self {
            SparkPlugTip::Copper => 2.5,
            SparkPlugTip::Platinum => 0.8,
            SparkPlugTip::Iridium => 0.4,
        }
    }

    pub fn rated_lifetime_km(self) -> f64 {
        match self {
            SparkPlugTip::Copper => 30_000.0,
            SparkPlugTip::Platinum => 100_000.0,
            SparkPlugTip::Iridium => 160_000.0,
        }
    }

    pub fn ignition_voltage_kv(self) -> f64 {
        match self {
            SparkPlugTip::Copper => 25.0,
            SparkPlugTip::Platinum => 30.0,
            SparkPlugTip::Iridium => 35.0,
        }
    }

    pub fn material(self) -> &'static Material {
        match self {
            SparkPlugTip::Copper => &COPPER_C110,
            SparkPlugTip::Platinum | SparkPlugTip::Iridium => &INCONEL_718,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SparkPlug {
    pub tip: SparkPlugTip,
    pub heat_range: u8,
    pub gap_mm: f64,
    pub thread: &'static str,
    pub reach_mm: f64,
    pub spark_energy_mj: f64,
    pub cylinders: u8,
    pub plugs_per_cylinder: u8,
}

impl SparkPlug {
    pub fn standard(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Self {
            tip: SparkPlugTip::Copper,
            heat_range: 6,
            gap_mm: 0.9,
            thread: "M14x1.25",
            reach_mm: 19.0,
            spark_energy_mj: 40.0,
            cylinders,
            plugs_per_cylinder,
        }
    }

    pub fn performance(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Self {
            tip: SparkPlugTip::Iridium,
            heat_range: 8,
            gap_mm: 0.7,
            thread: "M12x1.25",
            reach_mm: 26.5,
            spark_energy_mj: 80.0,
            cylinders,
            plugs_per_cylinder,
        }
    }

    pub fn long_life(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Self {
            tip: SparkPlugTip::Platinum,
            heat_range: 6,
            gap_mm: 1.0,
            thread: "M14x1.25",
            reach_mm: 19.0,
            spark_energy_mj: 50.0,
            cylinders,
            plugs_per_cylinder,
        }
    }

    pub fn total_count(&self) -> u16 {
        self.cylinders as u16 * self.plugs_per_cylinder as u16
    }

    pub fn shell_material(&self) -> &'static Material {
        &STEEL_4140
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlowPlugType {
    MetalSheath,
    CeramicSiN,
}

impl GlowPlugType {
    pub fn warmup_time_s(self) -> f64 {
        match self {
            GlowPlugType::MetalSheath => 6.0,
            GlowPlugType::CeramicSiN => 2.0,
        }
    }

    pub fn peak_tip_temp_k(self) -> f64 {
        match self {
            GlowPlugType::MetalSheath => 1173.0,
            GlowPlugType::CeramicSiN => 1373.0,
        }
    }

    pub fn rated_voltage_v(self) -> f64 {
        match self {
            GlowPlugType::MetalSheath => 11.0,
            GlowPlugType::CeramicSiN => 7.0,
        }
    }

    pub fn rated_current_a(self) -> f64 {
        match self {
            GlowPlugType::MetalSheath => 12.0,
            GlowPlugType::CeramicSiN => 22.0,
        }
    }

    pub fn sheath_material(self) -> &'static Material {
        match self {
            GlowPlugType::MetalSheath => &INCONEL_718,
            GlowPlugType::CeramicSiN => &STEEL_4140,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlowPlug {
    pub plug_type: GlowPlugType,
    pub thread: &'static str,
    pub reach_mm: f64,
    pub post_heat_s: f64,
    pub cylinders: u8,
    pub plugs_per_cylinder: u8,
}

impl GlowPlug {
    pub fn standard(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Self {
            plug_type: GlowPlugType::MetalSheath,
            thread: "M10x1.0",
            reach_mm: 24.0,
            post_heat_s: 180.0,
            cylinders,
            plugs_per_cylinder,
        }
    }

    pub fn ceramic(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Self {
            plug_type: GlowPlugType::CeramicSiN,
            thread: "M10x1.0",
            reach_mm: 26.0,
            post_heat_s: 300.0,
            cylinders,
            plugs_per_cylinder,
        }
    }

    pub fn total_count(&self) -> u16 {
        self.cylinders as u16 * self.plugs_per_cylinder as u16
    }

    pub fn power_w(&self) -> f64 {
        self.plug_type.rated_voltage_v() * self.plug_type.rated_current_a() * self.total_count() as f64
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ignition {
    Spark(SparkPlug),
    Glow(GlowPlug),
}

impl Ignition {
    pub fn for_gasoline(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Ignition::Spark(SparkPlug::standard(cylinders, plugs_per_cylinder))
    }

    pub fn for_diesel(cylinders: u8, plugs_per_cylinder: u8) -> Self {
        Ignition::Glow(GlowPlug::standard(cylinders, plugs_per_cylinder))
    }
}
