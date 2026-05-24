use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Bk",
        name: "Berkelium",
        atomic_number: 97,
        atomic_mass: 247.070307_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f⁹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Berkelium-247",
            symbol: "²⁴⁷Bk",
            mass_number: 247,
            neutrons: 150,
            atomic_mass: 247.070307_f64,
            half_life: Some(1380.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Americium-243"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Berkelium-249",
            symbol: "²⁴⁹Bk",
            mass_number: 249,
            neutrons: 152,
            atomic_mass: 249.074987_f64,
            half_life: Some(330.0_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Californium-249"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
