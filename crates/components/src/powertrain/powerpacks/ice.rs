use crate::powertrain::engines::thermals::assemblies::engine::Engine;
use crate::powertrain::transmissions::bellhousings::{
    self, Bellhousing,
};
use crate::powertrain::transmissions::clutches::Clutch;
use crate::powertrain::transmissions::gearboxes::{
    AutomaticGearbox, Cvt, Gearbox, ManualGearbox, RobotizedManual, SequentialGearbox,
};

use super::driveline::{self, Driveline, DrivelineSpec};

#[derive(Debug, Clone)]
pub struct IcePowerpack {
    pub engine: Engine,
    pub clutch: Option<Clutch>,
    pub gearbox: Gearbox,
    pub bellhousing: Bellhousing,
    pub driveline: Option<Driveline>,
}

impl IcePowerpack {
    pub fn new(engine: Engine, clutch: Option<Clutch>, gearbox: Gearbox) -> Self {
        let bellhousing = bellhousings::assemble(&engine, clutch.as_ref(), &gearbox);
        Self { engine, clutch, gearbox, bellhousing, driveline: None }
    }

    pub fn with_driveline(mut self, spec: &DrivelineSpec) -> Self {
        self.driveline = Some(driveline::assemble(spec));
        self
    }

    pub fn assemble_driveline(&mut self, spec: &DrivelineSpec) -> &Driveline {
        self.driveline = Some(driveline::assemble(spec));
        self.driveline.as_ref().unwrap()
    }

    pub fn new_manual(engine: Engine, clutch: Clutch, gearbox: ManualGearbox) -> Self {
        Self::new(engine, Some(clutch), Gearbox::Manual(gearbox))
    }

    pub fn new_sequential(engine: Engine, clutch: Clutch, gearbox: SequentialGearbox) -> Self {
        Self::new(engine, Some(clutch), Gearbox::Sequential(gearbox))
    }

    pub fn new_automatic(engine: Engine, gearbox: AutomaticGearbox) -> Self {
        Self::new(engine, None, Gearbox::Automatic(gearbox))
    }

    pub fn new_dct(engine: Engine, clutch: Clutch, gearbox: AutomaticGearbox) -> Self {
        Self::new(engine, Some(clutch), Gearbox::Automatic(gearbox))
    }

    pub fn new_cvt(engine: Engine, clutch: Option<Clutch>, gearbox: Cvt) -> Self {
        Self::new(engine, clutch, Gearbox::Cvt(gearbox))
    }

    pub fn new_robotized(engine: Engine, clutch: Clutch, gearbox: RobotizedManual) -> Self {
        Self::new(engine, Some(clutch), Gearbox::RobotizedManual(gearbox))
    }

    pub fn total_mass_kg(&self) -> f64 {
        let engine_mass = estimate_engine_mass_kg(&self.engine);
        let clutch_mass = self.clutch.as_ref().map(estimate_clutch_mass_kg).unwrap_or(0.0);
        let gearbox_mass = bellhousings::estimated_gearbox_mass_kg(&self.gearbox);
        let driveline_mass = self.driveline.as_ref().map(|d| d.total_mass_kg).unwrap_or(0.0);
        engine_mass + clutch_mass + gearbox_mass + self.bellhousing.mass_kg + driveline_mass
    }

    pub fn differential_count(&self) -> u8 {
        self.driveline.as_ref().map(|d| d.differential_count).unwrap_or(0)
    }

    pub fn total_length_mm(&self) -> f64 {
        estimate_engine_length_mm(&self.engine) + self.bellhousing.length_mm + gearbox_length_mm(&self.gearbox)
    }

    pub fn peak_power_kw(&self) -> f64 {
        self.engine.estimated_power_kw
    }

    pub fn max_input_torque_nm(&self) -> f64 {
        bellhousings::gearbox_max_input_torque_nm(&self.gearbox)
    }
}

fn estimate_engine_mass_kg(engine: &Engine) -> f64 {
    engine_mass_kg(engine)
}

fn estimate_engine_length_mm(engine: &Engine) -> f64 {
    engine_length_mm(engine)
}

pub fn engine_mass_kg(engine: &Engine) -> f64 {
    let displacement_l = engine.bloc.displacement_l();
    let bloc_mass = displacement_l * 80.0 + 25.0;
    bloc_mass + engine.flywheel.mass_kg
}

pub fn engine_length_mm(engine: &Engine) -> f64 {
    use crate::powertrain::engines::thermals::assemblies::layout::Layout;
    let cyl = engine.bloc.cylinders as f64;
    let bore = engine.bloc.bore_mm;
    let cyl_pitch = bore + 8.0;
    match engine.bloc.layout {
        Layout::Inline => cyl * cyl_pitch + 220.0,
        Layout::V | Layout::Vr | Layout::Flat => (cyl / 2.0) * cyl_pitch + 260.0,
        Layout::W => (cyl / 4.0) * cyl_pitch + 280.0,
        Layout::Radial => bore * 2.5 + 320.0,
    }
}

fn estimate_clutch_mass_kg(clutch: &Clutch) -> f64 {
    match clutch {
        Clutch::Dry(c) => {
            let d_m = c.disc_diameter_mm / 1000.0;
            d_m.powi(2) * 35.0 + 4.0
        }
        Clutch::Wet(c) => {
            let d_m = c.disc_diameter_mm / 1000.0;
            d_m.powi(2) * 35.0 * (c.disc_count as f64) + 6.0
        }
        Clutch::Dual(c) => {
            let t = c.max_torque_nm.max(50.0);
            12.0 + (t / 100.0).sqrt() * 4.0
        }
    }
}

fn gearbox_length_mm(g: &Gearbox) -> f64 {
    match g {
        Gearbox::Manual(_) => 380.0,
        Gearbox::Sequential(_) => 320.0,
        Gearbox::Automatic(_) => 540.0,
        Gearbox::Cvt(_) => 460.0,
        Gearbox::RobotizedManual(_) => 410.0,
    }
}
