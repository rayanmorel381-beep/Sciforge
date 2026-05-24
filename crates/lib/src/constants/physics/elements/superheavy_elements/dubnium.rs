use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Db",
        name: "Dubnium",
        atomic_number: 105,
        atomic_mass: 268.126_f64,
        electronegativity: None,
        group: Some(5),
        period: 7,
        category: "transition_metal",
        electron_configuration: "[Rn] 5f¹⁴ 6d³ 7s²",
        isotopes: vec![
        Isotope {
            name: "Dubnium-262",
            symbol: "²⁶²Db",
            mass_number: 262,
            neutrons: 157,
            atomic_mass: 262.11407_f64,
            half_life: Some(34.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.67_f64,
                daughter: Some("Lawrencium-258"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.23_f64,
                daughter: None,
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.1_f64,
                daughter: Some("Rutherfordium-262"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Dubnium-268",
            symbol: "²⁶⁸Db",
            mass_number: 268,
            neutrons: 163,
            atomic_mass: 268.126_f64,
            half_life: Some(28.0_f64),
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
        Isotope {
            name: "Dubnium-270",
            symbol: "²⁷⁰Db",
            mass_number: 270,
            neutrons: 165,
            atomic_mass: 270.131_f64,
            half_life: Some(23.15_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.83_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.17_f64,
                daughter: Some("Lawrencium-266"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
