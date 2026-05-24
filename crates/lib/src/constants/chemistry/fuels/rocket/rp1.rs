use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "RP-1",
        category: "rocket",
        composition: "highly refined kerosene",
        density_kg_l: 0.81,
        energy_density_mj_kg: 43.5,
        energy_density_mj_l: 35.2,
        octane_ron: None,
        cetane_number: None,
        flash_point_k: Some(336.0),
        autoignition_k: Some(498.0),
        state_at_stp: "liquid",
    }
}
