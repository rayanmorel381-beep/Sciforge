use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "K2SO4",
        name: "potassium sulfate",
        composition: &[(8, 4), (16, 1), (19, 2)],
        molar_mass: 174.259,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1342.0),
        boiling_point_k: Some(1962.0),
        density_g_cm3: Some(2.660),
    }
}
