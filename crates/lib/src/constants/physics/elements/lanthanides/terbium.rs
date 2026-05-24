use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Tb",
        name: "Terbium",
        atomic_number: 65,
        atomic_mass: 158.92535_f64,
        electronegativity: Some(1.2_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f⁹ 6s²",
        isotopes: vec![
        Isotope {
            name: "Terbium-157",
            symbol: "¹⁵⁷Tb",
            mass_number: 157,
            neutrons: 92,
            atomic_mass: 156.924025_f64,
            half_life: Some(71.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Gadolinium-157"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Terbium-158",
            symbol: "¹⁵⁸Tb",
            mass_number: 158,
            neutrons: 93,
            atomic_mass: 157.925413_f64,
            half_life: Some(180.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+",
                branching_ratio: 0.833_f64,
                daughter: Some("Gadolinium-158"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.167_f64,
                daughter: Some("Dysprosium-158"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Terbium-159",
            symbol: "¹⁵⁹Tb",
            mass_number: 159,
            neutrons: 94,
            atomic_mass: 158.925347_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("3/2+"),
        },
        Isotope {
            name: "Terbium-161",
            symbol: "¹⁶¹Tb",
            mass_number: 161,
            neutrons: 96,
            atomic_mass: 160.92757_f64,
            half_life: Some(6.89_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Dysprosium-161"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
