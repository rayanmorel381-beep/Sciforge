#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NozzleType {
    Converging,
    ConvergingDiverging,
    Vectoring,
    Ejector,
}

#[derive(Debug, Clone)]
pub struct Nozzle {
    pub nozzle_type: NozzleType,
    pub throat_area_m2: f64,
    pub exit_area_m2: f64,
    pub max_temp_k: f64,
    pub vectoring_range_deg: f64,
    pub variable_geometry: bool,
}

impl Nozzle {
    pub fn converging(throat_area_m2: f64, max_temp_k: f64) -> Self {
        Self {
            nozzle_type: NozzleType::Converging,
            throat_area_m2,
            exit_area_m2: throat_area_m2,
            max_temp_k,
            vectoring_range_deg: 0.0,
            variable_geometry: false,
        }
    }

    pub fn converging_diverging(throat_area_m2: f64, exit_area_m2: f64, max_temp_k: f64) -> Self {
        Self {
            nozzle_type: NozzleType::ConvergingDiverging,
            throat_area_m2,
            exit_area_m2,
            max_temp_k,
            vectoring_range_deg: 0.0,
            variable_geometry: true,
        }
    }

    pub fn vectoring(throat_area_m2: f64, exit_area_m2: f64, vectoring_range_deg: f64) -> Self {
        Self {
            nozzle_type: NozzleType::Vectoring,
            throat_area_m2,
            exit_area_m2,
            max_temp_k: 2_200.0,
            vectoring_range_deg,
            variable_geometry: true,
        }
    }

    pub fn expansion_ratio(&self) -> f64 {
        if self.throat_area_m2 > 0.0 {
            self.exit_area_m2 / self.throat_area_m2
        } else {
            1.0
        }
    }
}
