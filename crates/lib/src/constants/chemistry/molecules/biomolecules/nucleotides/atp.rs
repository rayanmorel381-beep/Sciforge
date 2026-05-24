use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H16N5O13P3",
        name: "adenosine triphosphate",
        composition: &[(6, 10), (1, 16), (7, 5), (8, 13), (15, 3)],
        molar_mass: 507.181,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.040),
    }
}
