use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H19N3O5S",
        name: "amoxicillin",
        composition: &[(1, 19), (6, 16), (7, 3), (8, 5), (16, 1)],
        molar_mass: 365.404,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(467.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.560),
    }
}
