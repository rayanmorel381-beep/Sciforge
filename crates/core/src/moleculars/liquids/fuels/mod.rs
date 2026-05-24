pub mod biodiesel_b100;
pub mod biodiesel_b20;
pub mod diesel;
pub mod ethanol;
pub mod gasoline;
pub mod gasoline_e85;
pub mod jet_b;
pub mod kerosene_jet_a1;
pub mod methanol;

pub use biodiesel_b100::*;
pub use biodiesel_b20::*;
pub use diesel::*;
pub use ethanol::*;
pub use gasoline::*;
pub use gasoline_e85::*;
pub use jet_b::*;
pub use kerosene_jet_a1::*;
pub use methanol::*;

use crate::moleculars::Liquid;

pub fn all_fuels() -> Vec<&'static Liquid> {
    vec![
        &GASOLINE,
        &GASOLINE_E85,
        &DIESEL,
        &BIODIESEL_B20,
        &BIODIESEL_B100,
        &KEROSENE_JET_A1,
        &JET_B,
        &ETHANOL,
        &METHANOL,
    ]
}
