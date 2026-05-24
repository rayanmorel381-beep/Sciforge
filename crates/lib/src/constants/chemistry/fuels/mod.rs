use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct Fuel {
    pub name: &'static str,
    pub category: &'static str,
    pub composition: &'static str,
    pub density_kg_l: f64,
    pub energy_density_mj_kg: f64,
    pub energy_density_mj_l: f64,
    pub octane_ron: Option<f64>,
    pub cetane_number: Option<f64>,
    pub flash_point_k: Option<f64>,
    pub autoignition_k: Option<f64>,
    pub state_at_stp: &'static str,
}

pub mod aviation;
pub mod biofuels;
pub mod gaseous;
pub mod marine;
pub mod petroleum;
pub mod rocket;

fn init_fuels() -> Vec<Fuel> {
    let mut all = Vec::new();
    all.extend(petroleum::all());
    all.extend(biofuels::all());
    all.extend(gaseous::all());
    all.extend(aviation::all());
    all.extend(marine::all());
    all.extend(rocket::all());
    all
}

fn fuels() -> &'static Vec<Fuel> {
    static FUELS: OnceLock<Vec<Fuel>> = OnceLock::new();
    FUELS.get_or_init(init_fuels)
}

pub fn all() -> &'static [Fuel] {
    fuels()
}

pub fn by_name(name: &str) -> Option<&'static Fuel> {
    fuels().iter().find(|f| f.name.eq_ignore_ascii_case(name))
}

pub fn by_category(category: &str) -> Vec<&'static Fuel> {
    fuels().iter().filter(|f| f.category.eq_ignore_ascii_case(category)).collect()
}
