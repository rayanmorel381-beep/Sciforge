use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H10",
        name: "o-xylene",
        composition: &[(6, 8), (1, 10)],
        molar_mass: 106.167,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(247.98),
        boiling_point_k: Some(417.6),
        density_g_cm3: Some(0.880),
    }
}
