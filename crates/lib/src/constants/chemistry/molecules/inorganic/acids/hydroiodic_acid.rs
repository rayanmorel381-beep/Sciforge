use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HI",
        name: "hydroiodic acid",
        composition: &[(1, 1), (53, 1)],
        molar_mass: 127.911,
        category: "acid",
        state_at_stp: "gas",
        melting_point_k: Some(222.0),
        boiling_point_k: Some(237.6),
        density_g_cm3: Some(2.850),
    }
}
