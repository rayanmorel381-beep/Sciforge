#[derive(Debug, Clone)]
pub struct Cassette {
    pub sprocket_count: u8,
    pub smallest_teeth: u8,
    pub largest_teeth: u8,
    pub freehub_standard: FreehubStandard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreehubStandard {
    Shimano,
    Campagnolo,
    Sram,
    MicroSpline,
    XdDriver,
}

impl Cassette {
    pub fn new(sprocket_count: u8, smallest_teeth: u8, largest_teeth: u8, freehub_standard: FreehubStandard) -> Self {
        Self { sprocket_count, smallest_teeth, largest_teeth, freehub_standard }
    }
}
