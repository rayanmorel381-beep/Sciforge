use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaAl2Si2O8",
        name: "anorthite",
        composition: &[(8, 8), (13, 2), (14, 2), (20, 1)],
        molar_mass: 278.210,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1823.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.730),
    }
}
