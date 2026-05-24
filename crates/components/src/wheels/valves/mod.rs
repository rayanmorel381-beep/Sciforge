pub mod presta;
pub mod schrader;
pub mod tpms;

pub use presta::{PrestaValve, PrestaValveMaterial};
pub use schrader::{SchraderCoreMaterial, SchraderValve};
pub use tpms::{TpmsProtocol, TpmsValve};

#[derive(Debug, Clone)]
pub enum Valve {
    Schrader(SchraderValve),
    Presta(PrestaValve),
    Tpms(TpmsValve),
}
