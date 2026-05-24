use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H10",
        name: "biphenyl",
        composition: &[(6, 12), (1, 10)],
        molar_mass: 154.211,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(342.0),
        boiling_point_k: Some(528.0),
        density_g_cm3: Some(1.04),
    }
}
