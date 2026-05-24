use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H10",
        name: "m-xylene",
        composition: &[(6, 8), (1, 10)],
        molar_mass: 106.167,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(225.3),
        boiling_point_k: Some(412.27),
        density_g_cm3: Some(0.864),
    }
}
