use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Mn",
        name: "Manganese",
        atomic_number: 25,
        atomic_mass: 54.938044_f64,
        electronegativity: Some(1.55_f64),
        group: Some(7),
        period: 4,
        category: "transition metal",
        electron_configuration: "[Ar] 3d⁵ 4s²",
        isotopes: vec![
        Isotope {
            name: "Manganese-52",
            symbol: "⁵²Mn",
            mass_number: 52,
            neutrons: 27,
            atomic_mass: 51.945566_f64,
            half_life: Some(5.591_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+ / electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Chromium-52"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Manganese-53",
            symbol: "⁵³Mn",
            mass_number: 53,
            neutrons: 28,
            atomic_mass: 52.94129_f64,
            half_life: Some(3740000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Chromium-53"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Manganese-54",
            symbol: "⁵⁴Mn",
            mass_number: 54,
            neutrons: 29,
            atomic_mass: 53.940363_f64,
            half_life: Some(312.2_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Chromium-54"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Manganese-55",
            symbol: "⁵⁵Mn",
            mass_number: 55,
            neutrons: 30,
            atomic_mass: 54.938045_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("5/2-"),
        },
        ],
    }
}
