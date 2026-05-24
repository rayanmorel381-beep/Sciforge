use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "MDO",
        category: "marine",
        composition: "marine diesel oil, distillate blend",
        density_kg_l: 0.86,
        energy_density_mj_kg: 42.7,
        energy_density_mj_l: 36.7,
        octane_ron: None,
        cetane_number: Some(40.0),
        flash_point_k: Some(333.0),
        autoignition_k: Some(498.0),
        state_at_stp: "liquid",
    }
}
