use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polyurethane",
        abbreviation: "PU",
        monomer_formula: "C17H16N2O4",
        monomer_molar_mass: 312.322,
        density_g_cm3: 1.20,
        glass_transition_k: Some(200.0),
        melting_point_k: Some(423.0),
        polymer_type: "thermoset",
        category: "polyurethane",
    }
}
