use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Mg3Si2O5(OH)4",
        name: "serpentine",
        composition: &[(1, 4), (8, 9), (12, 3), (14, 2)],
        molar_mass: 277.111,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.550),
    }
}
