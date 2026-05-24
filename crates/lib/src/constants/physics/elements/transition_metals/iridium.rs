use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ir",
        name: "Iridium",
        atomic_number: 77,
        atomic_mass: 192.217_f64,
        electronegativity: Some(2.2_f64),
        group: Some(9),
        period: 6,
        category: "transition metal",
        electron_configuration: "[Xe] 4f¹⁴ 5d⁷ 6s²",
        isotopes: vec![
        Isotope {
            name: "Iridium-191",
            symbol: "¹⁹¹Ir",
            mass_number: 191,
            neutrons: 114,
            atomic_mass: 190.960594_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.373_f64,
            nuclear_spin: Some("3/2+"),
        },
        Isotope {
            name: "Iridium-192",
            symbol: "¹⁹²Ir",
            mass_number: 192,
            neutrons: 115,
            atomic_mass: 191.962605_f64,
            half_life: Some(73.827_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.953_f64,
                daughter: Some("Platinum-192"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.047_f64,
                daughter: Some("Osmium-192"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Iridium-193",
            symbol: "¹⁹³Ir",
            mass_number: 193,
            neutrons: 116,
            atomic_mass: 192.962926_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.627_f64,
            nuclear_spin: Some("3/2+"),
        },
        ],
    }
}
