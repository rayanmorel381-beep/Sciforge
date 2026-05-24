use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Fr",
        name: "Francium",
        atomic_number: 87,
        atomic_mass: 223.0_f64,
        electronegativity: Some(0.7_f64),
        group: Some(1),
        period: 7,
        category: "alkali metal",
        electron_configuration: "[Rn] 7s¹",
        isotopes: vec![
        Isotope {
            name: "Francium-208",
            symbol: "²⁰⁸Fr",
            mass_number: 208,
            neutrons: 121,
            atomic_mass: 208.0036_f64,
            half_life: Some(59.1_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 0.89_f64,
                daughter: None,
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.11_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Francium-210",
            symbol: "²¹⁰Fr",
            mass_number: 210,
            neutrons: 123,
            atomic_mass: 210.00339_f64,
            half_life: Some(3.18_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 0.6_f64,
                daughter: None,
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.4_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Francium-212",
            symbol: "²¹²Fr",
            mass_number: 212,
            neutrons: 125,
            atomic_mass: 212.00462_f64,
            half_life: Some(20.0_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.57_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 0.43_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Francium-221",
            symbol: "²²¹Fr",
            mass_number: 221,
            neutrons: 134,
            atomic_mass: 221.01426_f64,
            half_life: Some(4.9_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Astatine-217"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Francium-223",
            symbol: "²²³Fr",
            mass_number: 223,
            neutrons: 136,
            atomic_mass: 223.01974_f64,
            half_life: Some(22.0_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta- (electron emission)",
                branching_ratio: 0.99994_f64,
                daughter: Some("Radium-223"),
            },
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 6e-05_f64,
                daughter: Some("Astatine-219"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
