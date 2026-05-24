#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InletType {
    Pitot,
    SubsonicDiffuser,
    VariableGeometry,
    DivertlessSupersonic,
    AxiSymmetricCone,
}

#[derive(Debug, Clone)]
pub struct TurbineInlet {
    pub inlet_type: InletType,
    pub capture_area_m2: f64,
    pub pressure_recovery: f64,
    pub max_mach: f64,
    pub lip_radius_m: f64,
    pub anti_ice: bool,
}

impl TurbineInlet {
    pub fn pitot(capture_area_m2: f64) -> Self {
        Self {
            inlet_type: InletType::Pitot,
            capture_area_m2,
            pressure_recovery: 0.98,
            max_mach: 0.85,
            lip_radius_m: 0.012,
            anti_ice: true,
        }
    }

    pub fn subsonic_diffuser(capture_area_m2: f64) -> Self {
        Self {
            inlet_type: InletType::SubsonicDiffuser,
            capture_area_m2,
            pressure_recovery: 0.97,
            max_mach: 0.90,
            lip_radius_m: 0.008,
            anti_ice: true,
        }
    }

    pub fn variable_geometry(capture_area_m2: f64, max_mach: f64) -> Self {
        Self {
            inlet_type: InletType::VariableGeometry,
            capture_area_m2,
            pressure_recovery: 0.93,
            max_mach,
            lip_radius_m: 0.005,
            anti_ice: false,
        }
    }

    pub fn divertless_supersonic(capture_area_m2: f64) -> Self {
        Self {
            inlet_type: InletType::DivertlessSupersonic,
            capture_area_m2,
            pressure_recovery: 0.91,
            max_mach: 1.8,
            lip_radius_m: 0.003,
            anti_ice: false,
        }
    }

    pub fn axisymmetric_cone(capture_area_m2: f64, max_mach: f64) -> Self {
        Self {
            inlet_type: InletType::AxiSymmetricCone,
            capture_area_m2,
            pressure_recovery: 0.92,
            max_mach,
            lip_radius_m: 0.004,
            anti_ice: false,
        }
    }

    pub fn ram_pressure_ratio(&self, mach: f64) -> f64 {
        let gamma = 1.4_f64;
        let ideal = (1.0 + (gamma - 1.0) / 2.0 * mach.powi(2)).powf(gamma / (gamma - 1.0));
        ideal * self.pressure_recovery
    }
}
