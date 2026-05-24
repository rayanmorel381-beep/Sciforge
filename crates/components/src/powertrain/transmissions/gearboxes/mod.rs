pub mod automatics;
pub mod manuals;
pub mod sequentials;

pub use automatics::cvt::{Cvt, CvtKind};
pub use automatics::robots::RobotizedManual;
pub use automatics::{AutomaticGearbox, AutomaticKind};
pub use manuals::ManualGearbox;
pub use sequentials::SequentialGearbox;

#[derive(Debug, Clone)]
pub enum Gearbox {
    Manual(ManualGearbox),
    Sequential(SequentialGearbox),
    Automatic(AutomaticGearbox),
    Cvt(Cvt),
    RobotizedManual(RobotizedManual),
}

impl From<ManualGearbox> for Gearbox {
    fn from(g: ManualGearbox) -> Self { Gearbox::Manual(g) }
}

impl From<SequentialGearbox> for Gearbox {
    fn from(g: SequentialGearbox) -> Self { Gearbox::Sequential(g) }
}

impl From<AutomaticGearbox> for Gearbox {
    fn from(g: AutomaticGearbox) -> Self { Gearbox::Automatic(g) }
}

impl From<Cvt> for Gearbox {
    fn from(g: Cvt) -> Self { Gearbox::Cvt(g) }
}

impl From<RobotizedManual> for Gearbox {
    fn from(g: RobotizedManual) -> Self { Gearbox::RobotizedManual(g) }
}
