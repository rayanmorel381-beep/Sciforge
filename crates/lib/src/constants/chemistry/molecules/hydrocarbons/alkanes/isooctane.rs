use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H18",
        name: "isooctane",
        composition: &[(6, 8), (1, 18)],
        molar_mass: 114.229,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(165.78),
        boiling_point_k: Some(372.39),
        density_g_cm3: Some(0.692),
    }
}
