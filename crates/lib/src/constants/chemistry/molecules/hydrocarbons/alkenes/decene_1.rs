use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H20",
        name: "1-decene",
        composition: &[(6, 10), (1, 20)],
        molar_mass: 140.266,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(206.9),
        boiling_point_k: Some(443.75),
        density_g_cm3: Some(0.741),
    }
}
