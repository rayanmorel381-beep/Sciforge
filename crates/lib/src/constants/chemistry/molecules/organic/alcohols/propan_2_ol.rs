use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7OH",
        name: "propan-2-ol",
        composition: &[(6, 3), (1, 8), (8, 1)],
        molar_mass: 60.096,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(184.0),
        boiling_point_k: Some(355.5),
        density_g_cm3: Some(0.786),
    }
}
