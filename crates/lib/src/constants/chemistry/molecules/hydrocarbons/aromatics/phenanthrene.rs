use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H10",
        name: "phenanthrene",
        composition: &[(6, 14), (1, 10)],
        molar_mass: 178.234,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(372.5),
        boiling_point_k: Some(613.15),
        density_g_cm3: Some(1.18),
    }
}
