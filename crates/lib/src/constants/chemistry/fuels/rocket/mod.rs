use super::Fuel;

pub mod hypergolic;
pub mod lh2_lox;
pub mod rp1;

pub fn all() -> Vec<Fuel> {
    vec![rp1::fuel(), lh2_lox::fuel(), hypergolic::fuel()]
}
