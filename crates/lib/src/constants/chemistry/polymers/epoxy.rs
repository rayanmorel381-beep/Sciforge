use super::Polymer;

pub fn polymer() -> Polymer {
    Polymer {
        name: "epoxy",
        abbreviation: "EP",
        monomer_formula: "C18H20O3",
        monomer_molar_mass: 284.351,
        density_g_cm3: 1.20,
        glass_transition_k: Some(393.0),
        melting_point_k: None,
        polymer_type: "thermoset",
        category: "epoxide",
    }
}
