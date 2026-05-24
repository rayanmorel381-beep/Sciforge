pub mod brakes;
pub mod crude;
pub mod compressor_oil_iso100;
pub mod engine;
pub mod gearboxes;
pub mod transformer_oil;
pub mod turbine;

pub use brakes::*;
pub use crude::*;
pub use compressor_oil_iso100::*;
pub use engine::*;
pub use gearboxes::*;
pub use transformer_oil::*;
pub use turbine::*;

use crate::moleculars::Liquid;

pub fn all_petroleum_oils() -> Vec<&'static Liquid> {
    vec![
        &CRUDE_OIL_LIGHT,
        &CRUDE_OIL_MEDIUM,
        &CRUDE_OIL_HEAVY,
        &ENGINE_OIL_SAE_5W30,
        &ENGINE_OIL_SAE_5W40,
        &ENGINE_OIL_SAE_10W40,
        &ENGINE_OIL_SAE_15W40,
        &ENGINE_OIL_SAE_0W20,
        &GEAR_OIL_70W,
        &GEAR_OIL_75W,
        &GEAR_OIL_80W,
        &GEAR_OIL_85W,
        &GEAR_OIL_75W80,
        &GEAR_OIL_75W85,
        &GEAR_OIL_75W90,
        &GEAR_OIL_75W95,
        &GEAR_OIL_80W90,
        &GEAR_OIL_85W90,
        &GEAR_OIL_75W110,
        &GEAR_OIL_80W140,
        &GEAR_OIL_85W140,
        &GEAR_OIL_75W140,
        &AUTOMATIC_TRANS_FLUID_CVTF,
        &AUTOMATIC_TRANS_FLUID_TRAN_M14,
        &AUTOMATIC_TRANS_FLUID_TRAN_M15,
        &AUTOMATIC_TRANS_FLUID_TRAN_M17,
        &AUTOMATIC_TRANS_FLUID_DEXRON_II,
        &AUTOMATIC_TRANS_FLUID_DEXRON_III,
        &AUTOMATIC_TRANS_FLUID_DEXRON_VI,
        &AUTOMATIC_TRANS_FLUID_MERCON_V,
        &AUTOMATIC_TRANS_FLUID_MERCON_LV,
        &AUTOMATIC_TRANS_FLUID_MERCON_ULV,
        &AUTOMATIC_TRANS_FLUID_ATF_PLUS4,
        &AUTOMATIC_TRANS_FLUID_DCTF,
        &AUTOMATIC_TRANS_FLUID_DSG,
        &AUTOMATIC_TRANS_FLUID_EV_REDUCTION_GEAR,
        &TURBINE_OIL_ISO68,
        &BRAKE_FLUID_DOT2,
        &BRAKE_FLUID_DOT3,
        &BRAKE_FLUID_DOT4,
        &BRAKE_FLUID_DOT5_1,
        &BRAKE_FLUID_DOT5,
        &COMPRESSOR_OIL_ISO100,
        &TRANSFORMER_OIL,
    ]
}
