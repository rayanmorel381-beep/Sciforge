use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Na2O2",
        name: "sodium peroxide",
        composition: &[(8, 2), (11, 2)],
        molar_mass: 77.978,
        category: "peroxide",
        state_at_stp: "solid",
        melting_point_k: Some(733.0),
        boiling_point_k: Some(930.0),
        density_g_cm3: Some(2.810),
    }
}
