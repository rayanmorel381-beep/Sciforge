use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H11NO2",
        name: "valine",
        composition: &[(6, 5), (1, 11), (7, 1), (8, 2)],
        molar_mass: 117.151,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(571.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.316),
    }
}
