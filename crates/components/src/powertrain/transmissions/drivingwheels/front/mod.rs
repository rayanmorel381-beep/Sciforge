pub mod longitudinal;
pub mod transverse;

pub use longitudinal::Longitudinal;
pub use transverse::Transverse;

#[derive(Debug, Clone)]
pub enum Fwd {
    Transverse(Transverse),
    Longitudinal(Longitudinal),
}
