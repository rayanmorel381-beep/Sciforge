pub mod locked;
pub mod lsd;
pub mod open;
pub mod torque_vectoring;

pub use locked::LockedDifferential;
pub use lsd::{LimitedSlipDifferential, LsdType};
pub use open::OpenDifferential;
pub use torque_vectoring::TorqueVectoring;

#[derive(Debug, Clone)]
pub enum Differential {
    Open(OpenDifferential),
    Lsd(LimitedSlipDifferential),
    Locked(LockedDifferential),
    TorqueVectoring(TorqueVectoring),
}
