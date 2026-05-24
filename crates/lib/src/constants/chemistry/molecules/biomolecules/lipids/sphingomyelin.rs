use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C39H79N2O6P",
        name: "sphingomyelin",
        composition: &[(6, 39), (1, 79), (7, 2), (8, 6), (15, 1)],
        molar_mass: 703.034,
        category: "sphingolipid",
        state_at_stp: "solid",
        melting_point_k: Some(486.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.025),
    }
}
