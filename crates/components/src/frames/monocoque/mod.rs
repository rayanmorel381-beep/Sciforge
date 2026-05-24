pub mod stressed_skin;
pub mod unibody;

pub use stressed_skin::{SkinMaterial, StressedSkin};
pub use unibody::{SteelGrade, Unibody};

#[derive(Debug, Clone)]
pub enum Monocoque {
    Unibody(Unibody),
    StressedSkin(StressedSkin),
}
