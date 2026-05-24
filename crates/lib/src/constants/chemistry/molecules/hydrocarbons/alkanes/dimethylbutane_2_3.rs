use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14",
        name: "2,3-dimethylbutane",
        composition: &[(6, 6), (1, 14)],
        molar_mass: 86.175,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(144.62),
        boiling_point_k: Some(331.13),
        density_g_cm3: Some(0.661),
    }
}
