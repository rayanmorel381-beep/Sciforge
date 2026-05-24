use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H26ClN3",
        name: "chloroquine",
        composition: &[(1, 26), (6, 18), (7, 3), (17, 1)],
        molar_mass: 319.872,
        category: "antimalarial",
        state_at_stp: "solid",
        melting_point_k: Some(360.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.140),
    }
}
