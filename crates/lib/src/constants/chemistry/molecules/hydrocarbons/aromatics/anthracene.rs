use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H10",
        name: "anthracene",
        composition: &[(6, 14), (1, 10)],
        molar_mass: 178.234,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(489.0),
        boiling_point_k: Some(613.0),
        density_g_cm3: Some(1.25),
    }
}
