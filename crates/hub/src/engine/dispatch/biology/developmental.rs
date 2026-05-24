//! Dispatch handler for developmental biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "waddington_landscape" => Ok(RunOutput::Scalar(
            bio::developmental::differentiation::waddington_landscape(
                get_f(p, "x")?,
                get_f(p, "param")?,
            ),
        )),
        "differentiation_potential" => Ok(RunOutput::Scalar(
            bio::developmental::differentiation::differentiation_potential(
                get_f(p, "x")?,
                get_f(p, "param")?,
            ),
        )),
        "differentiation_simulate" => Ok(RunOutput::PairVec(
            bio::developmental::differentiation::differentiation_simulate(
                get_f(p, "x0")?,
                get_f(p, "param_start")?,
                get_f(p, "param_end")?,
                get_f(p, "noise")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "devel_somitogenesis_clock" => Ok(RunOutput::Scalar(
            bio::developmental::differentiation::somitogenesis_clock(
                get_f(p, "phase")?,
                get_f(p, "omega")?,
                get_f(p, "coupling")?,
                get_f(p, "neighbor_phase")?,
            ),
        )),
        "lateral_inhibition_step" => {
            let (a, b) = bio::developmental::differentiation::lateral_inhibition_step(
                get_f(p, "signal")?,
                get_f(p, "neighbor_signal")?,
                get_f(p, "delta")?,
                get_f(p, "notch")?,
                get_f(p, "k")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "devel_cell_fate_probability" => Ok(RunOutput::Scalar(
            bio::developmental::differentiation::cell_fate_probability(
                get_f(p, "signal")?,
                get_f(p, "threshold")?,
                get_f(p, "steepness")?,
            ),
        )),
        "devel_toggle_switch" => {
            let (a, b) = bio::developmental::gene_regulation::toggle_switch(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "apical_basal_polarity" => Ok(RunOutput::Scalar(
            bio::developmental::gene_regulation::apical_basal_polarity(
                get_f(p, "par3")?,
                get_f(p, "par6")?,
                get_f(p, "atypical_pkc")?,
                get_f(p, "par1")?,
            ),
        )),
        "planar_cell_polarity" => {
            let (a, b) = bio::developmental::gene_regulation::planar_cell_polarity(
                get_f(p, "frizzled")?,
                get_f(p, "vang")?,
                get_f(p, "coupling")?,
                get_f(p, "neighbor_fz")?,
                get_f(p, "neighbor_vang")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "notch_delta_lateral_inhibition_ode" => {
            let (a, b) = bio::developmental::gene_regulation::notch_delta_lateral_inhibition_ode(
                get_f(p, "notch")?,
                get_f(p, "delta")?,
                get_f(p, "neighbor_delta")?,
                get_f(p, "beta_n")?,
                get_f(p, "beta_d")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "induction_competence" => Ok(RunOutput::Scalar(
            bio::developmental::gene_regulation::induction_competence(
                get_f(p, "signal")?,
                get_f(p, "competence_window")?,
                get_f(p, "time")?,
                get_f(p, "window_center")?,
            ),
        )),
        "reaction_diffusion_activator_inhibitor" => {
            let (a, b) =
                bio::developmental::gene_regulation::reaction_diffusion_activator_inhibitor(
                    get_f(p, "a")?,
                    get_f(p, "h")?,
                    get_f(p, "da")?,
                    get_f(p, "rho_a")?,
                    get_f(p, "mu_a")?,
                    get_f(p, "dh")?,
                    get_f(p, "rho_h")?,
                    get_f(p, "mu_h")?,
                    get_f(p, "laplacian_a")?,
                    get_f(p, "laplacian_h")?,
                );
            Ok(RunOutput::Pair(a, b))
        }
        "morphogen_gradient_steady" => Ok(RunOutput::Scalar(
            bio::developmental::morphogens::morphogen_gradient_steady(
                get_f(p, "source")?,
                get_f(p, "decay")?,
                get_f(p, "diffusion")?,
                get_f(p, "x")?,
            ),
        )),
        "french_flag_model" => Ok(RunOutput::Integer(
            bio::developmental::morphogens::french_flag_model(
                get_f(p, "concentration")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ) as i64,
        )),
        "bicoid_gradient" => Ok(RunOutput::Scalar(
            bio::developmental::morphogens::bicoid_gradient(
                get_f(p, "c0")?,
                get_f(p, "length")?,
                get_f(p, "lambda")?,
                get_f(p, "x")?,
            ),
        )),
        "positional_information" => Ok(RunOutput::Text(
            String::from_utf8_lossy(&bio::developmental::morphogens::positional_information(
                get_v(p, "thresholds")?,
                get_v(p, "gradient")?,
            ))
            .into_owned(),
        )),
        "morphogen_noise_filtering" => Ok(RunOutput::Vector(
            bio::developmental::morphogens::morphogen_noise_filtering(
                get_v(p, "signal")?,
                get_u(p, "window")?,
            ),
        )),
        "interpretation_delay" => Ok(RunOutput::Scalar(
            bio::developmental::morphogens::interpretation_delay(
                get_v(p, "concentration_history")?,
                get_u(p, "delay")?,
            ),
        )),
        "wnt_gradient" => Ok(RunOutput::Vector(
            bio::developmental::morphogens::wnt_gradient(
                get_f(p, "source_strength")?,
                get_f(p, "decay_rate")?,
                get_v(p, "positions")?,
            ),
        )),
        "turing_instability_condition" => Ok(RunOutput::Boolean(
            bio::developmental::patterns::turing_instability_condition(
                get_f(p, "du")?,
                get_f(p, "dv")?,
                get_f(p, "fu")?,
                get_f(p, "gv")?,
                get_f(p, "fg_det")?,
            ),
        )),
        "french_flag_positional" => Ok(RunOutput::Integer(
            bio::developmental::patterns::french_flag_positional(
                get_f(p, "position")?,
                get_f(p, "threshold_high")?,
                get_f(p, "threshold_low")?,
                get_f(p, "morphogen_source")?,
                get_f(p, "decay_length")?,
            ) as i64,
        )),
        "clock_and_wavefront" => Ok(RunOutput::Boolean(
            bio::developmental::patterns::clock_and_wavefront(
                get_f(p, "position")?,
                get_f(p, "wavefront_speed")?,
                get_f(p, "frequency")?,
                get_f(p, "t")?,
                get_f(p, "threshold")?,
            ),
        )),
        "morphogenetic_field_potential" => Ok(RunOutput::Scalar(
            bio::developmental::gene_regulation::morphogenetic_field_potential(
                (get_f(p, "cx")?, get_f(p, "cy")?),
                (get_f(p, "fx")?, get_f(p, "fy")?),
                get_f(p, "field_strength")?,
                get_f(p, "decay")?,
            ),
        )),
        "toggle_switch" => {
            let (a, b) = bio::developmental::gene_regulation::toggle_switch(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "hox_gene_expression" => {
            let v = get_v(p, "boundaries")?;
            let bounds: Vec<(f64, f64)> = v.chunks(2).map(|c| (c[0], c[1])).collect();
            let result = bio::developmental::gene_regulation::hox_gene_expression(
                get_f(p, "position")?,
                &bounds,
            );
            let bools: Vec<f64> = result.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect();
            Ok(RunOutput::Vector(bools))
        }
        "shh_patterning" => {
            let v = get_v(p, "thresholds")?;
            let th: Vec<(f64, u8)> = v.chunks(2).map(|c| (c[0], c[1] as u8)).collect();
            Ok(RunOutput::Integer(
                bio::developmental::morphogens::shh_patterning(
                    get_f(p, "distance")?,
                    get_f(p, "concentration")?,
                    &th,
                ) as i64,
            ))
        }
        "morphogen_diffusion_1d" => {
            let mut conc = get_v(p, "conc")?.to_vec();
            bio::developmental::morphogens::morphogen_diffusion_1d(
                &mut conc,
                get_f(p, "d")?,
                get_f(p, "decay")?,
                get_u(p, "source_idx")?,
                get_f(p, "source_rate")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Vector(conc))
        }
        "morphogen_gradient_reaction" => {
            let mut conc = get_v(p, "conc")?.to_vec();
            let production = get_v(p, "production")?;
            bio::developmental::morphogens::morphogen_gradient_reaction(
                &mut conc,
                production,
                get_f(p, "degradation")?,
                get_f(p, "diffusion")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Vector(conc))
        }
        "turing_reaction_diffusion" => {
            let mut u = get_v(p, "u")?.to_vec();
            let mut v = get_v(p, "v")?.to_vec();
            bio::developmental::patterns::turing_reaction_diffusion(
                &mut u,
                &mut v,
                get_f(p, "du")?,
                get_f(p, "dv")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = u;
            out.extend(v);
            Ok(RunOutput::Vector(out))
        }
        "gierer_meinhardt" => {
            let mut a = get_v(p, "activator")?.to_vec();
            let mut i = get_v(p, "inhibitor")?.to_vec();
            bio::developmental::patterns::gierer_meinhardt(
                &mut a,
                &mut i,
                get_f(p, "da")?,
                get_f(p, "di")?,
                get_f(p, "rho_a")?,
                get_f(p, "rho_i")?,
                get_f(p, "mu_a")?,
                get_f(p, "mu_i")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = a;
            out.extend(i);
            Ok(RunOutput::Vector(out))
        }
        "schnakenberg" => {
            let mut u = get_v(p, "u")?.to_vec();
            let mut v = get_v(p, "v")?.to_vec();
            bio::developmental::patterns::schnakenberg(
                &mut u,
                &mut v,
                get_f(p, "du")?,
                get_f(p, "dv")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "gamma")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = u;
            out.extend(v);
            Ok(RunOutput::Vector(out))
        }
        "wave_pinning" => {
            let mut u = get_v(p, "u")?.to_vec();
            let mut v = get_v(p, "v")?.to_vec();
            bio::developmental::patterns::wave_pinning(
                &mut u,
                &mut v,
                get_f(p, "d")?,
                get_f(p, "k_on")?,
                get_f(p, "k_off")?,
                get_f(p, "k_fb")?,
                get_f(p, "hill_n")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = u;
            out.extend(v);
            Ok(RunOutput::Vector(out))
        }
        "lateral_inhibition" => {
            let mut cells = get_v(p, "cells")?.to_vec();
            let mut notch = get_v(p, "notch")?.to_vec();
            let mut delta = get_v(p, "delta")?.to_vec();
            bio::developmental::patterns::lateral_inhibition(
                &mut cells,
                &mut notch,
                &mut delta,
                get_f(p, "beta_n")?,
                get_f(p, "beta_d")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = cells;
            out.extend(notch);
            out.extend(delta);
            Ok(RunOutput::Vector(out))
        }
        "somite_clock_simulate" => {
            let mut phases = get_v(p, "phases")?.to_vec();
            let result = bio::developmental::differentiation::somite_clock_simulate(
                &mut phases,
                get_f(p, "omega")?,
                get_f(p, "coupling")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let flat: Vec<f64> = result.into_iter().flatten().collect();
            Ok(RunOutput::Vector(flat))
        }
        "voronoi_cell_sorting" => {
            let pv = get_v(p, "positions")?;
            let positions: Vec<(f64, f64)> = pv.chunks(2).map(|c| (c[0], c[1])).collect();
            let tv = get_v(p, "types")?;
            let types: Vec<u8> = tv.iter().map(|&x| x as u8).collect();
            let result = bio::developmental::patterns::voronoi_cell_sorting(
                &positions,
                &types,
                get_f(p, "adhesion_same")?,
                get_f(p, "adhesion_diff")?,
            );
            let flat: Vec<f64> = result.iter().flat_map(|(x, y)| vec![*x, *y]).collect();
            Ok(RunOutput::Vector(flat))
        }
        "gene_regulatory_network_step" => {
            let mut expr = get_v(p, "expression")?.to_vec();
            let interactions = get_m(p, "interactions")?;
            let basal = get_v(p, "basal_rates")?;
            let degradation = get_v(p, "degradation")?;
            bio::developmental::gene_regulation::gene_regulatory_network_step(
                &mut expr,
                interactions,
                basal,
                degradation,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(expr))
        }

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
