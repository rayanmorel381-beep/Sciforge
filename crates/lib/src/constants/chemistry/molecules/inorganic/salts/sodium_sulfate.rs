use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Na2SO4",
        name: "sodium sulfate",
        composition: &[(8, 4), (11, 2), (16, 1)],
        molar_mass: 142.040,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1157.0),
        boiling_point_k: Some(1702.0),
        density_g_cm3: Some(2.664),
    }
}
