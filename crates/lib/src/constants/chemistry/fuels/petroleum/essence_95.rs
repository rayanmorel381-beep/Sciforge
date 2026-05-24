use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "essence 95",
        category: "petroleum",
        composition: "C5-C12 hydrocarbons, RON 95",
        density_kg_l: 0.745,
        energy_density_mj_kg: 46.4,
        energy_density_mj_l: 34.6,
        octane_ron: Some(95.0),
        cetane_number: None,
        flash_point_k: Some(230.0),
        autoignition_k: Some(553.0),
        state_at_stp: "liquid",
    }
}
