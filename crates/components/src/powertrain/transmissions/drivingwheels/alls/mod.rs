pub mod full_time;
pub mod on_demand;
pub mod part_time;

pub use full_time::FullTime;
pub use on_demand::OnDemand;
pub use part_time::PartTime;

#[derive(Debug, Clone)]
pub enum Awd {
    FullTime(FullTime),
    PartTime(PartTime),
    OnDemand(OnDemand),
}
