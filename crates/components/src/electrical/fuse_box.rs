#[derive(Debug, Clone)]
pub struct FuseBox {
    pub fuse_count: u8,
    pub relay_count: u8,
    pub smart_junction: bool,
    pub max_total_current_a: f64,
    pub busbar_rating_a: f64,
    pub mass_kg: f64,
    pub diagnostic_can: bool,
}

fn build(fuse_count: u8, relay_count: u8, smart_junction: bool) -> FuseBox {
    let avg_fuse_rating_a = if smart_junction { 18.0 } else { 12.0 };
    let max_total_current_a = fuse_count as f64 * avg_fuse_rating_a * 0.4;
    let busbar_rating_a = max_total_current_a * 1.5;
    let mass_kg = if smart_junction {
        0.45 + fuse_count as f64 * 0.025 + relay_count as f64 * 0.05 + 0.6
    } else {
        0.35 + fuse_count as f64 * 0.020 + relay_count as f64 * 0.05
    };
    FuseBox {
        fuse_count,
        relay_count,
        smart_junction,
        max_total_current_a: (max_total_current_a * 10.0).round() / 10.0,
        busbar_rating_a: (busbar_rating_a * 10.0).round() / 10.0,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        diagnostic_can: smart_junction,
    }
}

impl FuseBox {
    pub fn standard(fuse_count: u8) -> Self {
        build(fuse_count, 8, false)
    }

    pub fn smart(fuse_count: u8, relay_count: u8) -> Self {
        build(fuse_count, relay_count, true)
    }
}
