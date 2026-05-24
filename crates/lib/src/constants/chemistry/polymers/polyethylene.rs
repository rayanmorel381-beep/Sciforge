use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polyethylene",
        abbreviation: "PE",
        monomer_formula: "C2H4",
        monomer_molar_mass: 28.054,
        density_g_cm3: 0.94,
        glass_transition_k: Some(148.0),
        melting_point_k: Some(408.0),
        polymer_type: "thermoplastic",
        category: "polyolefin",
    }
}
