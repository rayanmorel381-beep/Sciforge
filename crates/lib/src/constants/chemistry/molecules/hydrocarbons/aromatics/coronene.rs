use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C24H12",
        name: "coronene",
        composition: &[(6, 24), (1, 12)],
        molar_mass: 300.357,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(711.6),
        boiling_point_k: Some(798.0),
        density_g_cm3: Some(1.371),
    }
}
