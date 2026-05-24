use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Lr",
        name: "Lawrencium",
        atomic_number: 103,
        atomic_mass: 266.12_f64,
        electronegativity: Some(1.3_f64),
        group: Some(3),
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹⁴ 7s² 7p¹",
        isotopes: vec![
        Isotope {
            name: "Lawrencium-256",
            symbol: "²⁵⁶Lr",
            mass_number: 256,
            neutrons: 153,
            atomic_mass: 256.09863_f64,
            half_life: Some(27.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.8_f64,
                daughter: Some("Mendelevium-252"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.2_f64,
                daughter: Some("Nobelium-256"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Lawrencium-260",
            symbol: "²⁶⁰Lr",
            mass_number: 260,
            neutrons: 157,
            atomic_mass: 260.10551_f64,
            half_life: Some(2.7_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.75_f64,
                daughter: Some("Mendelevium-256"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.15_f64,
                daughter: Some("Nobelium-260"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.1_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Lawrencium-262",
            symbol: "²⁶²Lr",
            mass_number: 262,
            neutrons: 159,
            atomic_mass: 262.10961_f64,
            half_life: Some(3.6_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Nobelium-262"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Lawrencium-266",
            symbol: "²⁶⁶Lr",
            mass_number: 266,
            neutrons: 163,
            atomic_mass: 266.12_f64,
            half_life: Some(11.0_f64),
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
