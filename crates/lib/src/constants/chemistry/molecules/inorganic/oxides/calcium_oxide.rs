use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaO",
        name: "calcium oxide",
        composition: &[(8, 1), (20, 1)],
        molar_mass: 56.077,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(2886.0),
        boiling_point_k: Some(3123.0),
        density_g_cm3: Some(3.340),
    }
}
