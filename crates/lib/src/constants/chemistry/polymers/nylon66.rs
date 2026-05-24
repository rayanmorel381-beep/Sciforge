use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "nylon 6,6",
        abbreviation: "PA66",
        monomer_formula: "C12H22N2O2",
        monomer_molar_mass: 226.318,
        density_g_cm3: 1.14,
        glass_transition_k: Some(323.0),
        melting_point_k: Some(537.0),
        polymer_type: "thermoplastic",
        category: "polyamide",
    }
}
