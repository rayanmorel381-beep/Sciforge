use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "biogas",
        category: "biofuel",
        composition: "60% CH4 + 40% CO2",
        density_kg_l: 0.0012,
        energy_density_mj_kg: 19.1,
        energy_density_mj_l: 0.023,
        octane_ron: Some(120.0),
        cetane_number: None,
        flash_point_k: None,
        autoignition_k: Some(810.0),
        state_at_stp: "gas",
    }
}
