use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KI",
        name: "potassium iodide",
        composition: &[(19, 1), (53, 1)],
        molar_mass: 166.003,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(954.0),
        boiling_point_k: Some(1603.0),
        density_g_cm3: Some(3.123),
    }
}
