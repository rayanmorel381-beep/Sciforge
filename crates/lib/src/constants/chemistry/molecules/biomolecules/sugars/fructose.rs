use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12O6",
        name: "fructose",
        composition: &[(6, 6), (1, 12), (8, 6)],
        molar_mass: 180.156,
        category: "monosaccharide",
        state_at_stp: "solid",
        melting_point_k: Some(376.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.694),
    }
}
