//! Dispatch handler for thermodynamics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "adiabatic_relation_pv" => Ok(RunOutput::Scalar(
            phys::thermodynamics::adiabatic_relation_pv(
                get_f(p, "p1")?,
                get_f(p, "v1")?,
                get_f(p, "v2")?,
                get_f(p, "gamma")?,
            ),
        )),
        "adiabatic_relation_tv" => Ok(RunOutput::Scalar(
            phys::thermodynamics::adiabatic_relation_tv(
                get_f(p, "t1")?,
                get_f(p, "v1")?,
                get_f(p, "v2")?,
                get_f(p, "gamma")?,
            ),
        )),
        "adiabatic_work" => Ok(RunOutput::Scalar(phys::thermodynamics::adiabatic_work(
            get_f(p, "n_moles")?,
            get_f(p, "cv")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "boltzmann_distribution" => Ok(RunOutput::Scalar(
            phys::thermodynamics::boltzmann_distribution(
                get_f(p, "energy")?,
                get_f(p, "temperature")?,
            ),
        )),
        "bose_einstein" => Ok(RunOutput::Scalar(phys::thermodynamics::bose_einstein(
            get_f(p, "energy")?,
            get_f(p, "mu")?,
            get_f(p, "temperature")?,
        ))),
        "carnot_cop_cooling" => Ok(RunOutput::Scalar(phys::thermodynamics::carnot_cop_cooling(
            get_f(p, "t_hot")?,
            get_f(p, "t_cold")?,
        ))),
        "carnot_cop_heating" => Ok(RunOutput::Scalar(phys::thermodynamics::carnot_cop_heating(
            get_f(p, "t_hot")?,
            get_f(p, "t_cold")?,
        ))),
        "carnot_efficiency" => Ok(RunOutput::Scalar(phys::thermodynamics::carnot_efficiency(
            get_f(p, "t_hot")?,
            get_f(p, "t_cold")?,
        ))),
        "chemical_potential_ideal_gas" => Ok(RunOutput::Scalar(
            phys::thermodynamics::chemical_potential_ideal_gas(
                get_f(p, "mu0")?,
                get_f(p, "temperature")?,
                get_f(p, "pressure")?,
                get_f(p, "p0")?,
            ),
        )),
        "clausius_clapeyron" => Ok(RunOutput::Scalar(phys::thermodynamics::clausius_clapeyron(
            get_f(p, "p1")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
            get_f(p, "delta_h_vap")?,
        ))),
        "compressibility_factor" => Ok(RunOutput::Scalar(
            phys::thermodynamics::compressibility_factor(
                get_f(p, "pressure")?,
                get_f(p, "volume")?,
                get_f(p, "n_moles")?,
                get_f(p, "temperature")?,
            ),
        )),
        "debye_heat_capacity" => Ok(RunOutput::Scalar(
            phys::thermodynamics::debye_heat_capacity(
                get_f(p, "temperature")?,
                get_f(p, "debye_temp")?,
                get_f(p, "n_atoms")?,
            ),
        )),
        "diesel_efficiency" => Ok(RunOutput::Scalar(phys::thermodynamics::diesel_efficiency(
            get_f(p, "compression_ratio")?,
            get_f(p, "cutoff_ratio")?,
            get_f(p, "gamma")?,
        ))),
        "einstein_heat_capacity" => Ok(RunOutput::Scalar(
            phys::thermodynamics::einstein_heat_capacity(
                get_f(p, "temperature")?,
                get_f(p, "einstein_temp")?,
                get_f(p, "n_atoms")?,
            ),
        )),
        "enthalpy_ideal" => Ok(RunOutput::Scalar(phys::thermodynamics::enthalpy_ideal(
            get_f(p, "n_moles")?,
            get_f(p, "cp")?,
            get_f(p, "temperature")?,
        ))),
        "entropy_canonical" => Ok(RunOutput::Scalar(phys::thermodynamics::entropy_canonical(
            get_v(p, "energies")?,
            get_f(p, "temperature")?,
        ))),
        "entropy_ideal_gas" => Ok(RunOutput::Scalar(phys::thermodynamics::entropy_ideal_gas(
            get_f(p, "n_moles")?,
            get_f(p, "cv")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
            get_f(p, "v1")?,
            get_f(p, "v2")?,
        ))),
        "equilibrium_constant" => Ok(RunOutput::Scalar(
            phys::thermodynamics::equilibrium_constant(
                get_f(p, "delta_g0")?,
                get_f(p, "temperature")?,
            ),
        )),
        "thermodynamics::fermi_dirac" => Ok(RunOutput::Scalar(phys::thermodynamics::fermi_dirac(
            get_f(p, "energy")?,
            get_f(p, "mu")?,
            get_f(p, "temperature")?,
        ))),
        "thermodynamics::fermi_energy" => Ok(RunOutput::Scalar(
            phys::thermodynamics::fermi_energy(get_f(p, "mass")?, get_f(p, "number_density")?),
        )),
        "gibbs_free_energy" => Ok(RunOutput::Scalar(phys::thermodynamics::gibbs_free_energy(
            get_f(p, "enthalpy")?,
            get_f(p, "temperature")?,
            get_f(p, "entropy")?,
        ))),
        "heat_capacity_ratio" => Ok(RunOutput::Scalar(
            phys::thermodynamics::heat_capacity_ratio(get_f(p, "cp")?, get_f(p, "cv")?),
        )),
        "heat_conduction_rate" => Ok(RunOutput::Scalar(
            phys::thermodynamics::heat_conduction_rate(
                get_f(p, "k")?,
                get_f(p, "area")?,
                get_f(p, "dt")?,
                get_f(p, "dx")?,
            ),
        )),
        "helmholtz_free_energy" => Ok(RunOutput::Scalar(
            phys::thermodynamics::helmholtz_free_energy(
                get_f(p, "internal_energy")?,
                get_f(p, "temperature")?,
                get_f(p, "entropy")?,
            ),
        )),
        "ideal_gas_pressure" => Ok(RunOutput::Scalar(phys::thermodynamics::ideal_gas_pressure(
            get_f(p, "n_moles")?,
            get_f(p, "temperature")?,
            get_f(p, "volume")?,
        ))),
        "ideal_gas_temperature" => Ok(RunOutput::Scalar(
            phys::thermodynamics::ideal_gas_temperature(
                get_f(p, "pressure")?,
                get_f(p, "volume")?,
                get_f(p, "n_moles")?,
            ),
        )),
        "ideal_gas_volume" => Ok(RunOutput::Scalar(phys::thermodynamics::ideal_gas_volume(
            get_f(p, "n_moles")?,
            get_f(p, "temperature")?,
            get_f(p, "pressure")?,
        ))),
        "internal_energy_ideal" => Ok(RunOutput::Scalar(
            phys::thermodynamics::internal_energy_ideal(
                get_f(p, "n_moles")?,
                get_f(p, "cv")?,
                get_f(p, "temperature")?,
            ),
        )),
        "isobaric_work" => Ok(RunOutput::Scalar(phys::thermodynamics::isobaric_work(
            get_f(p, "pressure")?,
            get_f(p, "v1")?,
            get_f(p, "v2")?,
        ))),
        "isothermal_work" => Ok(RunOutput::Scalar(phys::thermodynamics::isothermal_work(
            get_f(p, "n_moles")?,
            get_f(p, "temperature")?,
            get_f(p, "v1")?,
            get_f(p, "v2")?,
        ))),
        "joule_thomson_coefficient" => Ok(RunOutput::Scalar(
            phys::thermodynamics::joule_thomson_coefficient(
                get_f(p, "cp")?,
                get_f(p, "v_molar")?,
                get_f(p, "temperature")?,
                get_f(p, "dv_dt_p")?,
            ),
        )),
        "maxwell_speed_distribution" => Ok(RunOutput::Scalar(
            phys::thermodynamics::maxwell_speed_distribution(
                get_f(p, "v")?,
                get_f(p, "mass")?,
                get_f(p, "temperature")?,
            ),
        )),
        "mean_energy_canonical" => Ok(RunOutput::Scalar(
            phys::thermodynamics::mean_energy_canonical(
                get_v(p, "energies")?,
                get_f(p, "temperature")?,
            ),
        )),
        "thermodynamics::mean_free_path" => {
            Ok(RunOutput::Scalar(phys::thermodynamics::mean_free_path(
                get_f(p, "number_density")?,
                get_f(p, "cross_section")?,
            )))
        }
        "mean_speed" => Ok(RunOutput::Scalar(phys::thermodynamics::mean_speed(
            get_f(p, "mass")?,
            get_f(p, "temperature")?,
        ))),
        "most_probable_speed" => Ok(RunOutput::Scalar(
            phys::thermodynamics::most_probable_speed(get_f(p, "mass")?, get_f(p, "temperature")?),
        )),
        "otto_efficiency" => Ok(RunOutput::Scalar(phys::thermodynamics::otto_efficiency(
            get_f(p, "compression_ratio")?,
            get_f(p, "gamma")?,
        ))),
        "partition_function_discrete" => Ok(RunOutput::Scalar(
            phys::thermodynamics::partition_function_discrete(
                get_v(p, "energies")?,
                get_f(p, "temperature")?,
            ),
        )),
        "partition_function_harmonic" => Ok(RunOutput::Scalar(
            phys::thermodynamics::partition_function_harmonic(
                get_f(p, "omega")?,
                get_f(p, "temperature")?,
            ),
        )),
        "planck_radiation" => Ok(RunOutput::Scalar(phys::thermodynamics::planck_radiation(
            get_f(p, "frequency")?,
            get_f(p, "temperature")?,
        ))),
        "planck_radiation_wavelength" => Ok(RunOutput::Scalar(
            phys::thermodynamics::planck_radiation_wavelength(
                get_f(p, "wavelength")?,
                get_f(p, "temperature")?,
            ),
        )),
        "reaction_gibbs" => Ok(RunOutput::Scalar(phys::thermodynamics::reaction_gibbs(
            get_f(p, "delta_g0")?,
            get_f(p, "temperature")?,
            get_f(p, "q")?,
        ))),
        "redlich_kwong_pressure" => Ok(RunOutput::Scalar(
            phys::thermodynamics::redlich_kwong_pressure(
                get_f(p, "n_moles")?,
                get_f(p, "temperature")?,
                get_f(p, "volume")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "rms_speed" => Ok(RunOutput::Scalar(phys::thermodynamics::rms_speed(
            get_f(p, "mass")?,
            get_f(p, "temperature")?,
        ))),
        "sackur_tetrode" => Ok(RunOutput::Scalar(phys::thermodynamics::sackur_tetrode(
            get_f(p, "n_moles")?,
            get_f(p, "temperature")?,
            get_f(p, "volume")?,
            get_f(p, "mass")?,
        ))),
        "speed_of_sound_ideal" => Ok(RunOutput::Scalar(
            phys::thermodynamics::speed_of_sound_ideal(
                get_f(p, "gamma")?,
                get_f(p, "temperature")?,
                get_f(p, "molar_mass")?,
            ),
        )),
        "stefan_boltzmann_power" => Ok(RunOutput::Scalar(
            phys::thermodynamics::stefan_boltzmann_power(
                get_f(p, "temperature")?,
                get_f(p, "area")?,
            ),
        )),
        "throttling_temperature_change" => Ok(RunOutput::Scalar(
            phys::thermodynamics::throttling_temperature_change(
                get_f(p, "mu_jt")?,
                get_f(p, "dp")?,
            ),
        )),
        "van_der_waals_pressure" => Ok(RunOutput::Scalar(
            phys::thermodynamics::van_der_waals_pressure(
                get_f(p, "n_moles")?,
                get_f(p, "temperature")?,
                get_f(p, "volume")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "vant_hoff" => Ok(RunOutput::Scalar(phys::thermodynamics::vant_hoff(
            get_f(p, "k1")?,
            get_f(p, "delta_h")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "virial_eos" => Ok(RunOutput::Scalar(phys::thermodynamics::virial_eos(
            get_f(p, "pressure")?,
            get_f(p, "temperature")?,
            get_f(p, "b2")?,
            get_f(p, "b3")?,
        ))),
        "wien_displacement" => Ok(RunOutput::Scalar(phys::thermodynamics::wien_displacement(
            get_f(p, "temperature")?,
        ))),
        "mixing_entropy" => Ok(RunOutput::Scalar(phys::thermodynamics::mixing_entropy(
            get_v(p, "mole_fractions")?,
        ))),
        "thermal_diffusion_1d" => {
            let mut t = get_v(p, "t")?.to_vec();
            phys::thermodynamics::thermal_diffusion_1d(
                &mut t,
                get_f(p, "alpha")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Vector(t))
        }
        "pressure_atm_to_pascal" => Ok(RunOutput::Scalar(
            phys::thermodynamics::pressure_atm_to_pascal(get_f(p, "p_atm")?),
        )),
        "pressure_pascal_to_atm" => Ok(RunOutput::Scalar(
            phys::thermodynamics::pressure_pascal_to_atm(get_f(p, "p_pa")?),
        )),
        "pressure_bar_to_pascal" => Ok(RunOutput::Scalar(
            phys::thermodynamics::pressure_bar_to_pascal(get_f(p, "p_bar")?),
        )),
        "energy_calories_to_joules" => Ok(RunOutput::Scalar(
            phys::thermodynamics::energy_calories_to_joules(get_f(p, "cal")?),
        )),
        "energy_joules_to_calories" => Ok(RunOutput::Scalar(
            phys::thermodynamics::energy_joules_to_calories(get_f(p, "j")?),
        )),
        "energy_kwh_to_joules" => Ok(RunOutput::Scalar(
            phys::thermodynamics::energy_kwh_to_joules(get_f(p, "kwh")?),
        )),
        "plasma_temperature_kev_to_kelvin" => Ok(RunOutput::Scalar(
            phys::thermodynamics::plasma_temperature_kev_to_kelvin(get_f(p, "t_kev")?),
        )),
        "ideal_gas_pressure_atm" => Ok(RunOutput::Scalar(
            phys::thermodynamics::ideal_gas_pressure_atm(
                get_f(p, "n_moles")?,
                get_f(p, "temperature")?,
                get_f(p, "volume_liters")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
