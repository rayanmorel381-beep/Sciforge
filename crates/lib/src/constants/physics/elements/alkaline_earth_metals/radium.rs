use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ra",
        name: "Radium",
        atomic_number: 88,
        atomic_mass: 226.0_f64,
        electronegativity: Some(0.9_f64),
        group: Some(2),
        period: 7,
        category: "alkaline earth metal",
        electron_configuration: "[Rn] 7s²",
        isotopes: vec![
        Isotope {
            name: "Radium-223",
            symbol: "²²³Ra",
            mass_number: 223,
            neutrons: 135,
            atomic_mass: 223.0185_f64,
            half_life: Some(11.43_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Radon-219"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Radium-224",
            symbol: "²²⁴Ra",
            mass_number: 224,
            neutrons: 136,
            atomic_mass: 224.02021_f64,
            half_life: Some(3.6319_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Radon-220"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Radium-225",
            symbol: "²²⁵Ra",
            mass_number: 225,
            neutrons: 137,
            atomic_mass: 225.02361_f64,
            half_life: Some(14.9_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta- (electron emission)",
                branching_ratio: 1.0_f64,
                daughter: Some("Actinium-225"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Radium-226",
            symbol: "²²⁶Ra",
            mass_number: 226,
            neutrons: 138,
            atomic_mass: 226.02541_f64,
            half_life: Some(1600.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Radon-222"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Radium-228",
            symbol: "²²⁸Ra",
            mass_number: 228,
            neutrons: 140,
            atomic_mass: 228.03107_f64,
            half_life: Some(5.75_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta- (electron emission)",
                branching_ratio: 1.0_f64,
                daughter: Some("Actinium-228"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
