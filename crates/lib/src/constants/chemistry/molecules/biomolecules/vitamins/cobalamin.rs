use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C63H88CoN14O14P",
        name: "cobalamin",
        composition: &[(6, 63), (1, 88), (7, 14), (8, 14), (15, 1), (27, 1)],
        molar_mass: 1355.388,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(573.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.500),
    }
}
