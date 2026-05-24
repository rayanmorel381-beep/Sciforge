use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H22",
        name: "decane",
        composition: &[(6, 10), (1, 22)],
        molar_mass: 142.282,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(243.5),
        boiling_point_k: Some(447.30),
        density_g_cm3: Some(0.730),
    }
}
