use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCOOH",
        name: "formic acid",
        composition: &[(1, 2), (6, 1), (8, 2)],
        molar_mass: 46.025,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(281.4),
        boiling_point_k: Some(373.7),
        density_g_cm3: Some(1.220),
    }
}
