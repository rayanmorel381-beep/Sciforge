//! Dispatch handler for biology functions.
//!
//! Delegates to sub-modules covering all biology sub-disciplines from
//! aging to virology.

mod aging;
mod bioelectricity;
mod bioenergetics;
mod biogeography;
mod bioinformatics;
mod biomechanics;
mod biophysics;
mod biostatistics;
mod cancer_biology;
mod cell;
mod chronobiology;
mod cryobiology;
mod developmental;
mod ecology;
mod endocrinology;
mod enzyme;
mod epigenetics;
mod ethology;
mod evolution;
mod genetics;
mod genomics;
mod immunology;
mod marine_biology;
mod microbiology;
mod mycology;
mod neuroscience;
mod nutrition;
mod paleobiology;
mod parasitology;
mod pharmacology;
mod phylogenetics;
mod physiology;
mod plant_biology;
mod population;
mod proteomics;
mod radiobiology;
mod reproduction;
mod stem_cell;
mod structural;
mod synthetic_biology;
mod systems_biology;
mod tissue_engineering;
mod toxicology;
mod virology;

use super::params::Params;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    aging::dispatch(func, p)
        .or_else(|_| bioelectricity::dispatch(func, p))
        .or_else(|_| bioenergetics::dispatch(func, p))
        .or_else(|_| biogeography::dispatch(func, p))
        .or_else(|_| bioinformatics::dispatch(func, p))
        .or_else(|_| biomechanics::dispatch(func, p))
        .or_else(|_| biophysics::dispatch(func, p))
        .or_else(|_| biostatistics::dispatch(func, p))
        .or_else(|_| cancer_biology::dispatch(func, p))
        .or_else(|_| cell::dispatch(func, p))
        .or_else(|_| chronobiology::dispatch(func, p))
        .or_else(|_| cryobiology::dispatch(func, p))
        .or_else(|_| developmental::dispatch(func, p))
        .or_else(|_| ecology::dispatch(func, p))
        .or_else(|_| endocrinology::dispatch(func, p))
        .or_else(|_| enzyme::dispatch(func, p))
        .or_else(|_| epigenetics::dispatch(func, p))
        .or_else(|_| ethology::dispatch(func, p))
        .or_else(|_| evolution::dispatch(func, p))
        .or_else(|_| genetics::dispatch(func, p))
        .or_else(|_| genomics::dispatch(func, p))
        .or_else(|_| immunology::dispatch(func, p))
        .or_else(|_| marine_biology::dispatch(func, p))
        .or_else(|_| microbiology::dispatch(func, p))
        .or_else(|_| mycology::dispatch(func, p))
        .or_else(|_| neuroscience::dispatch(func, p))
        .or_else(|_| nutrition::dispatch(func, p))
        .or_else(|_| paleobiology::dispatch(func, p))
        .or_else(|_| parasitology::dispatch(func, p))
        .or_else(|_| pharmacology::dispatch(func, p))
        .or_else(|_| phylogenetics::dispatch(func, p))
        .or_else(|_| physiology::dispatch(func, p))
        .or_else(|_| plant_biology::dispatch(func, p))
        .or_else(|_| population::dispatch(func, p))
        .or_else(|_| proteomics::dispatch(func, p))
        .or_else(|_| radiobiology::dispatch(func, p))
        .or_else(|_| reproduction::dispatch(func, p))
        .or_else(|_| stem_cell::dispatch(func, p))
        .or_else(|_| structural::dispatch(func, p))
        .or_else(|_| synthetic_biology::dispatch(func, p))
        .or_else(|_| systems_biology::dispatch(func, p))
        .or_else(|_| tissue_engineering::dispatch(func, p))
        .or_else(|_| toxicology::dispatch(func, p))
        .or_else(|_| virology::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("biology: unknown function: {func}")))
}
