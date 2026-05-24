use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MoS2",
        name: "molybdenite",
        composition: &[(16, 2), (42, 1)],
        molar_mass: 160.066,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(4.800),
    }
}
