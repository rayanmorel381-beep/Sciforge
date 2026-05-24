use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Np",
        name: "Neptunium",
        atomic_number: 93,
        atomic_mass: 237.048174_f64,
        electronegativity: Some(1.36_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f⁴ 6d¹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Neptunium-235",
            symbol: "²³⁵Np",
            mass_number: 235,
            neutrons: 142,
            atomic_mass: 235.044063_f64,
            half_life: Some(396.1_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Uranium-235"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Neptunium-236",
            symbol: "²³⁶Np",
            mass_number: 236,
            neutrons: 143,
            atomic_mass: 236.04657_f64,
            half_life: Some(154000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.874_f64,
                daughter: Some("Uranium-236"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.126_f64,
                daughter: Some("Plutonium-236"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Neptunium-237",
            symbol: "²³⁷Np",
            mass_number: 237,
            neutrons: 144,
            atomic_mass: 237.048174_f64,
            half_life: Some(2144000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Protactinium-233"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Neptunium-239",
            symbol: "²³⁹Np",
            mass_number: 239,
            neutrons: 146,
            atomic_mass: 239.052939_f64,
            half_life: Some(2.356_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Plutonium-239"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
