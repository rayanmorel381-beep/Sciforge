use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "V",
        name: "Vanadium",
        atomic_number: 23,
        atomic_mass: 50.9415_f64,
        electronegativity: Some(1.63_f64),
        group: Some(5),
        period: 4,
        category: "transition metal",
        electron_configuration: "[Ar] 3d³ 4s²",
        isotopes: vec![
        Isotope {
            name: "Vanadium-48",
            symbol: "⁴⁸V",
            mass_number: 48,
            neutrons: 25,
            atomic_mass: 47.952254_f64,
            half_life: Some(15.9735_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+ / electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Titanium-48"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Vanadium-49",
            symbol: "⁴⁹V",
            mass_number: 49,
            neutrons: 26,
            atomic_mass: 48.948516_f64,
            half_life: Some(330.0_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Titanium-49"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Vanadium-50",
            symbol: "⁵⁰V",
            mass_number: 50,
            neutrons: 27,
            atomic_mass: 49.947159_f64,
            half_life: Some(2.1e+17_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+",
                branching_ratio: 0.83_f64,
                daughter: Some("Titanium-50"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.17_f64,
                daughter: Some("Chromium-50"),
            },
            ],
            natural_abundance: 0.0025_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Vanadium-51",
            symbol: "⁵¹V",
            mass_number: 51,
            neutrons: 28,
            atomic_mass: 50.94396_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.9975_f64,
            nuclear_spin: Some("7/2-"),
        },
        ],
    }
}
