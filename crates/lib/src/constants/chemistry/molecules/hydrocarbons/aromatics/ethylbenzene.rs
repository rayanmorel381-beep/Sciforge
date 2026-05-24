use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H10",
        name: "ethylbenzene",
        composition: &[(6, 8), (1, 10)],
        molar_mass: 106.167,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(178.2),
        boiling_point_k: Some(409.34),
        density_g_cm3: Some(0.867),
    }
}
