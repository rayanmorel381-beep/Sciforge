use crate::components::exhaust::{CatalyticConverter, DieselParticulateFilter, Muffler};

#[derive(Debug, Clone)]
pub struct ExhaustKit {
    pub catalyst: CatalyticConverter,
    pub dpf: Option<DieselParticulateFilter>,
    pub muffler: Muffler,
}

pub fn gasoline() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::three_way(),
        dpf: None,
        muffler: Muffler::standard(54.0),
    }
}

pub fn diesel() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::diesel_oxidation(),
        dpf: Some(DieselParticulateFilter::active(2.8)),
        muffler: Muffler::standard(52.0),
    }
}

pub fn all() -> Vec<ExhaustKit> {
    vec![gasoline(), diesel()]
}
