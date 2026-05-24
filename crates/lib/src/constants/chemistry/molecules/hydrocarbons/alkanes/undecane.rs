use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C11H24",
        name: "undecane",
        composition: &[(6, 11), (1, 24)],
        molar_mass: 156.308,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(247.6),
        boiling_point_k: Some(469.04),
        density_g_cm3: Some(0.740),
    }
}
