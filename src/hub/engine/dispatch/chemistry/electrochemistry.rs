//! Dispatch handler for electrochemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "nernst_potential" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::nernst_potential(
                get_f(p, "e0")?,
                get_f(p, "n")?,
                get_f(p, "q")?,
                get_f(p, "t")?,
            ),
        )),
        "cell_potential" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::cell_potential(
                get_f(p, "e_cathode")?,
                get_f(p, "e_anode")?,
            ),
        )),
        "gibbs_from_cell_potential" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::gibbs_from_cell_potential(
                get_f(p, "n")?,
                get_f(p, "e_cell")?,
            ),
        )),
        "faraday_mass" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::faraday_mass(
                get_f(p, "i")?,
                get_f(p, "t")?,
                get_f(p, "m")?,
                get_f(p, "n")?,
            ),
        )),
        "overpotential_tafel" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::overpotential_tafel(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "current_density")?,
            ),
        )),
        "butler_volmer" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::butler_volmer(
                get_f(p, "i0")?,
                get_f(p, "alpha_a")?,
                get_f(p, "alpha_c")?,
                get_f(p, "eta")?,
                get_f(p, "t")?,
            ),
        )),
        "open_circuit_voltage" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::open_circuit_voltage(
                get_f(p, "e_cathode")?,
                get_f(p, "e_anode")?,
                get_f(p, "n_electrons")?,
                get_f(p, "t")?,
                get_f(p, "q")?,
            ),
        )),
        "faradaic_efficiency" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::faradaic_efficiency(
                get_f(p, "actual_yield")?,
                get_f(p, "theoretical_yield")?,
            ),
        )),
        "energy_density_battery" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::energy_density_battery(
                get_f(p, "voltage")?,
                get_f(p, "capacity_ah")?,
                get_f(p, "mass_kg")?,
            ),
        )),
        "coulombic_efficiency" => Ok(RunOutput::Scalar(
            chem::electrochemistry::cells::coulombic_efficiency(
                get_f(p, "discharge_capacity")?,
                get_f(p, "charge_capacity")?,
            ),
        )),

        "conductivity" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::conductivity(
                get_f(p, "resistance")?,
                get_f(p, "cell_constant")?,
            ),
        )),
        "molar_conductivity" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::molar_conductivity(
                get_f(p, "conductivity")?,
                get_f(p, "concentration")?,
            ),
        )),
        "kohlrausch" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::kohlrausch(
                get_f(p, "lambda_inf")?,
                get_f(p, "k")?,
                get_f(p, "c")?,
            ),
        )),
        "debye_huckel_activity" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::debye_huckel_activity(
                get_f(p, "z")?,
                get_f(p, "ionic_strength")?,
            ),
        )),
        "ionic_strength" => {
            let m = get_m(p, "ions")?;
            let ions: Vec<(f64, f64)> = m.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                chem::electrochemistry::transport::ionic_strength(&ions),
            ))
        }
        "transference_number" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::transference_number(
                get_f(p, "lambda_ion")?,
                get_f(p, "lambda_total")?,
            ),
        )),
        "debye_huckel_extended" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::debye_huckel_extended(
                get_f(p, "z")?,
                get_f(p, "ionic_strength")?,
                get_f(p, "a_ion")?,
            ),
        )),
        "onsager_equation" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::onsager_equation(
                get_f(p, "lambda_inf")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
            ),
        )),
        "walden_product" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::walden_product(
                get_f(p, "viscosity")?,
                get_f(p, "molar_conductivity")?,
            ),
        )),
        "mobility_from_conductivity" => Ok(RunOutput::Scalar(
            chem::electrochemistry::transport::mobility_from_conductivity(
                get_f(p, "lambda_ion")?,
                get_f(p, "z")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
