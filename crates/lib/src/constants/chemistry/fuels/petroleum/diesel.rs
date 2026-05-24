use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "diesel",
        category: "petroleum",
        composition: "C8-C25 hydrocarbons",
        density_kg_l: 0.832,
        energy_density_mj_kg: 45.5,
        energy_density_mj_l: 37.9,
        octane_ron: None,
        cetane_number: Some(51.0),
        flash_point_k: Some(333.0),
        autoignition_k: Some(483.0),
        state_at_stp: "liquid",
    }
}
