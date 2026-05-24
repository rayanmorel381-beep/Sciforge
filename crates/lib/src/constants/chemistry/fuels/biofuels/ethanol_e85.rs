use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "ethanol E85",
        category: "biofuel",
        composition: "85% ethanol + 15% gasoline",
        density_kg_l: 0.781,
        energy_density_mj_kg: 33.1,
        energy_density_mj_l: 25.9,
        octane_ron: Some(105.0),
        cetane_number: None,
        flash_point_k: Some(260.0),
        autoignition_k: Some(636.0),
        state_at_stp: "liquid",
    }
}
