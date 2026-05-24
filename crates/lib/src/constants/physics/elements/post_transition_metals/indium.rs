use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "In",
        name: "Indium",
        atomic_number: 49,
        atomic_mass: 114.818_f64,
        electronegativity: Some(1.78_f64),
        group: Some(13),
        period: 5,
        category: "post-transition metal",
        electron_configuration: "[Kr] 4d¹⁰ 5s² 5p¹",
        isotopes: vec![
        Isotope {
            name: "Indium-111",
            symbol: "¹¹¹In",
            mass_number: 111,
            neutrons: 62,
            atomic_mass: 110.905103_f64,
            half_life: Some(2.8047_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Cadmium-111"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Indium-113",
            symbol: "¹¹³In",
            mass_number: 113,
            neutrons: 64,
            atomic_mass: 112.904058_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.0429_f64,
            nuclear_spin: Some("9/2+"),
        },
        Isotope {
            name: "Indium-114m",
            symbol: "¹¹⁴ᵐIn",
            mass_number: 114,
            neutrons: 65,
            atomic_mass: 113.904914_f64,
            half_life: Some(49.51_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "isomeric transition",
                branching_ratio: 0.965_f64,
                daughter: Some("Indium-114"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.035_f64,
                daughter: Some("Cadmium-114"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Indium-115",
            symbol: "¹¹⁵In",
            mass_number: 115,
            neutrons: 66,
            atomic_mass: 114.903878_f64,
            half_life: Some(441000000000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Tin-115"),
            },
            ],
            natural_abundance: 0.9571_f64,
            nuclear_spin: Some("9/2+"),
        },
        ],
    }
}
