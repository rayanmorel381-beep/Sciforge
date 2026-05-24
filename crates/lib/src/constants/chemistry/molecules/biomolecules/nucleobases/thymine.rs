use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H6N2O2",
        name: "thymine",
        composition: &[(6, 5), (1, 6), (7, 2), (8, 2)],
        molar_mass: 126.115,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(589.0),
        boiling_point_k: Some(608.0),
        density_g_cm3: Some(1.223),
    }
}
