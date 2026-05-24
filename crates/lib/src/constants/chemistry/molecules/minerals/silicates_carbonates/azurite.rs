use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cu3(CO3)2(OH)2",
        name: "azurite",
        composition: &[(1, 2), (6, 2), (8, 8), (29, 3)],
        molar_mass: 344.665,
        category: "carbonate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.830),
    }
}
