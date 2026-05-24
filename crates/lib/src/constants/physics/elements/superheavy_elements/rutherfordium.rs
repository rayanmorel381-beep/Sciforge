use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Rf",
        name: "Rutherfordium",
        atomic_number: 104,
        atomic_mass: 267.122_f64,
        electronegativity: None,
        group: Some(4),
        period: 7,
        category: "transition_metal",
        electron_configuration: "[Rn] 5f¹⁴ 6d² 7s²",
        isotopes: vec![
        Isotope {
            name: "Rutherfordium-261",
            symbol: "²⁶¹Rf",
            mass_number: 261,
            neutrons: 157,
            atomic_mass: 261.10877_f64,
            half_life: Some(68.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.76_f64,
                daughter: Some("Nobelium-257"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.14_f64,
                daughter: None,
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.1_f64,
                daughter: Some("Lawrencium-261"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Rutherfordium-263",
            symbol: "²⁶³Rf",
            mass_number: 263,
            neutrons: 159,
            atomic_mass: 263.11249_f64,
            half_life: Some(11.0_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.7_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.3_f64,
                daughter: Some("Nobelium-259"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Rutherfordium-267",
            symbol: "²⁶⁷Rf",
            mass_number: 267,
            neutrons: 163,
            atomic_mass: 267.122_f64,
            half_life: Some(1.3_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 1.0_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
