pub mod backbone;
pub mod ladder;
pub mod spaceframe;

pub use backbone::BackboneFrame;
pub use ladder::{LadderFrame, LadderMaterial};
pub use spaceframe::{Spaceframe, TubeMaterial};

#[derive(Debug, Clone)]
pub enum Chassis {
    Ladder(LadderFrame),
    Backbone(BackboneFrame),
    Spaceframe(Spaceframe),
}
