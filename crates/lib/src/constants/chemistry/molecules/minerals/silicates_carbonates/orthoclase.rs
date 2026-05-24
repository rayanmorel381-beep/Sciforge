use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KAlSi3O8",
        name: "orthoclase",
        composition: &[(8, 8), (13, 1), (14, 3), (19, 1)],
        molar_mass: 278.330,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1423.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.560),
    }
}
