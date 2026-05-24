use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "La",
        name: "Lanthanum",
        atomic_number: 57,
        atomic_mass: 138.90547_f64,
        electronegativity: Some(1.1_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 5d¹ 6s²",
        isotopes: vec![
        Isotope {
            name: "Lanthanum-137",
            symbol: "¹³⁷La",
            mass_number: 137,
            neutrons: 80,
            atomic_mass: 136.906494_f64,
            half_life: Some(60000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Barium-137"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Lanthanum-138",
            symbol: "¹³⁸La",
            mass_number: 138,
            neutrons: 81,
            atomic_mass: 137.907112_f64,
            half_life: Some(102000000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.655_f64,
                daughter: Some("Barium-138"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.345_f64,
                daughter: Some("Cerium-138"),
            },
            ],
            natural_abundance: 0.00089_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Lanthanum-139",
            symbol: "¹³⁹La",
            mass_number: 139,
            neutrons: 82,
            atomic_mass: 138.906353_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.99911_f64,
            nuclear_spin: Some("7/2+"),
        },
        ],
    }
}
