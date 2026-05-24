pub mod indoor;
pub mod outdoor;

pub use indoor::{AmbientLight, AmbientZone, DomeLight, InstrumentDisplayType, InstrumentLight};
pub use outdoor::{DaytimeRunning, FogLight, FogPosition, Headlight, HeadlightTechnology, Indicator, IndicatorPosition, Taillight, TaillightTechnology};
