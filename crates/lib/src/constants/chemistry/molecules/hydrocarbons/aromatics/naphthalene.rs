use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H8",
        name: "naphthalene",
        composition: &[(6, 10), (1, 8)],
        molar_mass: 128.174,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(353.4),
        boiling_point_k: Some(491.1),
        density_g_cm3: Some(1.025),
    }
}
