pub mod all_season;
pub mod off_road;
pub mod slick;
pub mod summer;
pub mod winter;

pub use all_season::AllSeasonTire;
pub use off_road::OffRoadTire;
pub use slick::{SlickCompound, SlickTire};
pub use summer::SummerTire;
pub use winter::WinterTire;

#[derive(Debug, Clone)]
pub enum Tire {
    Summer(SummerTire),
    Winter(WinterTire),
    AllSeason(AllSeasonTire),
    Slick(SlickTire),
    OffRoad(OffRoadTire),
}
