use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H16N5O14P3",
        name: "guanosine triphosphate",
        composition: &[(6, 10), (1, 16), (7, 5), (8, 14), (15, 3)],
        molar_mass: 523.180,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.500),
    }
}
