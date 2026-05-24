#[derive(Debug, Clone)]
pub struct DaytimeRunning {
    pub led: bool,
    pub integrated: bool,
    pub adaptive: bool,
}

impl DaytimeRunning {
    pub fn standard() -> Self {
        Self { led: true, integrated: false, adaptive: false }
    }

    pub fn integrated() -> Self {
        Self { led: true, integrated: true, adaptive: false }
    }

    pub fn adaptive() -> Self {
        Self { led: true, integrated: true, adaptive: true }
    }
}
