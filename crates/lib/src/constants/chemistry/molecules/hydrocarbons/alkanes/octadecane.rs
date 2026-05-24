use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H38",
        name: "octadecane",
        composition: &[(6, 18), (1, 38)],
        molar_mass: 254.494,
        category: "alkane",
        state_at_stp: "solid",
        melting_point_k: Some(301.3),
        boiling_point_k: Some(589.5),
        density_g_cm3: Some(0.777),
    }
}
