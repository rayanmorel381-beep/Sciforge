use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C43H66N12O12S2",
        name: "oxytocin",
        composition: &[(6, 43), (1, 66), (7, 12), (8, 12), (16, 2)],
        molar_mass: 1007.193,
        category: "hormone",
        state_at_stp: "solid",
        melting_point_k: Some(465.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.450),
    }
}
