use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polycarbonate",
        abbreviation: "PC",
        monomer_formula: "C16H14O3",
        monomer_molar_mass: 254.281,
        density_g_cm3: 1.20,
        glass_transition_k: Some(423.0),
        melting_point_k: Some(533.0),
        polymer_type: "thermoplastic",
        category: "polycarbonate",
    }
}
