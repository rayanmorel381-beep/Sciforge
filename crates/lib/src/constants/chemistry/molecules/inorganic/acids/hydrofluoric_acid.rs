use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HF",
        name: "hydrofluoric acid",
        composition: &[(1, 1), (9, 1)],
        molar_mass: 20.006,
        category: "acid",
        state_at_stp: "gas",
        melting_point_k: Some(189.8),
        boiling_point_k: Some(292.7),
        density_g_cm3: Some(1.150),
    }
}
