use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "bakelite",
        abbreviation: "PF",
        monomer_formula: "C15H12O2",
        monomer_molar_mass: 224.255,
        density_g_cm3: 1.30,
        glass_transition_k: Some(573.0),
        melting_point_k: None,
        polymer_type: "thermoset",
        category: "phenolic",
    }
}
