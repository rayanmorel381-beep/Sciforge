#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchConfig {
    Two,
    Three,
    Split60_40,
    Split40_20_40,
}

#[derive(Debug, Clone)]
pub struct BenchSeat {
    pub config: BenchConfig,
    pub foldable: bool,
    pub heated: bool,
}

impl BenchSeat {
    pub fn standard(config: BenchConfig) -> Self {
        Self { config, foldable: false, heated: false }
    }

    pub fn foldable(config: BenchConfig) -> Self {
        Self { config, foldable: true, heated: false }
    }

    pub fn heated(config: BenchConfig) -> Self {
        Self { config, foldable: false, heated: true }
    }
}
