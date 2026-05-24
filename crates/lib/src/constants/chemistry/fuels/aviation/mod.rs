use super::Fuel;

pub mod avgas;
pub mod jet_a1;

pub fn all() -> Vec<Fuel> {
    vec![jet_a1::fuel(), avgas::fuel()]
}
