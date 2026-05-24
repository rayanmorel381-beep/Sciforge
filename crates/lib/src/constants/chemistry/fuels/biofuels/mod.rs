use super::Fuel;

pub mod biodiesel_b100;
pub mod biogas;
pub mod ethanol_e85;

pub fn all() -> Vec<Fuel> {
    vec![
        ethanol_e85::fuel(),
        biodiesel_b100::fuel(),
        biogas::fuel(),
    ]
}
