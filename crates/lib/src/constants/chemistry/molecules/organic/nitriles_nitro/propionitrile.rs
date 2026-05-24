use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5CN",
        name: "propionitrile",
        composition: &[(6, 3), (1, 5), (7, 1)],
        molar_mass: 55.079,
        category: "nitrile",
        state_at_stp: "liquid",
        melting_point_k: Some(180.3),
        boiling_point_k: Some(370.5),
        density_g_cm3: Some(0.777),
    }
}
