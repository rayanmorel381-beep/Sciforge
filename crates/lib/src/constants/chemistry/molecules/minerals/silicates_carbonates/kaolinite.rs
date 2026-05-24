use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Al2Si2O5(OH)4",
        name: "kaolinite",
        composition: &[(1, 4), (8, 9), (13, 2), (14, 2)],
        molar_mass: 258.160,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.600),
    }
}
