use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "(C6H10O5)n",
        name: "cellulose",
        composition: &[(6, 6), (1, 10), (8, 5)],
        molar_mass: 162.140,
        category: "polysaccharide",
        state_at_stp: "solid",
        melting_point_k: Some(740.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.500),
    }
}
