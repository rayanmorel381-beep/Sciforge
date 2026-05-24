use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H24O2",
        name: "lauric acid",
        composition: &[(6, 12), (1, 24), (8, 2)],
        molar_mass: 200.318,
        category: "fatty acid",
        state_at_stp: "solid",
        melting_point_k: Some(316.7),
        boiling_point_k: Some(571.0),
        density_g_cm3: Some(0.880),
    }
}
