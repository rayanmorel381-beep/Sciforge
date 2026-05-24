use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KCl",
        name: "potassium chloride",
        composition: &[(17, 1), (19, 1)],
        molar_mass: 74.551,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1043.0),
        boiling_point_k: Some(1693.0),
        density_g_cm3: Some(1.984),
    }
}
