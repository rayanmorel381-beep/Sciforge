use crate::components::exhaust::{CatalyticConverter, Muffler};

#[derive(Debug, Clone)]
pub struct ExhaustKit {
    pub catalyst: CatalyticConverter,
    pub muffler: Muffler,
}

pub fn sport() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::three_way(),
        muffler: Muffler::active_valve(64.0, 78.0),
    }
}

pub fn all() -> Vec<ExhaustKit> {
    vec![sport()]
}
