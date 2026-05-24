use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H10",
        name: "isobutane",
        composition: &[(6, 4), (1, 10)],
        molar_mass: 58.122,
        category: "alkane",
        state_at_stp: "gas",
        melting_point_k: Some(113.5),
        boiling_point_k: Some(261.4),
        density_g_cm3: Some(0.00251),
    }
}
