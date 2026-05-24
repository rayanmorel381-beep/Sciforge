use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C57H110O6",
        name: "tristearin",
        composition: &[(6, 57), (1, 110), (8, 6)],
        molar_mass: 891.480,
        category: "triglyceride",
        state_at_stp: "solid",
        melting_point_k: Some(345.0),
        boiling_point_k: Some(586.0),
        density_g_cm3: Some(0.862),
    }
}
