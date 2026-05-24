pub mod alloy;
pub mod carbon;
pub mod steel;

pub use alloy::AlloyRim;
pub use carbon::CarbonRim;
pub use steel::SteelRim;

#[derive(Debug, Clone)]
pub enum Rim {
    Alloy(AlloyRim),
    Steel(SteelRim),
    Carbon(CarbonRim),
}
