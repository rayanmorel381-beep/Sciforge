use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "K2CO3",
        name: "potassium carbonate",
        composition: &[(6, 1), (8, 3), (19, 2)],
        molar_mass: 138.205,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1164.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.430),
    }
}
