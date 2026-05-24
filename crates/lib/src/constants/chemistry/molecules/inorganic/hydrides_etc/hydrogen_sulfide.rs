use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2S",
        name: "hydrogen sulfide",
        composition: &[(1, 2), (16, 1)],
        molar_mass: 34.081,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(187.6),
        boiling_point_k: Some(213.6),
        density_g_cm3: Some(0.001_539),
    }
}
