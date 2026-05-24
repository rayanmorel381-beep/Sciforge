pub mod double_wishbone;
pub mod macpherson;
pub mod multilink;
pub mod trailing_arm;

pub use double_wishbone::DoubleWishbone;
pub use macpherson::MacPherson;
pub use multilink::MultiLink;
pub use trailing_arm::TrailingArm;

#[derive(Debug, Clone)]
pub enum Independent {
    DoubleWishbone(DoubleWishbone),
    MacPherson(MacPherson),
    MultiLink(MultiLink),
    TrailingArm(TrailingArm),
}
