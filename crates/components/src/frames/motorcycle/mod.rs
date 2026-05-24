pub mod cradle;
pub mod perimeter;
pub mod trellis;

pub use cradle::{CradleFrame, CradleLayout};
pub use perimeter::PerimeterFrame;
pub use trellis::{TrellisMaterial, TrellisFrame};

#[derive(Debug, Clone)]
pub enum MotorcycleFrame {
    Perimeter(PerimeterFrame),
    Trellis(TrellisFrame),
    Cradle(CradleFrame),
}
