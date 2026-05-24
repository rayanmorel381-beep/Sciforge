use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polyvinyl chloride",
        abbreviation: "PVC",
        monomer_formula: "C2H3Cl",
        monomer_molar_mass: 62.498,
        density_g_cm3: 1.40,
        glass_transition_k: Some(358.0),
        melting_point_k: Some(453.0),
        polymer_type: "thermoplastic",
        category: "vinyl",
    }
}
