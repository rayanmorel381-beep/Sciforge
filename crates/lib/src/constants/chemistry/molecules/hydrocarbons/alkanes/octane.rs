use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H18",
        name: "octane",
        composition: &[(6, 8), (1, 18)],
        molar_mass: 114.229,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(216.4),
        boiling_point_k: Some(398.83),
        density_g_cm3: Some(0.703),
    }
}
