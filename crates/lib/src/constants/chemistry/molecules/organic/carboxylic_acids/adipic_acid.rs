use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H10O4",
        name: "adipic acid",
        composition: &[(6, 6), (1, 10), (8, 4)],
        molar_mass: 146.142,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(425.7),
        boiling_point_k: Some(610.7),
        density_g_cm3: Some(1.360),
    }
}
