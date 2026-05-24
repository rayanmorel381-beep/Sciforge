use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polytetrafluoroethylene",
        abbreviation: "PTFE",
        monomer_formula: "C2F4",
        monomer_molar_mass: 100.015,
        density_g_cm3: 2.20,
        glass_transition_k: Some(388.0),
        melting_point_k: Some(600.0),
        polymer_type: "thermoplastic",
        category: "fluoropolymer",
    }
}
