use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaF2",
        name: "fluorite",
        composition: &[(9, 2), (20, 1)],
        molar_mass: 78.075,
        category: "halide",
        state_at_stp: "solid",
        melting_point_k: Some(1691.0),
        boiling_point_k: Some(2806.0),
        density_g_cm3: Some(3.180),
    }
}
