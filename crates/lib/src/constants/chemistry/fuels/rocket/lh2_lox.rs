use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "LH2/LOX",
        category: "rocket",
        composition: "liquid hydrogen with liquid oxygen oxidizer",
        density_kg_l: 0.0708,
        energy_density_mj_kg: 142.0,
        energy_density_mj_l: 10.05,
        octane_ron: None,
        cetane_number: None,
        flash_point_k: None,
        autoignition_k: Some(773.0),
        state_at_stp: "liquid",
    }
}
