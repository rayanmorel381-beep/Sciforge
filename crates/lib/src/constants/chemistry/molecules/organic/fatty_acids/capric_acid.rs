use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H20O2",
        name: "capric acid",
        composition: &[(6, 10), (1, 20), (8, 2)],
        molar_mass: 172.265,
        category: "fatty acid",
        state_at_stp: "solid",
        melting_point_k: Some(304.7),
        boiling_point_k: Some(542.0),
        density_g_cm3: Some(0.893),
    }
}
