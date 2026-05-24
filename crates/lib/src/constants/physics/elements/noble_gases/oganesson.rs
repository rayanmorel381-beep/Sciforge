use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Og",
        name: "Oganesson",
        atomic_number: 118,
        atomic_mass: 294.0_f64,
        electronegativity: None,
        group: Some(18),
        period: 7,
        category: "noble gas (predicted)",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁶ (predicted)",
        isotopes: vec![
        Isotope {
            name: "Oganesson-294",
            symbol: "²⁹⁴Og",
            mass_number: 294,
            neutrons: 176,
            atomic_mass: 294.21392_f64,
            half_life: Some(0.69_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Livermorium-290"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
