use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polyethylene terephthalate",
        abbreviation: "PET",
        monomer_formula: "C10H8O4",
        monomer_molar_mass: 192.169,
        density_g_cm3: 1.38,
        glass_transition_k: Some(343.0),
        melting_point_k: Some(533.0),
        polymer_type: "thermoplastic",
        category: "polyester",
    }
}
