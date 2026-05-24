use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "poly(methyl methacrylate)",
        abbreviation: "PMMA",
        monomer_formula: "C5H8O2",
        monomer_molar_mass: 100.117,
        density_g_cm3: 1.18,
        glass_transition_k: Some(378.0),
        melting_point_k: Some(433.0),
        polymer_type: "thermoplastic",
        category: "acrylic",
    }
}
