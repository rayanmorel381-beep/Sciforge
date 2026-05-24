use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe2O3",
        name: "iron(III) oxide",
        composition: &[(26, 2), (8, 3)],
        molar_mass: 159.687,
        category: "mineral",
        state_at_stp: "solid",
        melting_point_k: Some(1838.0),
        boiling_point_k: None,
        density_g_cm3: Some(5.242),
    }
}
