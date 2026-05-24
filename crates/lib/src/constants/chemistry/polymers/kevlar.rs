use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "kevlar",
        abbreviation: "PPTA",
        monomer_formula: "C14H10N2O2",
        monomer_molar_mass: 238.244,
        density_g_cm3: 1.44,
        glass_transition_k: Some(648.0),
        melting_point_k: Some(773.0),
        polymer_type: "thermoplastic",
        category: "aramid",
    }
}
