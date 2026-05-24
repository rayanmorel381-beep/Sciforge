use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "U",
        name: "Uranium",
        atomic_number: 92,
        atomic_mass: 238.02891_f64,
        electronegativity: Some(1.38_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f³ 6d¹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Uranium-233",
            symbol: "²³³U",
            mass_number: 233,
            neutrons: 141,
            atomic_mass: 233.039635_f64,
            half_life: Some(159200.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-229"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Uranium-234",
            symbol: "²³⁴U",
            mass_number: 234,
            neutrons: 142,
            atomic_mass: 234.040952_f64,
            half_life: Some(245500.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-230"),
            },
            ],
            natural_abundance: 5.4e-05_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Uranium-235",
            symbol: "²³⁵U",
            mass_number: 235,
            neutrons: 143,
            atomic_mass: 235.04393_f64,
            half_life: Some(704000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-231"),
            },
            ],
            natural_abundance: 0.007204_f64,
            nuclear_spin: Some("7/2-"),
        },
        Isotope {
            name: "Uranium-236",
            symbol: "²³⁶U",
            mass_number: 236,
            neutrons: 144,
            atomic_mass: 236.045568_f64,
            half_life: Some(23420000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-232"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Uranium-238",
            symbol: "²³⁸U",
            mass_number: 238,
            neutrons: 146,
            atomic_mass: 238.050788_f64,
            half_life: Some(4468000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-234"),
            },
            ],
            natural_abundance: 0.992742_f64,
            nuclear_spin: Some("0+"),
        },
        ],
    }
}
