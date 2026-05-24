use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Mg3Si4O10(OH)2",
        name: "talc",
        composition: &[(1, 2), (8, 12), (12, 3), (14, 4)],
        molar_mass: 379.265,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.750),
    }
}
