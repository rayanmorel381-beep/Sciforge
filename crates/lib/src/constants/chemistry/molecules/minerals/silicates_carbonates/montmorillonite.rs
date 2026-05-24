use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Al2Si4O10(OH)2",
        name: "montmorillonite",
        composition: &[(1, 2), (8, 12), (13, 2), (14, 4)],
        molar_mass: 360.310,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.350),
    }
}
