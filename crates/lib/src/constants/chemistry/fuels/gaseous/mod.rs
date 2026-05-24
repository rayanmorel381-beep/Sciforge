use super::Fuel;

pub mod cng;
pub mod lng;
pub mod lpg;

pub fn all() -> Vec<Fuel> {
    vec![lpg::fuel(), cng::fuel(), lng::fuel()]
}
