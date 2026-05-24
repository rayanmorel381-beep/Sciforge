use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H34O2",
        name: "oleic acid",
        composition: &[(6, 18), (1, 34), (8, 2)],
        molar_mass: 282.461,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(286.5),
        boiling_point_k: Some(633.0),
        density_g_cm3: Some(0.895),
    }
}
