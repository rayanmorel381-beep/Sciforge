//! Dispatch handler for electronics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "and_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::and_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "binary_to_gray" => Ok(RunOutput::Scalar(
            phys::electronics::digital::binary_to_gray(get_i(p, "binary")? as u32) as f64,
        )),
        "bjt_alpha" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::bjt_alpha(get_f(p, "beta")?),
        )),
        "bjt_ic_active" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::bjt_ic_active(
                get_f(p, "beta")?,
                get_f(p, "ib")?,
            ),
        )),
        "bjt_ie" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::bjt_ie(get_f(p, "ic")?, get_f(p, "ib")?),
        )),
        "capacitor_energy" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::capacitor_energy(get_f(p, "c")?, get_f(p, "v")?),
        )),
        "cascaded_gain" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::cascaded_gain(get_v(p, "gains_db")?),
        )),
        "common_emitter_voltage_gain" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::common_emitter_voltage_gain(
                get_f(p, "gm")?,
                get_f(p, "r_c")?,
            ),
        )),
        "current_divider" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::current_divider(
                get_f(p, "i_total")?,
                get_f(p, "r_branch")?,
                get_f(p, "r_total_parallel")?,
            ),
        )),
        "d_flip_flop" => Ok(RunOutput::Boolean(phys::electronics::digital::d_flip_flop(
            get_f(p, "d")? != 0.0,
            get_f(p, "_clk_edge")? != 0.0,
        ))),
        "decibel_power" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::decibel_power(get_f(p, "p_out")?, get_f(p, "p_in")?),
        )),
        "decibel_voltage" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::decibel_voltage(get_f(p, "v_out")?, get_f(p, "v_in")?),
        )),
        "differential_gain" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::differential_gain(get_f(p, "r_f")?, get_f(p, "r_in")?),
        )),
        "differentiator_output" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::differentiator_output(
                get_f(p, "dv_dt")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
            ),
        )),
        "diode_shockley" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::diode_shockley(
                get_f(p, "is")?,
                get_f(p, "v")?,
                get_f(p, "n")?,
                get_f(p, "vt")?,
            ),
        )),
        "drain_induced_barrier_lowering" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::drain_induced_barrier_lowering(
                get_f(p, "vth0")?,
                get_f(p, "sigma")?,
                get_f(p, "vds")?,
            ),
        )),
        "early_effect" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::early_effect(
                get_f(p, "ic0")?,
                get_f(p, "vce")?,
                get_f(p, "va")?,
            ),
        )),
        "friis_noise_factor" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::friis_noise_factor(
                get_v(p, "factors")?,
                get_v(p, "gains")?,
            ),
        )),
        "gain_bandwidth_product" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::gain_bandwidth_product(
                get_f(p, "gain")?,
                get_f(p, "bandwidth")?,
            ),
        )),
        "gray_to_binary" => Ok(RunOutput::Scalar(
            phys::electronics::digital::gray_to_binary(get_i(p, "gray")? as u32) as f64,
        )),
        "impedance_capacitor" => {
            let r =
                phys::electronics::circuits::impedance_capacitor(get_f(p, "c")?, get_f(p, "freq")?);
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "impedance_inductor" => {
            let r =
                phys::electronics::circuits::impedance_inductor(get_f(p, "l")?, get_f(p, "freq")?);
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "electronics::circuits::impedance_magnitude" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::impedance_magnitude(get_f(p, "re")?, get_f(p, "im")?),
        )),
        "impedance_phase" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::impedance_phase(get_f(p, "re")?, get_f(p, "im")?),
        )),
        "inductor_energy" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::inductor_energy(get_f(p, "l")?, get_f(p, "i")?),
        )),
        "integrator_output" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::integrator_output(
                get_f(p, "v_in")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
                get_f(p, "t")?,
            ),
        )),
        "inverting_gain" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::inverting_gain(get_f(p, "r_f")?, get_f(p, "r_in")?),
        )),
        "jk_flip_flop" => Ok(RunOutput::Boolean(
            phys::electronics::digital::jk_flip_flop(
                get_f(p, "j")? != 0.0,
                get_f(p, "k")? != 0.0,
                get_f(p, "q_prev")? != 0.0,
            ),
        )),
        "led_resistor" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::led_resistor(
                get_f(p, "v_supply")?,
                get_f(p, "v_led")?,
                get_f(p, "i_led")?,
            ),
        )),
        "max_power_transfer" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::max_power_transfer(get_f(p, "v_th")?, get_f(p, "r_th")?),
        )),
        "mosfet_drain_current_linear" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::mosfet_drain_current_linear(
                get_f(p, "kn")?,
                get_f(p, "vgs")?,
                get_f(p, "vth")?,
                get_f(p, "vds")?,
            ),
        )),
        "mosfet_drain_current_saturation" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::mosfet_drain_current_saturation(
                get_f(p, "kn")?,
                get_f(p, "vgs")?,
                get_f(p, "vth")?,
            ),
        )),
        "mosfet_threshold_body_effect" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::mosfet_threshold_body_effect(
                get_f(p, "vth0")?,
                get_f(p, "gamma")?,
                get_f(p, "vsb")?,
                get_f(p, "phi")?,
            ),
        )),
        "multiplexer_2to1" => Ok(RunOutput::Boolean(
            phys::electronics::digital::multiplexer_2to1(
                get_f(p, "a")? != 0.0,
                get_f(p, "b")? != 0.0,
                get_f(p, "sel")? != 0.0,
            ),
        )),
        "nand_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::nand_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "noise_figure" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::noise_figure(get_f(p, "snr_in")?, get_f(p, "snr_out")?),
        )),
        "non_inverting_gain" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::non_inverting_gain(get_f(p, "r_f")?, get_f(p, "r_in")?),
        )),
        "nor_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::nor_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "not_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::not_gate(
            get_f(p, "a")? != 0.0,
        ))),
        "ohm_current" => Ok(RunOutput::Scalar(phys::electronics::circuits::ohm_current(
            get_f(p, "v")?,
            get_f(p, "r")?,
        ))),
        "ohm_resistance" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::ohm_resistance(get_f(p, "v")?, get_f(p, "i")?),
        )),
        "ohm_voltage" => Ok(RunOutput::Scalar(phys::electronics::circuits::ohm_voltage(
            get_f(p, "i")?,
            get_f(p, "r")?,
        ))),
        "ones_complement" => Ok(RunOutput::Scalar(
            phys::electronics::digital::ones_complement(
                get_i(p, "val")? as u32,
                get_i(p, "bits")? as u32,
            ) as f64,
        )),
        "or_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::or_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "electronics::circuits::parallel_resistance" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::parallel_resistance(get_v(p, "resistors")?),
        )),
        "photodiode_responsivity" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::photodiode_responsivity(
                get_f(p, "i_photo")?,
                get_f(p, "p_optical")?,
            ),
        )),
        "pn_junction_capacitance" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::pn_junction_capacitance(
                get_f(p, "c0")?,
                get_f(p, "v")?,
                get_f(p, "v_bi")?,
                get_f(p, "m")?,
            ),
        )),
        "power_dc" => Ok(RunOutput::Scalar(phys::electronics::circuits::power_dc(
            get_f(p, "v")?,
            get_f(p, "i")?,
        ))),
        "electronics::circuits::rc_charging" => {
            Ok(RunOutput::Scalar(phys::electronics::circuits::rc_charging(
                get_f(p, "v_source")?,
                get_f(p, "t")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
            )))
        }
        "electronics::circuits::rc_discharging" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rc_discharging(
                get_f(p, "v0")?,
                get_f(p, "t")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
            ),
        )),
        "rl_current_decay" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rl_current_decay(
                get_f(p, "i0")?,
                get_f(p, "r")?,
                get_f(p, "l")?,
                get_f(p, "t")?,
            ),
        )),
        "rl_current_rise" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rl_current_rise(
                get_f(p, "v")?,
                get_f(p, "r")?,
                get_f(p, "l")?,
                get_f(p, "t")?,
            ),
        )),
        "rlc_bandwidth" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rlc_bandwidth(get_f(p, "f0")?, get_f(p, "q")?),
        )),
        "rlc_quality_factor" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rlc_quality_factor(
                get_f(p, "r")?,
                get_f(p, "l")?,
                get_f(p, "c")?,
            ),
        )),
        "rlc_resonant_frequency" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::rlc_resonant_frequency(get_f(p, "l")?, get_f(p, "c")?),
        )),
        "electronics::circuits::series_resistance" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::series_resistance(get_v(p, "resistors")?),
        )),
        "solar_cell_iv" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::solar_cell_iv(
                get_f(p, "i_photo")?,
                get_f(p, "i0")?,
                get_f(p, "v")?,
                get_f(p, "n")?,
                get_f(p, "vt")?,
                get_f(p, "r_s")?,
            ),
        )),
        "sr_latch" => Ok(RunOutput::Boolean(phys::electronics::digital::sr_latch(
            get_f(p, "s")? != 0.0,
            get_f(p, "r")? != 0.0,
            get_f(p, "q_prev")? != 0.0,
        ))),
        "summing_amplifier" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::summing_amplifier(
                get_v(p, "v_inputs")?,
                get_v(p, "r_inputs")?,
                get_f(p, "r_f")?,
            ),
        )),
        "thermal_voltage" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::thermal_voltage(get_f(p, "temperature_k")?),
        )),
        "thevenin_resistance" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::thevenin_resistance(get_f(p, "v_oc")?, get_f(p, "i_sc")?),
        )),
        "thevenin_voltage" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::thevenin_voltage(get_f(p, "v_oc")?),
        )),
        "transconductance" => Ok(RunOutput::Scalar(
            phys::electronics::amplifiers::transconductance(get_f(p, "i_c")?, get_f(p, "v_t")?),
        )),
        "tunnel_diode_current" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::tunnel_diode_current(
                get_f(p, "ip")?,
                get_f(p, "iv")?,
                get_f(p, "vp")?,
                get_f(p, "vv")?,
                get_f(p, "v")?,
            ),
        )),
        "twos_complement" => Ok(RunOutput::Scalar(
            phys::electronics::digital::twos_complement(
                get_i(p, "val")? as u32,
                get_i(p, "bits")? as u32,
            ) as f64,
        )),
        "electronics::circuits::voltage_divider" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::voltage_divider(
                get_f(p, "v_in")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
            ),
        )),
        "wheatstone_bridge_voltage" => Ok(RunOutput::Scalar(
            phys::electronics::circuits::wheatstone_bridge_voltage(
                get_f(p, "v_in")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "r3")?,
                get_f(p, "r4")?,
            ),
        )),
        "xnor_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::xnor_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "xor_gate" => Ok(RunOutput::Boolean(phys::electronics::digital::xor_gate(
            get_f(p, "a")? != 0.0,
            get_f(p, "b")? != 0.0,
        ))),
        "zener_voltage_regulation" => Ok(RunOutput::Scalar(
            phys::electronics::semiconductor_devices::zener_voltage_regulation(
                get_f(p, "v_in")?,
                get_f(p, "v_zener")?,
            ),
        )),
        "decoder_2to4" => {
            let r = phys::electronics::digital::decoder_2to4(
                get_f(p, "a")? != 0.0,
                get_f(p, "b")? != 0.0,
            );
            Ok(RunOutput::Vector(
                r.iter().map(|&b| if b { 1.0 } else { 0.0 }).collect(),
            ))
        }
        "demultiplexer_1to2" => {
            let r = phys::electronics::digital::demultiplexer_1to2(
                get_f(p, "input")? != 0.0,
                get_f(p, "sel")? != 0.0,
            );
            Ok(RunOutput::Pair(
                if r.0 { 1.0 } else { 0.0 },
                if r.1 { 1.0 } else { 0.0 },
            ))
        }
        "encoder_4to2" => {
            let v = get_v(p, "inputs")?;
            let arr = [v[0] != 0.0, v[1] != 0.0, v[2] != 0.0, v[3] != 0.0];
            let r = phys::electronics::digital::encoder_4to2(&arr);
            Ok(RunOutput::Pair(
                if r.0 { 1.0 } else { 0.0 },
                if r.1 { 1.0 } else { 0.0 },
            ))
        }
        "full_adder" => {
            let r = phys::electronics::digital::full_adder(
                get_f(p, "a")? != 0.0,
                get_f(p, "b")? != 0.0,
                get_f(p, "cin")? != 0.0,
            );
            Ok(RunOutput::Pair(
                if r.0 { 1.0 } else { 0.0 },
                if r.1 { 1.0 } else { 0.0 },
            ))
        }
        "half_adder" => {
            let r = phys::electronics::digital::half_adder(
                get_f(p, "a")? != 0.0,
                get_f(p, "b")? != 0.0,
            );
            Ok(RunOutput::Pair(
                if r.0 { 1.0 } else { 0.0 },
                if r.1 { 1.0 } else { 0.0 },
            ))
        }
        "ripple_carry_adder" => {
            let a: Vec<bool> = get_v(p, "a")?.iter().map(|&x| x != 0.0).collect();
            let b: Vec<bool> = get_v(p, "b")?.iter().map(|&x| x != 0.0).collect();
            let (sum, carry) = phys::electronics::digital::ripple_carry_adder(&a, &b);
            let mut r: Vec<f64> = sum.iter().map(|&s| if s { 1.0 } else { 0.0 }).collect();
            r.push(if carry { 1.0 } else { 0.0 });
            Ok(RunOutput::Vector(r))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
