use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "biodiesel B100",
        category: "biofuel",
        composition: "fatty acid methyl esters (FAME)",
        density_kg_l: 0.880,
        energy_density_mj_kg: 37.5,
        energy_density_mj_l: 33.0,
        octane_ron: None,
        cetane_number: Some(50.0),
        flash_point_k: Some(403.0),
        autoignition_k: Some(450.0),
        state_at_stp: "liquid",
    }
}
