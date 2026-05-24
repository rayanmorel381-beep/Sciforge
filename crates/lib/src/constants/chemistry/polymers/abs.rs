use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "acrylonitrile butadiene styrene",
        abbreviation: "ABS",
        monomer_formula: "C15H17N",
        monomer_molar_mass: 211.302,
        density_g_cm3: 1.05,
        glass_transition_k: Some(378.0),
        melting_point_k: None,
        polymer_type: "thermoplastic",
        category: "copolymer",
    }
}
