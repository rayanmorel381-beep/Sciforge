use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5CHO",
        name: "propionaldehyde",
        composition: &[(6, 3), (1, 6), (8, 1)],
        molar_mass: 58.080,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(192.0),
        boiling_point_k: Some(321.0),
        density_g_cm3: Some(0.805),
    }
}
