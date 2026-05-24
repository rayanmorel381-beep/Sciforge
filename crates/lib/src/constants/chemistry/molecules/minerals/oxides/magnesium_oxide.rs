use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MgO",
        name: "magnesium oxide",
        composition: &[(12, 1), (8, 1)],
        molar_mass: 40.304,
        category: "mineral",
        state_at_stp: "solid",
        melting_point_k: Some(3125.0),
        boiling_point_k: Some(3870.0),
        density_g_cm3: Some(3.58),
    }
}
