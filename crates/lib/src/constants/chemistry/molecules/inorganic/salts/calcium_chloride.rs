use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaCl2",
        name: "calcium chloride",
        composition: &[(17, 2), (20, 1)],
        molar_mass: 110.984,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1045.0),
        boiling_point_k: Some(2208.0),
        density_g_cm3: Some(2.150),
    }
}
