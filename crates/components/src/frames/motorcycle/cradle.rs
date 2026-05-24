#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CradleLayout {
    Single,
    Double,
    Semi,
}

#[derive(Debug, Clone)]
pub struct CradleFrame {
    pub layout: CradleLayout,
    pub tube_diameter_mm: f64,
    pub weight_kg: f64,
    pub down_tube_split: bool,
}

impl CradleFrame {
    pub fn single(tube_diameter_mm: f64, weight_kg: f64) -> Self {
        Self { layout: CradleLayout::Single, tube_diameter_mm, weight_kg, down_tube_split: false }
    }

    pub fn double(tube_diameter_mm: f64, weight_kg: f64) -> Self {
        Self { layout: CradleLayout::Double, tube_diameter_mm, weight_kg, down_tube_split: true }
    }

    pub fn semi(tube_diameter_mm: f64, weight_kg: f64) -> Self {
        Self { layout: CradleLayout::Semi, tube_diameter_mm, weight_kg, down_tube_split: false }
    }
}
