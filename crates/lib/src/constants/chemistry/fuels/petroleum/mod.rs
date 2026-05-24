use super::Fuel;

pub mod diesel;
pub mod essence_95;
pub mod essence_98;
pub mod kerosene;

pub fn all() -> Vec<Fuel> {
    vec![
        essence_95::fuel(),
        essence_98::fuel(),
        diesel::fuel(),
        kerosene::fuel(),
    ]
}
