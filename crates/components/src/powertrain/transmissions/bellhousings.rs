use crate::powertrain::engines::thermals::assemblies::engine::Engine;
use crate::powertrain::transmissions::clutches::{
    Clutch, DryCutch, DualClutch, WetClutch,
};
use crate::powertrain::transmissions::gearboxes::{
    AutomaticGearbox, AutomaticKind, Cvt, Gearbox, ManualGearbox, RobotizedManual, SequentialGearbox,
};
use sciforge_hub::prelude::physics_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GearboxKind {
    Manual,
    Sequential,
    DctDsg,
    TorqueConverter,
    Cvt,
    RobotizedManual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HousingMaterial {
    AluminumCast,
    MagnesiumAlloy,
    CastIron,
    SteelStamped,
}

impl HousingMaterial {
    pub fn density_kg_m3(self) -> f64 {
        match self {
            Self::AluminumCast => 2700.0,
            Self::MagnesiumAlloy => 1810.0,
            Self::CastIron => 7200.0,
            Self::SteelStamped => 7850.0,
        }
    }

    pub fn yield_strength_pa(self) -> f64 {
        match self {
            Self::AluminumCast => 240.0e6,
            Self::MagnesiumAlloy => 160.0e6,
            Self::CastIron => 200.0e6,
            Self::SteelStamped => 350.0e6,
        }
    }

    pub fn safety_factor(self) -> f64 { 2.0 }

    pub fn allowable_stress_pa(self) -> f64 {
        self.yield_strength_pa() / self.safety_factor()
    }

    pub fn allowable_shear_pa(self) -> f64 {
        self.allowable_stress_pa() / 3.0_f64.sqrt()
    }
}

impl GearboxKind {
    pub fn from_gearbox(g: &Gearbox) -> Self {
        match g {
            Gearbox::Manual(_) => Self::Manual,
            Gearbox::Sequential(_) => Self::Sequential,
            Gearbox::Automatic(a) => match a.kind {
                AutomaticKind::Dct => Self::DctDsg,
                AutomaticKind::TorqueConverter => Self::TorqueConverter,
            },
            Gearbox::Cvt(_) => Self::Cvt,
            Gearbox::RobotizedManual(_) => Self::RobotizedManual,
        }
    }

    pub fn length_mm(self) -> f64 {
        match self {
            Self::Sequential => 95.0,
            Self::Manual => 110.0,
            Self::RobotizedManual => 125.0,
            Self::DctDsg => 140.0,
            Self::Cvt => 150.0,
            Self::TorqueConverter => 185.0,
        }
    }

    pub fn radial_clearance_mm(self) -> f64 {
        match self {
            Self::Sequential => 12.0,
            Self::Manual | Self::RobotizedManual => 18.0,
            Self::DctDsg | Self::Cvt => 22.0,
            Self::TorqueConverter => 28.0,
        }
    }

    pub fn min_wall_mm(self) -> f64 {
        match self {
            Self::Sequential => 5.0,
            Self::Manual | Self::RobotizedManual => 6.0,
            Self::DctDsg | Self::Cvt => 8.0,
            Self::TorqueConverter => 10.0,
        }
    }

    pub fn material(self) -> HousingMaterial {
        match self {
            Self::Sequential => HousingMaterial::MagnesiumAlloy,
            Self::Manual | Self::RobotizedManual | Self::DctDsg | Self::Cvt => HousingMaterial::AluminumCast,
            Self::TorqueConverter => HousingMaterial::CastIron,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bellhousing {
    pub gearbox_kind: GearboxKind,
    pub material: HousingMaterial,
    pub flywheel_diameter_mm: f64,
    pub clutch_diameter_mm: f64,
    pub bolt_circle_diameter_mm: f64,
    pub inner_diameter_mm: f64,
    pub outer_diameter_mm: f64,
    pub wall_thickness_mm: f64,
    pub length_mm: f64,
    pub mass_kg: f64,
    pub design_torque_nm: f64,
    pub gearbox_mass_kg: f64,
    pub torsion_stress_pa: f64,
    pub bending_stress_pa: f64,
    pub von_mises_stress_pa: f64,
    pub safety_margin: f64,
}

fn dual_clutch_diameter_mm(c: &DualClutch) -> f64 {
    let t = c.max_torque_nm.max(50.0);
    180.0 + (t / 10.0).sqrt() * 6.0
}

pub fn dry_diameter_mm(c: &DryCutch) -> f64 { c.disc_diameter_mm }
pub fn wet_diameter_mm(c: &WetClutch) -> f64 { c.disc_diameter_mm }

pub fn clutch_diameter_mm(clutch: &Clutch) -> f64 {
    match clutch {
        Clutch::Dry(c) => c.disc_diameter_mm,
        Clutch::Wet(c) => c.disc_diameter_mm,
        Clutch::Dual(c) => dual_clutch_diameter_mm(c),
    }
}

fn manual_max_torque(g: &ManualGearbox) -> f64 { g.max_input_torque_nm }
fn sequential_max_torque(g: &SequentialGearbox) -> f64 { g.max_input_torque_nm }
fn automatic_max_torque(g: &AutomaticGearbox) -> f64 { g.max_input_torque_nm }
fn cvt_max_torque(g: &Cvt) -> f64 { g.max_input_torque_nm }
fn robotized_max_torque(g: &RobotizedManual) -> f64 { g.gearbox.max_input_torque_nm }

pub fn gearbox_max_input_torque_nm(g: &Gearbox) -> f64 {
    match g {
        Gearbox::Manual(g) => manual_max_torque(g),
        Gearbox::Sequential(g) => sequential_max_torque(g),
        Gearbox::Automatic(g) => automatic_max_torque(g),
        Gearbox::Cvt(g) => cvt_max_torque(g),
        Gearbox::RobotizedManual(g) => robotized_max_torque(g),
    }
}

fn engine_peak_torque_nm(engine: &Engine) -> f64 {
    let bmep_pa = engine.peak_pressure_bar * 1.0e5 * 0.30;
    let vd_m3 = engine.bloc.displacement_l() / 1000.0;
    bmep_pa * vd_m3 / (4.0 * std::f64::consts::PI)
}

pub fn estimated_gearbox_mass_kg(g: &Gearbox) -> f64 {
    let kind = GearboxKind::from_gearbox(g);
    let t_max = gearbox_max_input_torque_nm(g);
    let base = match kind {
        GearboxKind::Sequential => 18.0,
        GearboxKind::Manual => 28.0,
        GearboxKind::RobotizedManual => 32.0,
        GearboxKind::DctDsg => 70.0,
        GearboxKind::Cvt => 55.0,
        GearboxKind::TorqueConverter => 85.0,
    };
    base + (t_max / 10.0).max(0.0)
}

pub fn assemble(engine: &Engine, clutch: Option<&Clutch>, gearbox: &Gearbox) -> Bellhousing {
    let kind = GearboxKind::from_gearbox(gearbox);
    let material = kind.material();
    let flywheel_d_mm = engine.flywheel.diameter_mm;
    let clutch_d_mm = clutch.map(clutch_diameter_mm).unwrap_or(0.0);
    let inner_d_mm = flywheel_d_mm.max(clutch_d_mm) + kind.radial_clearance_mm();
    let r_mount_mm = inner_d_mm / 2.0 + 12.0;
    let r_mount_m = r_mount_mm / 1000.0;
    let length_mm = kind.length_mm();
    let length_m = length_mm / 1000.0;

    let t_engine = engine_peak_torque_nm(engine);
    let t_gb = gearbox_max_input_torque_nm(gearbox);
    let design_torque_nm = t_engine.max(t_gb) * 1.25;

    let gearbox_mass_kg = estimated_gearbox_mass_kg(gearbox);
    let g_acc = 9.81;
    let cantilever_arm_m = length_m;
    let bending_moment_nm = gearbox_mass_kg * g_acc * cantilever_arm_m;

    let sigma_allow = material.allowable_stress_pa();
    let tau_allow = material.allowable_shear_pa();

    let t_torsion_m = design_torque_nm / (2.0 * std::f64::consts::PI * r_mount_m.powi(2) * tau_allow);
    let t_bend_m = bending_moment_nm / (std::f64::consts::PI * r_mount_m.powi(2) * sigma_allow);
    let t_min_m = kind.min_wall_mm() / 1000.0;
    let wall_m = t_torsion_m.max(t_bend_m).max(t_min_m);
    let wall_mm = wall_m * 1000.0;

    let inner_m = inner_d_mm / 1000.0;
    let outer_m = inner_m + 2.0 * wall_m;
    let outer_d_mm = outer_m * 1000.0;

    let r_outer_m = outer_m / 2.0;
    let r_inner_m = inner_m / 2.0;
    let area_annular_m2 = std::f64::consts::PI * (r_outer_m.powi(2) - r_inner_m.powi(2));
    let i_annular_m4 = std::f64::consts::PI * (r_outer_m.powi(4) - r_inner_m.powi(4)) / 4.0;
    let j_polar_m4 = 2.0 * i_annular_m4;

    let torsion_stress_pa = design_torque_nm * r_outer_m / j_polar_m4;
    let bending_stress_pa = physics_scalar(
        "beam_bending_stress",
        &["m", "y", "i"],
        &[bending_moment_nm, r_outer_m, i_annular_m4],
    )
        .unwrap_or(bending_moment_nm * r_outer_m / i_annular_m4);
    let von_mises_pa = (bending_stress_pa.powi(2) + 3.0 * torsion_stress_pa.powi(2)).sqrt();
    let safety_margin = if von_mises_pa > 0.0 { sigma_allow / von_mises_pa } else { f64::INFINITY };

    let shell_volume_m3 = area_annular_m2 * length_m;
    let flange_volume_m3 = area_annular_m2 * wall_m * 2.0;
    let mass_kg = (shell_volume_m3 + flange_volume_m3) * material.density_kg_m3();

    Bellhousing {
        gearbox_kind: kind,
        material,
        flywheel_diameter_mm: (flywheel_d_mm * 10.0).round() / 10.0,
        clutch_diameter_mm: (clutch_d_mm * 10.0).round() / 10.0,
        bolt_circle_diameter_mm: (r_mount_mm * 2.0 * 10.0).round() / 10.0,
        inner_diameter_mm: (inner_d_mm * 10.0).round() / 10.0,
        outer_diameter_mm: (outer_d_mm * 10.0).round() / 10.0,
        wall_thickness_mm: (wall_mm * 100.0).round() / 100.0,
        length_mm,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        design_torque_nm: (design_torque_nm * 10.0).round() / 10.0,
        gearbox_mass_kg: (gearbox_mass_kg * 10.0).round() / 10.0,
        torsion_stress_pa,
        bending_stress_pa,
        von_mises_stress_pa: von_mises_pa,
        safety_margin: (safety_margin * 100.0).round() / 100.0,
    }
}

pub fn assemble_open(engine: &Engine, gearbox: &Gearbox) -> Bellhousing {
    assemble(engine, None, gearbox)
}
