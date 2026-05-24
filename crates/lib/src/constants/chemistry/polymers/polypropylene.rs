use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polypropylene",
        abbreviation: "PP",
        monomer_formula: "C3H6",
        monomer_molar_mass: 42.081,
        density_g_cm3: 0.90,
        glass_transition_k: Some(263.0),
        melting_point_k: Some(453.0),
        polymer_type: "thermoplastic",
        category: "polyolefin",
    }
}
