use sciforge_core::materials::Material;
use sciforge_core::materials::alus::cast::{A356, AC4B};
use sciforge_core::materials::alus::forged::AL_2618;
use sciforge_core::materials::irons::cast_iron::CGI_400;
use sciforge_core::materials::irons::steels::STEEL_4140;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CamConfig {
    Ohv,
    Sohc,
    Dohc,
}

impl CamConfig {
    pub fn code(&self) -> &'static str {
        match self {
            CamConfig::Ohv => "OHV",
            CamConfig::Sohc => "SOHC",
            CamConfig::Dohc => "DOHC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombustionChamber {
    Wedge,
    Hemi,
    PentRoof,
    Bowl,
    PrecombustionChamber,
}

impl CombustionChamber {
    pub fn code(&self) -> &'static str {
        match self {
            CombustionChamber::Wedge => "WG",
            CombustionChamber::Hemi => "HM",
            CombustionChamber::PentRoof => "PR",
            CombustionChamber::Bowl => "BW",
            CombustionChamber::PrecombustionChamber => "PC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadCooling {
    Air,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadAlloy {
    CastIron,
    CastAluminum,
    HighGradeAluminum,
    ForgedAluminum,
    NitridedSteel,
}

impl HeadAlloy {
    pub fn material(&self) -> &'static Material {
        match self {
            HeadAlloy::CastIron => &CGI_400,
            HeadAlloy::CastAluminum => &AC4B,
            HeadAlloy::HighGradeAluminum => &A356,
            HeadAlloy::ForgedAluminum => &AL_2618,
            HeadAlloy::NitridedSteel => &STEEL_4140,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            HeadAlloy::CastIron => "CI",
            HeadAlloy::CastAluminum => "AC",
            HeadAlloy::HighGradeAluminum => "AH",
            HeadAlloy::ForgedAluminum => "AF",
            HeadAlloy::NitridedSteel => "NS",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Culasse {
    pub variant_code: &'static str,
    pub cylinders: u8,
    pub plugs_per_cylinder: u8,
    pub valves_per_cylinder: u8,
    pub cam_config: CamConfig,
    pub combustion_chamber: CombustionChamber,
    pub cooling: HeadCooling,
    pub alloy: HeadAlloy,
    pub port_diameter_intake_mm: f64,
    pub port_diameter_exhaust_mm: f64,
    pub deck_thickness_mm: f64,
    pub head_bolt_count: u8,
    pub head_bolt_torque_nm: f64,
}

impl Culasse {
    pub fn material(&self) -> &'static Material {
        self.alloy.material()
    }

    pub fn total_valves(&self) -> u16 {
        self.valves_per_cylinder as u16 * self.cylinders as u16
    }

    pub fn total_plugs(&self) -> u16 {
        self.plugs_per_cylinder as u16 * self.cylinders as u16
    }

    pub fn code(&self) -> String {
        format!(
            "{}v{}p-{}-{}-{}",
            self.valves_per_cylinder,
            self.plugs_per_cylinder,
            self.cam_config.code(),
            self.combustion_chamber.code(),
            self.alloy.code(),
        )
    }

    pub fn ohv_2v_1p_wedge(cylinders: u8) -> Self {
        Self {
            variant_code: "VINTAGE",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 2,
            cam_config: CamConfig::Ohv,
            combustion_chamber: CombustionChamber::Wedge,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::CastIron,
            port_diameter_intake_mm: 38.0,
            port_diameter_exhaust_mm: 32.0,
            deck_thickness_mm: 14.0,
            head_bolt_count: cylinders.saturating_mul(4),
            head_bolt_torque_nm: 90.0,
        }
    }

    pub fn ohv_2v_1p_bowl(cylinders: u8) -> Self {
        Self {
            variant_code: "INDU",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 2,
            cam_config: CamConfig::Ohv,
            combustion_chamber: CombustionChamber::Bowl,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::CastIron,
            port_diameter_intake_mm: 36.0,
            port_diameter_exhaust_mm: 30.0,
            deck_thickness_mm: 18.0,
            head_bolt_count: cylinders.saturating_mul(4),
            head_bolt_torque_nm: 140.0,
        }
    }

    pub fn sohc_2v_1p_wedge(cylinders: u8) -> Self {
        Self {
            variant_code: "ECO",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 2,
            cam_config: CamConfig::Sohc,
            combustion_chamber: CombustionChamber::Wedge,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::CastAluminum,
            port_diameter_intake_mm: 36.0,
            port_diameter_exhaust_mm: 30.0,
            deck_thickness_mm: 12.0,
            head_bolt_count: cylinders.saturating_mul(4),
            head_bolt_torque_nm: 80.0,
        }
    }

    pub fn sohc_2v_1p_pre_chamber(cylinders: u8) -> Self {
        Self {
            variant_code: "IDI",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 2,
            cam_config: CamConfig::Sohc,
            combustion_chamber: CombustionChamber::PrecombustionChamber,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::CastIron,
            port_diameter_intake_mm: 35.0,
            port_diameter_exhaust_mm: 29.0,
            deck_thickness_mm: 16.0,
            head_bolt_count: cylinders.saturating_mul(4),
            head_bolt_torque_nm: 130.0,
        }
    }

    pub fn dohc_4v_1p_pent_roof(cylinders: u8) -> Self {
        Self {
            variant_code: "STD",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 4,
            cam_config: CamConfig::Dohc,
            combustion_chamber: CombustionChamber::PentRoof,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::HighGradeAluminum,
            port_diameter_intake_mm: 34.0,
            port_diameter_exhaust_mm: 28.0,
            deck_thickness_mm: 11.0,
            head_bolt_count: cylinders.saturating_mul(4),
            head_bolt_torque_nm: 75.0,
        }
    }

    pub fn dohc_4v_1p_bowl(cylinders: u8) -> Self {
        Self {
            variant_code: "CR",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 4,
            cam_config: CamConfig::Dohc,
            combustion_chamber: CombustionChamber::Bowl,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::HighGradeAluminum,
            port_diameter_intake_mm: 33.0,
            port_diameter_exhaust_mm: 28.0,
            deck_thickness_mm: 14.0,
            head_bolt_count: cylinders.saturating_mul(6),
            head_bolt_torque_nm: 120.0,
        }
    }

    pub fn dohc_4v_2p_hemi(cylinders: u8) -> Self {
        Self {
            variant_code: "TWIN",
            cylinders,
            plugs_per_cylinder: 2,
            valves_per_cylinder: 4,
            cam_config: CamConfig::Dohc,
            combustion_chamber: CombustionChamber::Hemi,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::ForgedAluminum,
            port_diameter_intake_mm: 36.0,
            port_diameter_exhaust_mm: 30.0,
            deck_thickness_mm: 12.0,
            head_bolt_count: cylinders.saturating_mul(6),
            head_bolt_torque_nm: 95.0,
        }
    }

    pub fn dohc_4v_2p_bowl(cylinders: u8) -> Self {
        Self {
            variant_code: "TWIN",
            cylinders,
            plugs_per_cylinder: 2,
            valves_per_cylinder: 4,
            cam_config: CamConfig::Dohc,
            combustion_chamber: CombustionChamber::Bowl,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::ForgedAluminum,
            port_diameter_intake_mm: 34.0,
            port_diameter_exhaust_mm: 29.0,
            deck_thickness_mm: 15.0,
            head_bolt_count: cylinders.saturating_mul(6),
            head_bolt_torque_nm: 130.0,
        }
    }

    pub fn dohc_5v_1p(cylinders: u8) -> Self {
        Self {
            variant_code: "PERF",
            cylinders,
            plugs_per_cylinder: 1,
            valves_per_cylinder: 5,
            cam_config: CamConfig::Dohc,
            combustion_chamber: CombustionChamber::PentRoof,
            cooling: HeadCooling::Water,
            alloy: HeadAlloy::ForgedAluminum,
            port_diameter_intake_mm: 32.0,
            port_diameter_exhaust_mm: 27.0,
            deck_thickness_mm: 11.0,
            head_bolt_count: cylinders.saturating_mul(6),
            head_bolt_torque_nm: 90.0,
        }
    }
}
