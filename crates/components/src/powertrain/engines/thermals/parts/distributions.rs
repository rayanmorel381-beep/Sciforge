use sciforge_core::materials::Material;
use sciforge_core::materials::irons::cast_iron::CAST_IRON_NODULAR_SG;
use sciforge_core::materials::irons::nitrided::STEEL_NITRIDED_32CRMOV13;
use sciforge_core::materials::irons::steels::STEEL_8620;
use sciforge_core::materials::plastics::elastomers::FKM_VITON;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingDriveType {
    Belt,
    Chain,
    DualChain,
    GearDrive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeltProfile {
    Trapezoidal,
    Htd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensionerType {
    Spring,
    Hydraulic,
    Automatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainType {
    SingleRow,
    DoubleRow,
    Silent,
}

#[derive(Debug, Clone)]
pub struct BeltSystem {
    pub profile: BeltProfile,
    pub width_mm: f64,
    pub teeth_count: u16,
    pub tensioner: TensionerType,
    pub idler_count: u8,
    pub water_pump_driven: bool,
    pub replacement_interval_km: u32,
}

#[derive(Debug, Clone)]
pub struct ChainSystem {
    pub chain_type: ChainType,
    pub link_count: u16,
    pub pitch_mm: f64,
    pub tensioner: TensionerType,
    pub guide_count: u8,
    pub sprocket_count: u8,
}

#[derive(Debug, Clone)]
pub struct DualChainSystem {
    pub primary: ChainSystem,
    pub secondary: ChainSystem,
    pub phaser_count: u8,
}

#[derive(Debug, Clone)]
pub struct GearDriveSystem {
    pub gear_count: u8,
    pub module_mm: f64,
    pub idler_count: u8,
    pub backlash_mm: f64,
}

#[derive(Debug, Clone)]
pub enum TimingDrive {
    Belt(BeltSystem),
    Chain(ChainSystem),
    DualChain(DualChainSystem),
    GearDrive(GearDriveSystem),
}

impl TimingDrive {
    pub fn drive_type(&self) -> TimingDriveType {
        match self {
            TimingDrive::Belt(_) => TimingDriveType::Belt,
            TimingDrive::Chain(_) => TimingDriveType::Chain,
            TimingDrive::DualChain(_) => TimingDriveType::DualChain,
            TimingDrive::GearDrive(_) => TimingDriveType::GearDrive,
        }
    }

    pub fn material(&self) -> &'static Material {
        match self {
            TimingDrive::Belt(_) => &FKM_VITON,
            TimingDrive::Chain(_) | TimingDrive::DualChain(_) => &STEEL_8620,
            TimingDrive::GearDrive(_) => &STEEL_NITRIDED_32CRMOV13,
        }
    }

    pub fn guide_material(&self) -> Option<&'static Material> {
        match self {
            TimingDrive::Chain(_) | TimingDrive::DualChain(_) => Some(&CAST_IRON_NODULAR_SG),
            _ => None,
        }
    }

    pub fn noise_factor(&self) -> f64 {
        match self {
            TimingDrive::Belt(_) => 0.10,
            TimingDrive::Chain(s) => match s.chain_type {
                ChainType::Silent => 0.30,
                ChainType::SingleRow => 0.55,
                ChainType::DoubleRow => 0.65,
            },
            TimingDrive::DualChain(_) => 0.70,
            TimingDrive::GearDrive(_) => 0.45,
        }
    }

    pub fn belt() -> Self {
        TimingDrive::Belt(BeltSystem {
            profile: BeltProfile::Htd,
            width_mm: 25.4,
            teeth_count: 120,
            tensioner: TensionerType::Automatic,
            idler_count: 1,
            water_pump_driven: true,
            replacement_interval_km: 120_000,
        })
    }

    pub fn chain() -> Self {
        TimingDrive::Chain(ChainSystem {
            chain_type: ChainType::SingleRow,
            link_count: 96,
            pitch_mm: 8.0,
            tensioner: TensionerType::Hydraulic,
            guide_count: 2,
            sprocket_count: 3,
        })
    }

    pub fn dual_chain() -> Self {
        let primary = ChainSystem {
            chain_type: ChainType::DoubleRow,
            link_count: 64,
            pitch_mm: 8.0,
            tensioner: TensionerType::Hydraulic,
            guide_count: 2,
            sprocket_count: 2,
        };
        let secondary = ChainSystem {
            chain_type: ChainType::SingleRow,
            link_count: 80,
            pitch_mm: 6.35,
            tensioner: TensionerType::Hydraulic,
            guide_count: 2,
            sprocket_count: 3,
        };
        TimingDrive::DualChain(DualChainSystem {
            primary,
            secondary,
            phaser_count: 2,
        })
    }

    pub fn gear_drive() -> Self {
        TimingDrive::GearDrive(GearDriveSystem {
            gear_count: 3,
            module_mm: 2.5,
            idler_count: 1,
            backlash_mm: 0.05,
        })
    }
}
