#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WingMount {
    TrunkMounted,
    BodyMounted,
    Pedestal,
    Swan,
}

#[derive(Debug, Clone)]
pub struct Wing {
    pub mount: WingMount,
    pub span_mm: f64,
    pub chord_mm: f64,
    pub angle_of_attack_deg: f64,
    pub dual_element: bool,
    pub adjustable: bool,
}

impl Wing {
    pub fn fixed(mount: WingMount, span_mm: f64, chord_mm: f64, angle_of_attack_deg: f64) -> Self {
        Self { mount, span_mm, chord_mm, angle_of_attack_deg, dual_element: false, adjustable: false }
    }

    pub fn dual_element(mount: WingMount, span_mm: f64, chord_mm: f64) -> Self {
        Self { mount, span_mm, chord_mm, angle_of_attack_deg: 15.0, dual_element: true, adjustable: true }
    }
}
