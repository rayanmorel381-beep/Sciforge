use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KAl3Si3O10(OH)2",
        name: "muscovite",
        composition: &[(1, 2), (8, 12), (13, 3), (14, 3), (19, 1)],
        molar_mass: 398.310,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.820),
    }
}
