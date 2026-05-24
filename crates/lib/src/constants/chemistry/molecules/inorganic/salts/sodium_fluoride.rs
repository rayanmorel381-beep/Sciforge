use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaF",
        name: "sodium fluoride",
        composition: &[(9, 1), (11, 1)],
        molar_mass: 41.988,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1266.0),
        boiling_point_k: Some(1977.0),
        density_g_cm3: Some(2.558),
    }
}
