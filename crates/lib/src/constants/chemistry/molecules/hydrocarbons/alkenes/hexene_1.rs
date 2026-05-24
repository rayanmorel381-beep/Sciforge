use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12",
        name: "1-hexene",
        composition: &[(6, 6), (1, 12)],
        molar_mass: 84.160,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(133.4),
        boiling_point_k: Some(336.6),
        density_g_cm3: Some(0.673),
    }
}
