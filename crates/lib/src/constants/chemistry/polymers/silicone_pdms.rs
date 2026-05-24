use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "polydimethylsiloxane",
        abbreviation: "PDMS",
        monomer_formula: "C2H6OSi",
        monomer_molar_mass: 74.154,
        density_g_cm3: 0.97,
        glass_transition_k: Some(150.0),
        melting_point_k: Some(235.0),
        polymer_type: "elastomer",
        category: "silicone",
    }
}
