use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "(C8H13NO5)n",
        name: "chitin",
        composition: &[(6, 8), (1, 13), (7, 1), (8, 5)],
        molar_mass: 203.193,
        category: "polysaccharide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.420),
    }
}
