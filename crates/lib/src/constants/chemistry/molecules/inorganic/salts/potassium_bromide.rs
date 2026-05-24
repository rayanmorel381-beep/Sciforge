use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KBr",
        name: "potassium bromide",
        composition: &[(19, 1), (35, 1)],
        molar_mass: 119.002,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1007.0),
        boiling_point_k: Some(1708.0),
        density_g_cm3: Some(2.740),
    }
}
