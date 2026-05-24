use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12O2",
        name: "caproic acid",
        composition: &[(6, 6), (1, 12), (8, 2)],
        molar_mass: 116.158,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(269.4),
        boiling_point_k: Some(478.4),
        density_g_cm3: Some(0.929),
    }
}
