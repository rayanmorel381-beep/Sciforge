#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAdjust {
    Fixed,
    Tilt,
    TiltAndTelescope,
    FullyElectric,
}

#[derive(Debug, Clone)]
pub struct SteeringColumn {
    pub adjust: ColumnAdjust,
    pub collapsible: bool,
    pub airbag_mount: bool,
}

impl SteeringColumn {
    pub fn fixed() -> Self {
        Self { adjust: ColumnAdjust::Fixed, collapsible: true, airbag_mount: true }
    }

    pub fn adjustable(adjust: ColumnAdjust) -> Self {
        Self { adjust, collapsible: true, airbag_mount: true }
    }
}
