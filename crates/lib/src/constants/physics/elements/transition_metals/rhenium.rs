use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Re",
        name: "Rhenium",
        atomic_number: 75,
        atomic_mass: 186.207_f64,
        electronegativity: Some(1.9_f64),
        group: Some(7),
        period: 6,
        category: "transition metal",
        electron_configuration: "[Xe] 4f¹⁴ 5d⁵ 6s²",
        isotopes: vec![
        Isotope {
            name: "Rhenium-185",
            symbol: "¹⁸⁵Re",
            mass_number: 185,
            neutrons: 110,
            atomic_mass: 184.952955_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.374_f64,
            nuclear_spin: Some("5/2+"),
        },
        Isotope {
            name: "Rhenium-186",
            symbol: "¹⁸⁶Re",
            mass_number: 186,
            neutrons: 111,
            atomic_mass: 185.954986_f64,
            half_life: Some(3.7186_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.927_f64,
                daughter: Some("Osmium-186"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.073_f64,
                daughter: Some("Tungsten-186"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Rhenium-187",
            symbol: "¹⁸⁷Re",
            mass_number: 187,
            neutrons: 112,
            atomic_mass: 186.955753_f64,
            half_life: Some(41200000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Osmium-187"),
            },
            ],
            natural_abundance: 0.626_f64,
            nuclear_spin: Some("5/2+"),
        },
        Isotope {
            name: "Rhenium-188",
            symbol: "¹⁸⁸Re",
            mass_number: 188,
            neutrons: 113,
            atomic_mass: 187.958114_f64,
            half_life: Some(17.003_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Osmium-188"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
