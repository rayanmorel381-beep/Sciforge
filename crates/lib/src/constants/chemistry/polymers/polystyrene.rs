use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polystyrene",
        abbreviation: "PS",
        monomer_formula: "C8H8",
        monomer_molar_mass: 104.150,
        density_g_cm3: 1.05,
        glass_transition_k: Some(373.0),
        melting_point_k: Some(513.0),
        polymer_type: "thermoplastic",
        category: "vinyl",
    }
}
