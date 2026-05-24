use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Ca2Mg5Si8O22(OH)2",
        name: "tremolite",
        composition: &[(1, 2), (8, 24), (12, 5), (14, 8), (20, 2)],
        molar_mass: 812.410,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.990),
    }
}
