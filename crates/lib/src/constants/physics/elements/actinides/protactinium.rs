use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Pa",
        name: "Protactinium",
        atomic_number: 91,
        atomic_mass: 231.03588_f64,
        electronegativity: Some(1.5_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f² 6d¹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Protactinium-231",
            symbol: "²³¹Pa",
            mass_number: 231,
            neutrons: 140,
            atomic_mass: 231.035884_f64,
            half_life: Some(32760.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Actinium-227"),
            },
            ],
            natural_abundance: 1.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Protactinium-233",
            symbol: "²³³Pa",
            mass_number: 233,
            neutrons: 142,
            atomic_mass: 233.040247_f64,
            half_life: Some(26.975_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Uranium-233"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Protactinium-234",
            symbol: "²³⁴Pa",
            mass_number: 234,
            neutrons: 143,
            atomic_mass: 234.043308_f64,
            half_life: Some(6.7_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Uranium-234"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
