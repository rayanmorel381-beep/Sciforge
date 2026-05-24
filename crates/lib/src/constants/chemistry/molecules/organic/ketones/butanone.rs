use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5COCH3",
        name: "butan-2-one",
        composition: &[(6, 4), (1, 8), (8, 1)],
        molar_mass: 72.107,
        category: "ketone",
        state_at_stp: "liquid",
        melting_point_k: Some(186.5),
        boiling_point_k: Some(352.7),
        density_g_cm3: Some(0.805),
    }
}
