use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe(OH)3",
        name: "iron(III) hydroxide",
        composition: &[(1, 3), (8, 3), (26, 1)],
        molar_mass: 106.870,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.400),
    }
}
