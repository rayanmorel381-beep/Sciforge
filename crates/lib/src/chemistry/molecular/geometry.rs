pub fn vsepr_angle(bonding_pairs: u32, lone_pairs: u32) -> f64 {
    match (bonding_pairs + lone_pairs, lone_pairs) {
        (2, 0) => 180.0,
        (3, 0) => 120.0,
        (3, 1) => 117.0,
        (4, 0) => 109.5,
        (4, 1) => 107.0,
        (4, 2) => 104.5,
        (5, 0) => 90.0,
        (6, 0) => 90.0,
        _ => 109.5,
    }
}

pub fn hybridization_sp(bonding_regions: u32) -> &'static str {
    match bonding_regions {
        2 => "sp",
        3 => "sp2",
        4 => "sp3",
        5 => "sp3d",
        6 => "sp3d2",
        _ => "unknown",
    }
}

pub fn ideal_gas_moles(p: f64, v: f64, t: f64) -> f64 {
    p * v / (crate::constants::R_GAS * t)
}

pub fn molecular_geometry_name(bonding_pairs: u32, lone_pairs: u32) -> &'static str {
    match (bonding_pairs, lone_pairs) {
        (2, 0) => "linear",
        (3, 0) => "trigonal planar",
        (2, 1) => "bent",
        (4, 0) => "tetrahedral",
        (3, 1) => "trigonal pyramidal",
        (2, 2) => "bent",
        (5, 0) => "trigonal bipyramidal",
        (4, 1) => "seesaw",
        (3, 2) => "t-shaped",
        (2, 3) => "linear",
        (6, 0) => "octahedral",
        (5, 1) => "square pyramidal",
        (4, 2) => "square planar",
        _ => "unknown",
    }
}

pub fn bond_length_estimate(r1: f64, r2: f64) -> f64 {
    r1 + r2
}

pub fn bond_energy_badger(r_e: f64, a: f64, b: f64) -> f64 {
    a / (r_e - b).powi(3).max(1e-30)
}

pub fn coordination_geometry_angles(coordination: u32) -> f64 {
    match coordination {
        2 => 180.0,
        3 => 120.0,
        4 => 109.5,
        5 => 90.0,
        6 => 90.0,
        8 => 70.53,
        _ => 360.0 / coordination.max(1) as f64,
    }
}

pub fn effective_nuclear_charge(z: u32, s: f64) -> f64 {
    z as f64 - s
}
