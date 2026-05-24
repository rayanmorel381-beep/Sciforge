#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafSpringLayout {
    Longitudinal,
    Transverse,
}

#[derive(Debug, Clone)]
pub struct LeafSpring {
    pub layout: LeafSpringLayout,
    pub leaf_count: u8,
    pub leaf_width_mm: f64,
    pub parabolic: bool,
}

impl LeafSpring {
    pub fn longitudinal(leaf_count: u8, leaf_width_mm: f64) -> Self {
        Self { layout: LeafSpringLayout::Longitudinal, leaf_count, leaf_width_mm, parabolic: false }
    }

    pub fn parabolic(leaf_width_mm: f64) -> Self {
        Self { layout: LeafSpringLayout::Longitudinal, leaf_count: 2, leaf_width_mm, parabolic: true }
    }

    pub fn transverse(leaf_count: u8, leaf_width_mm: f64) -> Self {
        Self { layout: LeafSpringLayout::Transverse, leaf_count, leaf_width_mm, parabolic: false }
    }
}
