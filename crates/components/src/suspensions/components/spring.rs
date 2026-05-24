#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringType {
    Coil,
    Torsion,
    Air,
    Rubber,
}

#[derive(Debug, Clone)]
pub struct Spring {
    pub spring_type: SpringType,
    pub rate_n_per_mm: f64,
    pub free_length_mm: f64,
    pub progressive: bool,
}

impl Spring {
    pub fn coil(rate_n_per_mm: f64, free_length_mm: f64) -> Self {
        Self { spring_type: SpringType::Coil, rate_n_per_mm, free_length_mm, progressive: false }
    }

    pub fn progressive_coil(rate_n_per_mm: f64, free_length_mm: f64) -> Self {
        Self { spring_type: SpringType::Coil, rate_n_per_mm, free_length_mm, progressive: true }
    }

    pub fn air(rate_n_per_mm: f64) -> Self {
        Self { spring_type: SpringType::Air, rate_n_per_mm, free_length_mm: 0.0, progressive: true }
    }
}
