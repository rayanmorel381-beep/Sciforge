use std::fmt;

pub const PROTON_MASS: f64 = crate::constants::PROTON_MASS_AMU;
pub const NEUTRON_MASS: f64 = crate::constants::NEUTRON_MASS_AMU;
pub const ELECTRON_MASS: f64 = crate::constants::ELECTRON_MASS_AMU;
pub const AMU_TO_MEV: f64 = crate::constants::AMU_TO_MEV;
pub const BOLTZMANN_KEV: f64 = crate::constants::KELVIN_TO_KEV;

#[derive(Clone, Debug)]
pub struct Nuclide {
    pub z: u32,
    pub a: u32,
    pub mass_excess_mev: f64,
    pub binding_energy_per_nucleon: f64,
    pub half_life_seconds: Option<f64>,
    pub name: &'static str,
}

impl Nuclide {
    pub fn n(&self) -> u32 {
        self.a - self.z
    }

    pub fn atomic_mass_amu(&self) -> f64 {
        self.z as f64 * PROTON_MASS + self.n() as f64 * NEUTRON_MASS
            - self.binding_energy() / AMU_TO_MEV
    }

    pub fn binding_energy(&self) -> f64 {
        self.binding_energy_per_nucleon * self.a as f64
    }

    pub fn is_stable(&self) -> bool {
        self.half_life_seconds.is_none()
    }

    pub fn decay_constant(&self) -> Option<f64> {
        self.half_life_seconds.map(|t| core::f64::consts::LN_2 / t)
    }

    pub fn activity_bq(&self, num_atoms: f64) -> Option<f64> {
        self.decay_constant().map(|l| l * num_atoms)
    }
}

impl fmt::Display for Nuclide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}(Z={}, A={}, B/A={:.3} MeV)",
            self.name, self.z, self.a, self.binding_energy_per_nucleon
        )
    }
}

pub fn semi_empirical_mass(z: u32, a: u32) -> f64 {
    let a_f = a as f64;
    let z_f = z as f64;
    let n_f = (a - z) as f64;

    let a_v = 15.56;
    let a_s = 17.23;
    let a_c = 0.7;
    let a_a = 23.285;
    let a_p = 12.0;

    let volume = a_v * a_f;
    let surface = -a_s * a_f.powf(2.0 / 3.0);
    let coulomb = -a_c * z_f * (z_f - 1.0) / a_f.powf(1.0 / 3.0);
    let asymmetry = -a_a * (n_f - z_f).powi(2) / a_f;
    let pairing = if !a.is_multiple_of(2) {
        0.0
    } else if z.is_multiple_of(2) {
        a_p / a_f.powf(0.5)
    } else {
        -a_p / a_f.powf(0.5)
    };

    volume + surface + coulomb + asymmetry + pairing
}

pub fn binding_energy_per_nucleon_semf(z: u32, a: u32) -> f64 {
    semi_empirical_mass(z, a) / a as f64
}

pub fn nuclear_radius_fm(a: u32) -> f64 {
    1.2 * (a as f64).powf(1.0 / 3.0)
}

pub fn nuclear_density_kg_m3() -> f64 {
    2.3e17
}

pub fn separation_energy_proton(z: u32, a: u32) -> f64 {
    if z == 0 || a <= 1 {
        return 0.0;
    }
    semi_empirical_mass(z - 1, a - 1) + 7.289 - semi_empirical_mass(z, a)
}

pub fn separation_energy_neutron(z: u32, a: u32) -> f64 {
    if a <= 1 {
        return 0.0;
    }
    semi_empirical_mass(z, a - 1) + 8.071 - semi_empirical_mass(z, a)
}

pub fn separation_energy_alpha(z: u32, a: u32) -> f64 {
    if z < 2 || a < 4 {
        return 0.0;
    }
    semi_empirical_mass(z - 2, a - 4) + 28.296 - semi_empirical_mass(z, a)
}

pub fn valley_of_stability_z(a: u32) -> f64 {
    let a_f = a as f64;
    a_f / (2.0 + 0.0155 * a_f.powf(2.0 / 3.0))
}

pub fn neutron_excess(z: u32, a: u32) -> i32 {
    a as i32 - 2 * z as i32
}

pub fn isospin_z(z: u32, a: u32) -> f64 {
    (a as f64 - 2.0 * z as f64) / 2.0
}

pub fn liquid_drop_fission_parameter(z: u32, a: u32) -> f64 {
    let z_f = z as f64;
    let a_f = a as f64;
    z_f * z_f / (50.88 * a_f)
}

pub fn fission_barrier_estimate_mev(z: u32, a: u32) -> f64 {
    let x = liquid_drop_fission_parameter(z, a);
    if x >= 1.0 {
        return 0.0;
    }
    0.38 * (0.75 - x) * semi_empirical_mass(z, a).abs()
}

pub fn nuclear_skin_thickness_fm(z: u32, a: u32) -> f64 {
    let n = a - z;
    0.9 * (n as f64 - z as f64) / a as f64
}

pub fn weizsacker_mass_excess_mev(z: u32, a: u32) -> f64 {
    let z_f = z as f64;
    let n_f = (a - z) as f64;
    z_f * 7.289 + n_f * 8.071 - semi_empirical_mass(z, a)
}

pub fn proton_drip_line_a(z: u32) -> u32 {
    let mut best_a = 2 * z;
    let mut best_sep = separation_energy_proton(z, best_a);
    for a in (z + 1)..=(3 * z) {
        let sep = separation_energy_proton(z, a);
        if sep > best_sep {
            best_sep = sep;
            best_a = a;
        }
    }
    best_a
}

pub fn neutron_drip_line_a(z: u32) -> u32 {
    let mut a = 2 * z;
    loop {
        let sep = separation_energy_neutron(z, a);
        if sep < 0.0 || a > 4 * z + 50 {
            return a;
        }
        a += 1;
    }
}

pub fn magic_number_nearest(n: u32) -> u32 {
    let magic = [2, 8, 20, 28, 50, 82, 126, 184];
    let mut best = magic[0];
    let mut best_dist = n.abs_diff(best);
    for &m in &magic[1..] {
        let d = n.abs_diff(m);
        if d < best_dist {
            best_dist = d;
            best = m;
        }
    }
    best
}

pub fn is_doubly_magic(z: u32, n: u32) -> bool {
    let magic = [2, 8, 20, 28, 50, 82, 126];
    magic.contains(&z) && magic.contains(&n)
}
