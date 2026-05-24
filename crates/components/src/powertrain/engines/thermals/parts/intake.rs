use sciforge_core::materials::Material;
use sciforge_core::materials::alus::cast::A356;
use sciforge_core::materials::composites::cfrp::CFRP_T700;
use sciforge_core::materials::plastics::thermoplastics::PA66;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntakeType {
    NaturallyAspirated,
    Turbocharged,
    Supercharged,
    Twincharged,
    TwinTurbo,
    MultiTurbo,
}

#[derive(Debug, Clone)]
pub struct IntakeSystem {
    pub intake_type: IntakeType,
    pub throttle_body_mm: f64,
    pub plenum_volume_l: f64,
    pub runner_length_mm: f64,
    pub variable_geometry: bool,
    pub resonance_chamber: bool,
    pub cold_air_route: bool,
}

impl IntakeSystem {
    pub fn helmholtz_hz(&self, charge_temp_k: f64) -> f64 {
        use crate::sounds::acoustics as ac;
        let c = ac::speed_of_sound_air(charge_temp_k);
        let runner_d_m = self.throttle_body_mm / 1000.0;
        let runner_a_m2 = std::f64::consts::PI * runner_d_m.powi(2) / 4.0;
        let plenum_v_m3 = self.plenum_volume_l * 1.0e-3;
        let runner_l_m = self.runner_length_mm / 1000.0;
        ac::intake_helmholtz_hz(c, runner_a_m2, plenum_v_m3, runner_l_m)
    }

    pub fn manifold_material(&self) -> &'static Material {
        match self.intake_type {
            IntakeType::Turbocharged
            | IntakeType::TwinTurbo
            | IntakeType::MultiTurbo
            | IntakeType::Twincharged => &A356,
            _ if self.cold_air_route => &CFRP_T700,
            _ => &PA66,
        }
    }

    pub fn naturally_aspirated(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::NaturallyAspirated,
            throttle_body_mm: 40.0 + displacement_l * 8.0,
            plenum_volume_l: displacement_l * 0.9,
            runner_length_mm: 320.0,
            variable_geometry: false,
            resonance_chamber: false,
            cold_air_route: false,
        }
    }

    pub fn turbocharged(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::Turbocharged,
            throttle_body_mm: 60.0 + displacement_l * 5.0,
            plenum_volume_l: displacement_l * 1.2,
            runner_length_mm: 180.0,
            variable_geometry: false,
            resonance_chamber: false,
            cold_air_route: true,
        }
    }

    pub fn twin_turbo(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::TwinTurbo,
            throttle_body_mm: 80.0 + displacement_l * 4.0,
            plenum_volume_l: displacement_l * 1.5,
            runner_length_mm: 150.0,
            variable_geometry: true,
            resonance_chamber: false,
            cold_air_route: true,
        }
    }

    pub fn supercharged(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::Supercharged,
            throttle_body_mm: 70.0 + displacement_l * 5.0,
            plenum_volume_l: displacement_l * 1.1,
            runner_length_mm: 200.0,
            variable_geometry: false,
            resonance_chamber: true,
            cold_air_route: false,
        }
    }

    pub fn multi_turbo(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::MultiTurbo,
            throttle_body_mm: 90.0 + displacement_l * 3.0,
            plenum_volume_l: displacement_l * 1.8,
            runner_length_mm: 120.0,
            variable_geometry: true,
            resonance_chamber: false,
            cold_air_route: true,
        }
    }

    pub fn twincharged(displacement_l: f64) -> Self {
        Self {
            intake_type: IntakeType::Twincharged,
            throttle_body_mm: 75.0 + displacement_l * 5.0,
            plenum_volume_l: displacement_l * 1.4,
            runner_length_mm: 160.0,
            variable_geometry: false,
            resonance_chamber: false,
            cold_air_route: true,
        }
    }
}
