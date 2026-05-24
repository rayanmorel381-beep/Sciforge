use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C48H74O14",
        name: "ivermectin",
        composition: &[(1, 74), (6, 48), (8, 14)],
        molar_mass: 875.106,
        category: "antiparasitic",
        state_at_stp: "solid",
        melting_point_k: Some(428.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.220),
    }
}
