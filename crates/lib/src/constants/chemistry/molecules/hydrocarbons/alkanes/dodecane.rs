use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H26",
        name: "dodecane",
        composition: &[(6, 12), (1, 26)],
        molar_mass: 170.335,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(263.6),
        boiling_point_k: Some(489.45),
        density_g_cm3: Some(0.749),
    }
}
