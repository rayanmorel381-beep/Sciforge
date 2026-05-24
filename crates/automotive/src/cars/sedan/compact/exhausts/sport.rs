use crate::components::exhaust::{CatalyticConverter, DieselParticulateFilter, Muffler};

#[derive(Debug, Clone)]
pub struct ExhaustKit {
    pub catalyst: CatalyticConverter,
    pub dpf: Option<DieselParticulateFilter>,
    pub muffler: Muffler,
}

pub fn sport() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::three_way(),
        dpf: None,
        muffler: Muffler::active_valve(62.0, 72.0),
    }
}

pub fn all() -> Vec<ExhaustKit> {
    vec![sport()]
}
