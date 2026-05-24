use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H22O11",
        name: "sucrose",
        composition: &[(6, 12), (1, 22), (8, 11)],
        molar_mass: 342.297,
        category: "disaccharide",
        state_at_stp: "solid",
        melting_point_k: Some(459.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.587),
    }
}
