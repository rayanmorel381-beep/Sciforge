use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14N2O3",
        name: "hydroxylysine",
        composition: &[(6, 6), (1, 14), (7, 2), (8, 3)],
        molar_mass: 162.187,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(478.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.310),
    }
}
