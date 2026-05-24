use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaI",
        name: "sodium iodide",
        composition: &[(11, 1), (53, 1)],
        molar_mass: 149.894,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(933.0),
        boiling_point_k: Some(1577.0),
        density_g_cm3: Some(3.670),
    }
}
