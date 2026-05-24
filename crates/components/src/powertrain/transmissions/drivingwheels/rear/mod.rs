pub mod de_dion;
pub mod independent;
pub mod live_axle;

pub use de_dion::DeDion;
pub use independent::Independent;
pub use live_axle::LiveAxle;

#[derive(Debug, Clone)]
pub enum Rwd {
    LiveAxle(LiveAxle),
    Independent(Independent),
    DeDion(DeDion),
}
