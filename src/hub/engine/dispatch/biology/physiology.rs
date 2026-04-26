//! Dispatch handler for physiology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "frank_starling_mechanism" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::frank_starling_mechanism(
                get_f(p, "end_diastolic_volume")?,
                get_f(p, "contractility")?,
                get_f(p, "max_stroke_volume")?,
            ),
        )),
        "stroke_volume_cardiac_output" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::stroke_volume_cardiac_output(
                get_f(p, "heart_rate")?,
                get_f(p, "stroke_volume")?,
            ),
        )),
        "physio_ejection_fraction" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::ejection_fraction(
                get_f(p, "stroke_volume")?,
                get_f(p, "end_diastolic_volume")?,
            ),
        )),
        "map_calculation" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::map_calculation(
                get_f(p, "systolic")?,
                get_f(p, "diastolic")?,
            ),
        )),
        "physio_systemic_vascular_resistance" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::systemic_vascular_resistance(
                get_f(p, "map")?,
                get_f(p, "cvp")?,
                get_f(p, "cardiac_output")?,
            ),
        )),
        "myocardial_oxygen_consumption" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::myocardial_oxygen_consumption(
                get_f(p, "heart_rate")?,
                get_f(p, "systolic_bp")?,
            ),
        )),
        "windkessel_pressure" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::windkessel_pressure(
                get_f(p, "cardiac_output")?,
                get_f(p, "resistance")?,
                get_f(p, "compliance")?,
                get_f(p, "t")?,
                get_f(p, "heart_rate")?,
            ),
        )),
        "coronary_flow_reserve" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::coronary_flow_reserve(
                get_f(p, "hyperemic_flow")?,
                get_f(p, "resting_flow")?,
            ),
        )),
        "qt_correction_bazett" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::qt_correction_bazett(get_f(p, "qt_ms")?, get_f(p, "rr_ms")?),
        )),
        "cardiac_work" => Ok(RunOutput::Scalar(bio::physiology::cardiac::cardiac_work(
            get_f(p, "stroke_volume_ml")?,
            get_f(p, "mean_pressure_mmhg")?,
        ))),
        "preload_recruitable_stroke_work" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::preload_recruitable_stroke_work(
                get_f(p, "stroke_work")?,
                get_f(p, "edv")?,
                get_f(p, "v0")?,
            ),
        )),
        "physio_poiseuille_flow" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::poiseuille_flow(
                get_f(p, "radius")?,
                get_f(p, "length")?,
                get_f(p, "pressure_drop")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "physio_wall_shear_stress" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::wall_shear_stress(
                get_f(p, "viscosity")?,
                get_f(p, "flow_rate")?,
                get_f(p, "radius")?,
            ),
        )),
        "physio_mean_arterial_pressure" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::mean_arterial_pressure(
                get_f(p, "systolic")?,
                get_f(p, "diastolic")?,
            ),
        )),
        "physio_cardiac_output" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::cardiac_output(
                get_f(p, "stroke_volume")?,
                get_f(p, "heart_rate")?,
            ),
        )),
        "physio_total_peripheral_resistance" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::total_peripheral_resistance(
                get_f(p, "map")?,
                get_f(p, "cvp")?,
                get_f(p, "cardiac_output")?,
            ),
        )),
        "frank_starling" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::frank_starling(
                get_f(p, "preload")?,
                get_f(p, "k")?,
                get_f(p, "max_force")?,
            ),
        )),
        "physio_pulse_wave_velocity" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::pulse_wave_velocity(
                get_f(p, "elasticity")?,
                get_f(p, "wall_thickness")?,
                get_f(p, "radius")?,
                get_f(p, "density")?,
            ),
        )),
        "windkessel_2" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::windkessel_2(
                get_f(p, "flow")?,
                get_f(p, "pressure")?,
                get_f(p, "resistance")?,
                get_f(p, "compliance")?,
            ),
        )),
        "glomerular_filtration_rate" => Ok(RunOutput::Scalar(
            bio::physiology::renal::glomerular_filtration_rate(
                get_f(p, "kf")?,
                get_f(p, "p_gc")?,
                get_f(p, "p_bs")?,
                get_f(p, "pi_gc")?,
            ),
        )),
        "creatinine_clearance" => Ok(RunOutput::Scalar(
            bio::physiology::renal::creatinine_clearance(
                get_f(p, "urine_cr")?,
                get_f(p, "urine_volume")?,
                get_f(p, "plasma_cr")?,
            ),
        )),
        "fractional_excretion" => Ok(RunOutput::Scalar(
            bio::physiology::renal::fractional_excretion(
                get_f(p, "urine_x")?,
                get_f(p, "plasma_cr")?,
                get_f(p, "plasma_x")?,
                get_f(p, "urine_cr")?,
            ),
        )),
        "free_water_clearance" => Ok(RunOutput::Scalar(
            bio::physiology::renal::free_water_clearance(
                get_f(p, "urine_volume")?,
                get_f(p, "urine_osm")?,
                get_f(p, "plasma_osm")?,
            ),
        )),
        "tubular_reabsorption_rate" => Ok(RunOutput::Scalar(
            bio::physiology::renal::tubular_reabsorption_rate(
                get_f(p, "filtered_load")?,
                get_f(p, "excretion_rate")?,
            ),
        )),
        "cockcroft_gault" => Ok(RunOutput::Scalar(bio::physiology::renal::cockcroft_gault(
            get_f(p, "age")?,
            get_f(p, "weight")?,
            get_f(p, "serum_cr")?,
            get_b(p, "is_female")?,
        ))),
        "mdrd_gfr" => Ok(RunOutput::Scalar(bio::physiology::renal::mdrd_gfr(
            get_f(p, "serum_cr")?,
            get_f(p, "age")?,
            get_b(p, "is_female")?,
            get_b(p, "is_black")?,
        ))),
        "tubuloglomerular_feedback" => Ok(RunOutput::Scalar(
            bio::physiology::renal::tubuloglomerular_feedback(
                get_f(p, "nacl_macula")?,
                get_f(p, "nacl_target")?,
                get_f(p, "sensitivity")?,
                get_f(p, "gfr_baseline")?,
            ),
        )),
        "urine_concentration_ratio" => Ok(RunOutput::Scalar(
            bio::physiology::renal::urine_concentration_ratio(
                get_f(p, "urine_osm")?,
                get_f(p, "plasma_osm")?,
            ),
        )),
        "anion_gap" => Ok(RunOutput::Scalar(bio::physiology::renal::anion_gap(
            get_f(p, "sodium")?,
            get_f(p, "chloride")?,
            get_f(p, "bicarbonate")?,
        ))),
        "tidal_volume" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::tidal_volume(
                get_f(p, "respiratory_rate")?,
                get_f(p, "minute_ventilation")?,
            ),
        )),
        "alveolar_ventilation" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::alveolar_ventilation(
                get_f(p, "tidal_volume")?,
                get_f(p, "dead_space")?,
                get_f(p, "rate")?,
            ),
        )),
        "alveolar_gas_equation" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::alveolar_gas_equation(
                get_f(p, "fio2")?,
                get_f(p, "p_atm")?,
                get_f(p, "p_h2o")?,
                get_f(p, "paco2")?,
                get_f(p, "rq")?,
            ),
        )),
        "airway_resistance" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::airway_resistance(
                get_f(p, "pressure_drop")?,
                get_f(p, "flow")?,
            ),
        )),
        "lung_compliance" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::lung_compliance(
                get_f(p, "volume_change")?,
                get_f(p, "pressure_change")?,
            ),
        )),
        "oxygen_content" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::oxygen_content(
                get_f(p, "hb")?,
                get_f(p, "sao2")?,
                get_f(p, "pao2")?,
            ),
        )),
        "oxygen_delivery" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::oxygen_delivery(
                get_f(p, "cardiac_output")?,
                get_f(p, "cao2")?,
            ),
        )),
        "fick_oxygen_consumption" => Ok(RunOutput::Scalar(
            bio::physiology::respiratory::fick_oxygen_consumption(
                get_f(p, "cardiac_output")?,
                get_f(p, "cao2")?,
                get_f(p, "cvo2")?,
            ),
        )),
        "heat_balance" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::heat_balance(
                get_f(p, "metabolic_rate")?,
                get_f(p, "work")?,
                get_f(p, "radiation")?,
                get_f(p, "convection")?,
                get_f(p, "evaporation")?,
            ),
        )),
        "newton_cooling" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::newton_cooling(
                get_f(p, "body_temp")?,
                get_f(p, "ambient_temp")?,
                get_f(p, "h")?,
                get_f(p, "surface_area")?,
            ),
        )),
        "evaporative_heat_loss" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::evaporative_heat_loss(
                get_f(p, "sweat_rate")?,
                get_f(p, "latent_heat")?,
            ),
        )),
        "core_temperature_regulation" => {
            let (a, b) = bio::physiology::thermoregulation::core_temperature_regulation(
                get_f(p, "set_point")?,
                get_f(p, "core_temp")?,
                get_f(p, "gain_shiver")?,
                get_f(p, "gain_sweat")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "wind_chill_index" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::wind_chill_index(
                get_f(p, "air_temp")?,
                get_f(p, "wind_speed_kmh")?,
            ),
        )),
        "heat_index" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::heat_index(
                get_f(p, "temperature_f")?,
                get_f(p, "relative_humidity")?,
            ),
        )),
        "physio_body_surface_area_dubois" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::body_surface_area_dubois(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
            ),
        )),
        "counter_current_heat_exchange" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::counter_current_heat_exchange(
                get_f(p, "arterial_temp")?,
                get_f(p, "venous_temp_return")?,
                get_f(p, "efficiency")?,
            ),
        )),
        "brown_adipose_tissue_thermogenesis" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::brown_adipose_tissue_thermogenesis(
                get_f(p, "bat_mass")?,
                get_f(p, "ucp1_activity")?,
                get_f(p, "substrate_availability")?,
            ),
        )),
        "physio_metabolic_rate_q10" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::metabolic_rate_q10(
                get_f(p, "rate_ref")?,
                get_f(p, "temp")?,
                get_f(p, "temp_ref")?,
                get_f(p, "q10")?,
            ),
        )),
        "cardiac_output" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::cardiac_output(
                get_f(p, "stroke_volume")?,
                get_f(p, "heart_rate")?,
            ),
        )),
        "ejection_fraction" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::ejection_fraction(
                get_f(p, "stroke_volume")?,
                get_f(p, "end_diastolic_volume")?,
            ),
        )),
        "mean_arterial_pressure" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::mean_arterial_pressure(
                get_f(p, "systolic")?,
                get_f(p, "diastolic")?,
            ),
        )),
        "metabolic_rate_q10" => Ok(RunOutput::Scalar(
            bio::physiology::thermoregulation::metabolic_rate_q10(
                get_f(p, "rate_ref")?,
                get_f(p, "temp")?,
                get_f(p, "temp_ref")?,
                get_f(p, "q10")?,
            ),
        )),
        "poiseuille_flow" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::poiseuille_flow(
                get_f(p, "radius")?,
                get_f(p, "length")?,
                get_f(p, "pressure_drop")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "pulse_wave_velocity" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::pulse_wave_velocity(
                get_f(p, "elasticity")?,
                get_f(p, "wall_thickness")?,
                get_f(p, "radius")?,
                get_f(p, "density")?,
            ),
        )),
        "systemic_vascular_resistance" => Ok(RunOutput::Scalar(
            bio::physiology::cardiac::systemic_vascular_resistance(
                get_f(p, "map")?,
                get_f(p, "cvp")?,
                get_f(p, "cardiac_output")?,
            ),
        )),
        "total_peripheral_resistance" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::total_peripheral_resistance(
                get_f(p, "map")?,
                get_f(p, "cvp")?,
                get_f(p, "cardiac_output")?,
            ),
        )),
        "wall_shear_stress" => Ok(RunOutput::Scalar(
            bio::physiology::hemodynamics::wall_shear_stress(
                get_f(p, "viscosity")?,
                get_f(p, "flow_rate")?,
                get_f(p, "radius")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
