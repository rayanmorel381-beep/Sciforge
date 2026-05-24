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
        muffler: Muffler::standard(57.0),
    }
}

pub fn gasoline_premium() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::three_way(),
        dpf: None,
        muffler: Muffler::active_valve(57.0, 64.0),
    }
}

pub fn diesel() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::diesel_oxidation(),
        dpf: Some(DieselParticulateFilter::active(3.0)),
        muffler: Muffler::standard(54.0),
    }
}

pub fn diesel_premium() -> ExhaustKit {
    ExhaustKit {
        catalyst: CatalyticConverter::scr(),
        dpf: Some(DieselParticulateFilter::active(3.4)),
        muffler: Muffler::active_valve(54.0, 61.0),
    }
}

pub fn all() -> Vec<ExhaustKit> {
    vec![gasoline(), gasoline_premium(), diesel(), diesel_premium()]
}
