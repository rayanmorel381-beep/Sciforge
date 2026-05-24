pub mod swingarm;
pub mod telescopic_fork;

pub use swingarm::{Swingarm, SwingarmType};
pub use telescopic_fork::TelescopicFork;

#[derive(Debug, Clone)]
pub enum MotorcycleSuspension {
    TelescopicFork(TelescopicFork),
    Swingarm(Swingarm),
}
