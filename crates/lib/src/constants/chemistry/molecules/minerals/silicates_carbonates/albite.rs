use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaAlSi3O8",
        name: "albite",
        composition: &[(8, 8), (11, 1), (13, 1), (14, 3)],
        molar_mass: 262.220,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1373.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.620),
    }
}
