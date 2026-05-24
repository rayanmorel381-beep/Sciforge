use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Tm",
        name: "Thulium",
        atomic_number: 69,
        atomic_mass: 168.93422_f64,
        electronegativity: Some(1.25_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f¹³ 6s²",
        isotopes: vec![
        Isotope {
            name: "Thulium-167",
            symbol: "¹⁶⁷Tm",
            mass_number: 167,
            neutrons: 98,
            atomic_mass: 166.932852_f64,
            half_life: Some(9.25_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Erbium-167"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Thulium-169",
            symbol: "¹⁶⁹Tm",
            mass_number: 169,
            neutrons: 100,
            atomic_mass: 168.934213_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("1/2+"),
        },
        Isotope {
            name: "Thulium-170",
            symbol: "¹⁷⁰Tm",
            mass_number: 170,
            neutrons: 101,
            atomic_mass: 169.935801_f64,
            half_life: Some(128.6_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.9986_f64,
                daughter: Some("Ytterbium-170"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.0014_f64,
                daughter: Some("Erbium-170"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Thulium-171",
            symbol: "¹⁷¹Tm",
            mass_number: 171,
            neutrons: 102,
            atomic_mass: 170.936429_f64,
            half_life: Some(1.92_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Ytterbium-171"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
