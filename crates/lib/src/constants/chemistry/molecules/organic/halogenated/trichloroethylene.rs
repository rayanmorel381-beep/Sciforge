use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2HCl3",
        name: "trichloroethylene",
        composition: &[(6, 2), (1, 1), (17, 3)],
        molar_mass: 131.388,
        category: "haloalkene",
        state_at_stp: "liquid",
        melting_point_k: Some(186.0),
        boiling_point_k: Some(360.4),
        density_g_cm3: Some(1.464),
    }
}
