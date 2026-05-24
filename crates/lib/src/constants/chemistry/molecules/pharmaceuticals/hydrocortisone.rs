use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H30O5",
        name: "hydrocortisone",
        composition: &[(1, 30), (6, 21), (8, 5)],
        molar_mass: 362.460,
        category: "corticosteroid",
        state_at_stp: "solid",
        melting_point_k: Some(493.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
