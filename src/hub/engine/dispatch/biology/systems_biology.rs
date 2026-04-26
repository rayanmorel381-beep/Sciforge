//! Dispatch handler for systems biology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "sysbio_toggle_switch" => {
            let (a, b) = bio::systems_biology::bistability::toggle_switch(
                get_f(p, "u")?,
                get_f(p, "v")?,
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "bistable_toggle_switch" => Ok(RunOutput::PairVec(
            bio::systems_biology::bistability::bistable_toggle_switch(
                get_f(p, "u0")?,
                get_f(p, "v0")?,
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "n")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "ultrasensitivity_index" => Ok(RunOutput::Scalar(
            bio::systems_biology::bistability::ultrasensitivity_index(
                get_f(p, "ec10")?,
                get_f(p, "ec90")?,
            ),
        )),
        "network_robustness" => Ok(RunOutput::Scalar(
            bio::systems_biology::bistability::network_robustness(
                get_f(p, "output_nominal")?,
                get_f(p, "output_perturbed")?,
                get_f(p, "perturbation_fraction")?,
            ),
        )),
        "adaptation_precision" => Ok(RunOutput::Scalar(
            bio::systems_biology::bistability::adaptation_precision(
                get_f(p, "response_peak")?,
                get_f(p, "response_steady")?,
                get_f(p, "response_basal")?,
            ),
        )),
        "hysteresis_width" => Ok(RunOutput::Scalar(
            bio::systems_biology::bistability::hysteresis_width(
                get_v(p, "forward_thresholds")?,
                get_v(p, "backward_thresholds")?,
            ),
        )),
        "saddle_node_condition" => {
            let jacobian_m = get_m(p, "jacobian")?;
            let jacobian = [
                [jacobian_m[0][0], jacobian_m[0][1]],
                [jacobian_m[1][0], jacobian_m[1][1]],
            ];
            Ok(RunOutput::Scalar(
                bio::systems_biology::bistability::saddle_node_condition(&jacobian),
            ))
        }
        "stoichiometry_flux" => Ok(RunOutput::Vector(
            bio::systems_biology::flux::stoichiometry_flux(
                get_m(p, "stoichiometry")?,
                get_v(p, "fluxes")?,
            ),
        )),
        "fba_objective" => Ok(RunOutput::Scalar(
            bio::systems_biology::flux::fba_objective(get_v(p, "c")?, get_v(p, "fluxes")?),
        )),
        "flux_balance_feasibility" => Ok(RunOutput::Boolean(
            bio::systems_biology::flux::flux_balance_feasibility(
                get_m(p, "stoichiometry")?,
                get_v(p, "fluxes")?,
                get_f(p, "tolerance")?,
            ),
        )),
        "flux_variability" => {
            let (a, b) = bio::systems_biology::flux::flux_variability(
                get_f(p, "flux_nominal")?,
                get_f(p, "objective_fraction")?,
                get_f(p, "objective_optimal")?,
                get_f(p, "c_i")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "metabolic_sensitivity" => Ok(RunOutput::Scalar(
            bio::systems_biology::flux::metabolic_sensitivity(
                get_f(p, "flux")?,
                get_f(p, "parameter")?,
                get_f(p, "delta_flux")?,
                get_f(p, "delta_param")?,
            ),
        )),
        "dead_end_metabolites" => Ok(RunOutput::IntVector(
            bio::systems_biology::flux::dead_end_metabolites(get_m(p, "stoichiometry")?)
                .into_iter()
                .map(|x| x as i64)
                .collect(),
        )),
        "elementary_flux_mode_check" => Ok(RunOutput::Boolean(
            bio::systems_biology::flux::elementary_flux_mode_check(
                get_m(p, "stoichiometry")?,
                get_v(p, "mode")?,
                get_f(p, "tolerance")?,
            ),
        )),
        "yield_coefficient" => Ok(RunOutput::Scalar(
            bio::systems_biology::flux::yield_coefficient(
                get_f(p, "product_flux")?,
                get_f(p, "substrate_flux")?,
            ),
        )),
        "atp_yield" => Ok(RunOutput::Scalar(bio::systems_biology::flux::atp_yield(
            get_f(p, "atp_production_flux")?,
            get_f(p, "glucose_uptake_flux")?,
        ))),
        "shadow_price" => Ok(RunOutput::Scalar(bio::systems_biology::flux::shadow_price(
            get_f(p, "objective_change")?,
            get_f(p, "constraint_change")?,
        ))),
        "sysbio_repressilator" => {
            let m_v = get_v(p, "m")?;
            let m = [m_v[0], m_v[1], m_v[2]];
            let p_v = get_v(p, "p")?;
            let alpha = get_f(p, "alpha")?;
            let alpha0 = get_f(p, "alpha0")?;
            let beta = get_f(p, "beta")?;
            let n = get_f(p, "n")?;
            let p_arr = [p_v[0], p_v[1], p_v[2]];
            {
                let (a, b) = bio::systems_biology::networks::repressilator(
                    &m, &p_arr, alpha, alpha0, beta, n,
                );
                Ok(RunOutput::Matrix(vec![a.to_vec(), b.to_vec()]))
            }
        }
        "repressilator_simulate" => {
            let m0_v = get_v(p, "m0")?;
            let m0 = [m0_v[0], m0_v[1], m0_v[2]];
            let p0_v = get_v(p, "p0")?;
            let p0 = [p0_v[0], p0_v[1], p0_v[2]];
            Ok(RunOutput::Matrix(
                bio::systems_biology::networks::repressilator_simulate(
                    m0,
                    p0,
                    get_f(p, "alpha")?,
                    get_f(p, "alpha0")?,
                    get_f(p, "beta")?,
                    get_f(p, "n")?,
                    get_f(p, "dt")?,
                    get_u(p, "steps")?,
                )
                .into_iter()
                .flat_map(|(a, b)| vec![a.to_vec(), b.to_vec()])
                .collect(),
            ))
        }
        "genetic_toggle_switch" => {
            let (a, b) = bio::systems_biology::networks::genetic_toggle_switch(
                get_f(p, "u")?,
                get_f(p, "v")?,
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "toggle_switch_simulate" => Ok(RunOutput::PairVec(
            bio::systems_biology::networks::toggle_switch_simulate(
                get_f(p, "u0")?,
                get_f(p, "v0")?,
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "feed_forward_loop" => Ok(RunOutput::Scalar(
            bio::systems_biology::networks::feed_forward_loop(
                get_f(p, "input")?,
                get_f(p, "x")?,
                get_f(p, "k_xy")?,
                get_f(p, "k_iy")?,
                get_f(p, "n")?,
                get_b(p, "coherent")?,
            ),
        )),
        "robustness_coefficient" => Ok(RunOutput::Scalar(
            bio::systems_biology::networks::robustness_coefficient(
                get_f(p, "nominal_output")?,
                get_v(p, "perturbed_outputs")?,
            ),
        )),
        "goodwin_oscillator_step" => {
            let (a, b, c) = bio::systems_biology::oscillations::goodwin_oscillator_step(
                get_f(p, "mrna")?,
                get_f(p, "protein")?,
                get_f(p, "repressor")?,
                get_f(p, "dt")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "k3")?,
                get_f(p, "d1")?,
                get_f(p, "d2")?,
                get_f(p, "d3")?,
                get_f(p, "n")?,
                get_f(p, "ki")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "repressilator_step" => {
            let x_v = get_v(p, "x")?;
            let x = [x_v[0], x_v[1], x_v[2]];
            {
                let r = bio::systems_biology::oscillations::repressilator_step(
                    &x,
                    get_f(p, "dt")?,
                    get_f(p, "alpha")?,
                    get_f(p, "alpha0")?,
                    get_f(p, "beta")?,
                    get_f(p, "n")?,
                );
                Ok(RunOutput::Triple(r[0], r[1], r[2]))
            }
        }
        "oscillation_period" => Ok(RunOutput::Scalar(
            bio::systems_biology::oscillations::oscillation_period(
                get_v(p, "time_series")?,
                get_f(p, "dt")?,
            ),
        )),
        "oscillation_amplitude" => Ok(RunOutput::Scalar(
            bio::systems_biology::oscillations::oscillation_amplitude(get_v(p, "time_series")?),
        )),
        "fitzhugh_nagumo_step" => {
            let (a, b) = bio::systems_biology::oscillations::fitzhugh_nagumo_step(
                get_f(p, "v")?,
                get_f(p, "w")?,
                get_f(p, "dt")?,
                get_f(p, "i_ext")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "tau")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "entrainment_arnold_tongue" => Ok(RunOutput::Boolean(
            bio::systems_biology::oscillations::entrainment_arnold_tongue(
                get_f(p, "coupling_strength")?,
                get_f(p, "frequency_mismatch")?,
            ),
        )),
        "local_sensitivity" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::local_sensitivity(
                get_f(p, "output_perturbed")?,
                get_f(p, "output_base")?,
                get_f(p, "param_perturbed")?,
                get_f(p, "param_base")?,
            ),
        )),
        "morris_elementary_effect" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::morris_elementary_effect(
                get_f(p, "output_high")?,
                get_f(p, "output_base")?,
                get_f(p, "delta")?,
            ),
        )),
        "sobol_first_order" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::sobol_first_order(
                get_f(p, "variance_conditional")?,
                get_f(p, "variance_total")?,
            ),
        )),
        "sobol_total_order" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::sobol_total_order(
                get_f(p, "variance_remaining")?,
                get_f(p, "variance_total")?,
            ),
        )),
        "prcc_partial_rank_correlation" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::prcc_partial_rank_correlation(get_f(
                p,
                "r_xy_given_z",
            )?),
        )),
        "parameter_identifiability" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::parameter_identifiability(get_f(
                p,
                "fisher_information_diagonal",
            )?),
        )),
        "robustness_index" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::robustness_index(
                get_v(p, "outputs_perturbed")?,
                get_f(p, "output_nominal")?,
                get_f(p, "perturbation_range")?,
            ),
        )),
        "bifurcation_distance" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::bifurcation_distance(
                get_f(p, "parameter")?,
                get_f(p, "critical_value")?,
            ),
        )),
        "latin_hypercube_sample" => Ok(RunOutput::Matrix(
            bio::systems_biology::sensitivity::latin_hypercube_sample(
                get_u(p, "n_samples")?,
                get_u(p, "n_params")?,
            ),
        )),
        "sysbio_metabolic_control_coefficient" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::metabolic_control_coefficient(
                get_f(p, "flux_change")?,
                get_f(p, "flux_base")?,
                get_f(p, "enzyme_change")?,
                get_f(p, "enzyme_base")?,
            ),
        )),
        "metabolic_control_coefficient" => Ok(RunOutput::Scalar(
            bio::systems_biology::sensitivity::metabolic_control_coefficient(
                get_f(p, "flux_change")?,
                get_f(p, "flux_base")?,
                get_f(p, "enzyme_change")?,
                get_f(p, "enzyme_base")?,
            ),
        )),
        "phase_plane_nullcline_x" => Ok(RunOutput::Scalar(
            bio::systems_biology::oscillations::phase_plane_nullcline_x(
                get_f(p, "y")?,
                (get_f(p, "p0")?, get_f(p, "p1")?, get_f(p, "p2")?),
            ),
        )),
        "gene_regulatory_hill" => {
            let v = get_v(p, "activators")?;
            let r = get_v(p, "repressors")?;
            let act: Vec<(f64, f64, f64)> = v.chunks(3).map(|c| (c[0], c[1], c[2])).collect();
            let rep: Vec<(f64, f64, f64)> = r.chunks(3).map(|c| (c[0], c[1], c[2])).collect();
            Ok(RunOutput::Scalar(
                bio::systems_biology::networks::gene_regulatory_hill(
                    &act,
                    &rep,
                    get_f(p, "basal")?,
                ),
            ))
        }
        "repressilator" => {
            let mv = get_v(p, "m")?;
            let pv = get_v(p, "pr")?;
            let m_arr = [mv[0], mv[1], mv[2]];
            let p_arr = [pv[0], pv[1], pv[2]];
            let (mr, pr) = bio::systems_biology::networks::repressilator(
                &m_arr,
                &p_arr,
                get_f(p, "alpha")?,
                get_f(p, "alpha0")?,
                get_f(p, "beta")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Vector(vec![
                mr[0], mr[1], mr[2], pr[0], pr[1], pr[2],
            ]))
        }
        "boolean_network_step" => {
            let sv = get_v(p, "state")?;
            let state: Vec<bool> = sv.iter().map(|&x| x != 0.0).collect();
            let rm = get_m(p, "rules")?;
            let rules: Vec<Vec<(usize, bool)>> = rm
                .iter()
                .map(|r| r.chunks(2).map(|c| (c[0] as usize, c[1] != 0.0)).collect())
                .collect();
            let result = bio::systems_biology::networks::boolean_network_step(&state, &rules);
            Ok(RunOutput::Vector(
                result.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect(),
            ))
        }
        "boolean_network_simulate" => {
            let sv = get_v(p, "state")?;
            let state: Vec<bool> = sv.iter().map(|&x| x != 0.0).collect();
            let rm = get_m(p, "rules")?;
            let rules: Vec<Vec<(usize, bool)>> = rm
                .iter()
                .map(|r| r.chunks(2).map(|c| (c[0] as usize, c[1] != 0.0)).collect())
                .collect();
            let result = bio::systems_biology::networks::boolean_network_simulate(
                &state,
                &rules,
                get_u(p, "steps")?,
            );
            let flat: Vec<f64> = result
                .into_iter()
                .flatten()
                .map(|b| if b { 1.0 } else { 0.0 })
                .collect();
            Ok(RunOutput::Vector(flat))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
