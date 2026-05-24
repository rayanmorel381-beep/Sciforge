pub mod alls;
pub mod front;
pub mod rear;

pub use alls::{Awd, FullTime, OnDemand, PartTime};
pub use front::{Fwd, Longitudinal, Transverse};
pub use rear::{DeDion, Independent, LiveAxle, Rwd};

#[derive(Debug, Clone)]
pub enum DriveLayout {
    Fwd(Fwd),
    Rwd(Rwd),
    Awd(Awd),
}
