use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Pm",
        name: "Promethium",
        atomic_number: 61,
        atomic_mass: 144.912749_f64,
        electronegativity: Some(1.13_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f⁵ 6s²",
        isotopes: vec![
        Isotope {
            name: "Promethium-143",
            symbol: "¹⁴³Pm",
            mass_number: 143,
            neutrons: 82,
            atomic_mass: 142.910933_f64,
            half_life: Some(265.0_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Neodymium-143"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Promethium-145",
            symbol: "¹⁴⁵Pm",
            mass_number: 145,
            neutrons: 84,
            atomic_mass: 144.912749_f64,
            half_life: Some(17.7_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Neodymium-145"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Promethium-146",
            symbol: "¹⁴⁶Pm",
            mass_number: 146,
            neutrons: 85,
            atomic_mass: 145.914696_f64,
            half_life: Some(5.53_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.66_f64,
                daughter: Some("Neodymium-146"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.34_f64,
                daughter: Some("Samarium-146"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Promethium-147",
            symbol: "¹⁴⁷Pm",
            mass_number: 147,
            neutrons: 86,
            atomic_mass: 146.915139_f64,
            half_life: Some(2.6234_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Samarium-147"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Promethium-148",
            symbol: "¹⁴⁸Pm",
            mass_number: 148,
            neutrons: 87,
            atomic_mass: 147.917475_f64,
            half_life: Some(5.368_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Samarium-148"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
