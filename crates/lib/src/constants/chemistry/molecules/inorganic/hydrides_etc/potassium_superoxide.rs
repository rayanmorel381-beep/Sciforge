use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KO2",
        name: "potassium superoxide",
        composition: &[(8, 2), (19, 1)],
        molar_mass: 71.097,
        category: "peroxide",
        state_at_stp: "solid",
        melting_point_k: Some(833.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.140),
    }
}
