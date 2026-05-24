use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7OH",
        name: "propan-1-ol",
        composition: &[(6, 3), (1, 8), (8, 1)],
        molar_mass: 60.096,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(146.0),
        boiling_point_k: Some(370.4),
        density_g_cm3: Some(0.803),
    }
}
