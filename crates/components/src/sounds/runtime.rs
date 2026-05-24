use crate::powertrain::engines::thermals::assemblies::engine::Engine;
use crate::powertrain::engines::thermals::parts::cylinders::BankArrangement;
use super::acoustics::{
    self as ac, BoostKind,
};
use super::{LayoutKind, VoiceLoudness};

#[derive(Debug, Clone, Copy)]
pub struct EngineRuntimeState {
    pub rpm: u32,
    pub throttle_0_1: f64,
}

impl EngineRuntimeState {
    pub fn idle(engine: &Engine) -> Self {
        let layout = layout_of(engine);
        let liters = displacement_liters(engine);
        Self {
            rpm: ac::idle_rpm(liters, layout),
            throttle_0_1: 0.05,
        }
    }

    pub fn cruise(engine: &Engine) -> Self {
        Self {
            rpm: (engine.rpm_max as u32 / 3).max(1500),
            throttle_0_1: 0.30,
        }
    }

    pub fn wide_open(engine: &Engine) -> Self {
        Self {
            rpm: engine.rpm_max as u32,
            throttle_0_1: 1.0,
        }
    }

    pub fn current_egt_k(&self, engine: &Engine) -> f64 {
        ac::dynamic_egt_k(engine.egt_k, self.rpm, engine.rpm_max as u32, self.throttle_0_1)
    }

    pub fn current_charge_temp_k(&self, engine: &Engine) -> f64 {
        let boost = boost_of(engine);
        ac::dynamic_charge_temp_k(
            engine.charge_temp_k,
            self.rpm,
            engine.rpm_max as u32,
            self.throttle_0_1,
            boost,
        )
    }

    pub fn fundamental_hz(&self, engine: &Engine) -> f64 {
        ac::fundamental_hz(self.rpm, engine.cylinders.cylinder_count)
    }

    pub fn exhaust_spl_at_source_db(&self, engine: &Engine, voice: VoiceLoudness) -> f64 {
        let liters = displacement_liters(engine);
        let egt_k = self.current_egt_k(engine);
        let exhaust_port_d_m = (engine.cylinders.bore_mm * 0.35) / 1000.0;
        ac::exhaust_loudness_db(
            liters,
            voice,
            egt_k,
            self.rpm,
            engine.cylinders.cylinder_count,
            exhaust_port_d_m,
        )
    }

    pub fn exhaust_spl_at_tail_db(&self, engine: &Engine, voice: VoiceLoudness) -> f64 {
        let source = self.exhaust_spl_at_source_db(engine, voice);
        let egt_k = self.current_egt_k(engine);
        engine.exhaust.tail_pipe_loudness_db(
            source,
            self.rpm,
            engine.cylinders.cylinder_count,
            egt_k,
        )
    }

    pub fn induction_spl_db(&self, engine: &Engine) -> f64 {
        let liters = displacement_liters(engine);
        let intake_d_m = engine.intake.throttle_body_mm / 1000.0;
        let charge_k = self.current_charge_temp_k(engine);
        ac::induction_loudness_db(
            liters,
            self.rpm,
            engine.cylinders.cylinder_count,
            intake_d_m,
            charge_k,
        )
    }

    pub fn intake_resonance_hz(&self, engine: &Engine) -> f64 {
        let charge_k = self.current_charge_temp_k(engine);
        engine.intake.helmholtz_hz(charge_k)
    }

    pub fn harmonic_richness(&self, engine: &Engine) -> f64 {
        let liters = displacement_liters(engine);
        let egt_k = self.current_egt_k(engine);
        ac::harmonic_richness(engine.cylinders.head_material(), egt_k, liters)
    }

    pub fn turbo_spool_hz(&self, engine: &Engine) -> Option<f64> {
        engine.turbo.as_ref().map(|t| {
            let inducer_mm = compressor_inducer_mm(engine);
            let twin = matches!(engine.cylinders.cylinder_count, c if c >= 6) && engine.turbo.is_some();
            let _ = t;
            ac::turbo_spool_hz(self.rpm, inducer_mm, twin)
        })
    }

    pub fn supercharger_whine_hz(&self, engine: &Engine) -> Option<f64> {
        engine.supercharger.as_ref().map(|_sc| {
            let liters = displacement_liters(engine);
            let drive_ratio = 2.1 + (liters * 0.05).min(0.5);
            let lobes = if liters > 4.0 { 4 } else { 3 };
            ac::supercharger_whine_hz(self.rpm, drive_ratio, lobes)
        })
    }
}

pub fn displacement_liters(engine: &Engine) -> f64 {
    let bore_m = engine.cylinders.bore_mm / 1000.0;
    let stroke_m = engine.crankshaft.stroke_mm / 1000.0;
    let cyl_vol = std::f64::consts::PI * bore_m.powi(2) / 4.0 * stroke_m;
    cyl_vol * engine.cylinders.cylinder_count as f64 * 1000.0
}

pub fn layout_of(engine: &Engine) -> LayoutKind {
    match engine.cylinders.bank_arrangement {
        BankArrangement::SingleBank => LayoutKind::Inline,
        BankArrangement::QuadBank => {
            if engine.bank_angle_deg >= 170.0 {
                LayoutKind::H
            } else if engine.bank_angle_deg >= 80.0 {
                LayoutKind::X
            } else {
                LayoutKind::W
            }
        }
        BankArrangement::DualBank => {
            if engine.bank_angle_deg <= 30.0 {
                LayoutKind::Vr
            } else if engine.bank_angle_deg >= 170.0 {
                LayoutKind::Flat
            } else {
                LayoutKind::V
            }
        }
    }
}

pub fn boost_of(engine: &Engine) -> BoostKind {
    if engine.supercharger.is_some() {
        if engine.cylinders.cylinder_count >= 8 {
            BoostKind::SuperchargerHowl
        } else {
            BoostKind::SuperchargerWhine
        }
    } else if engine.turbo.is_some() {
        if engine.cylinders.cylinder_count >= 6 {
            BoostKind::TwinTurbo
        } else {
            BoostKind::Turbo
        }
    } else {
        BoostKind::Natural
    }
}

fn compressor_inducer_mm(engine: &Engine) -> f64 {
    30.0 + displacement_liters(engine) * 12.0
}
