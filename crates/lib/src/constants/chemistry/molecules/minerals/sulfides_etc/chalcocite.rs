use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cu2S",
        name: "chalcocite",
        composition: &[(16, 1), (29, 2)],
        molar_mass: 159.157,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(1402.0),
        boiling_point_k: None,
        density_g_cm3: Some(5.780),
    }
}
