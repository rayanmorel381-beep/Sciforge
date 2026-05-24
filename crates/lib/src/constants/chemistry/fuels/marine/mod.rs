use super::Fuel;

pub mod hfo;
pub mod mdo;

pub fn all() -> Vec<Fuel> {
    vec![hfo::fuel(), mdo::fuel()]
}
