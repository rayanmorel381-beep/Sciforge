# Physics Source Code Guide

This page documents the source implementation behind `sciforge::physics`, including module layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/physics/`.
- Main computation groups and where they are implemented.
- Runtime call path when physics functions are executed through Hub dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/physics/`
- Module entry point: `src/physics/mod.rs`
- Hub routing (when applicable): `src/hub/engine/dispatch/physics.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `acoustics`
- `electrodynamics`
- `electronics`
- `fluid_mechanics`
- `materials`
- `nucleosynthesis`
- `optics`
- `particle`
- `quantum`
- `relativity`
- `solid_mechanics`
- `thermodynamics`

---

## Particle API (39 functions)

| Function | Signature |
|----------|-----------|
| `planck_energy` | `() -> f64` |
| `planck_density` | `() -> f64` |
| `planck_force` | `() -> f64` |
| `planck_pressure` | `() -> f64` |
| `planck_temperature` | `() -> f64` |
| `planck_charge` | `() -> f64` |
| `planck_impedance` | `() -> f64` |
| `planck_angular_frequency` | `() -> f64` |
| `schwarzschild_radius_planck` | `() -> f64` |
| `hawking_temperature` | `(mass: f64) -> f64` |
| `hawking_luminosity` | `(mass: f64) -> f64` |
| `hawking_evaporation_time` | `(mass: f64) -> f64` |
| `unruh_temperature` | `(acceleration: f64) -> f64` |
| `fermi_coupling_constant` | `() -> f64` |
| `weak_decay_rate` | `(coupling: f64, mass: f64) -> f64` |
| `muon_decay_width` | `() -> f64` |
| `fine_structure_constant` | `() -> f64` |
| `strong_coupling_constant` | `() -> f64` |
| `qcd_running_coupling` | `(q2: f64) -> f64` |
| `electromagnetic_coupling_running` | `(q2: f64) -> f64` |
| `weak_mixing_angle` | `() -> f64` |
| `w_boson_mass_gev` | `() -> f64` |
| `z_boson_mass_gev` | `() -> f64` |
| `higgs_vev_gev` | `() -> f64` |
| `compton_time` | `(mass: f64) -> f64` |
| `gravitational_coupling` | `() -> f64` |
| `photon_energy` | `(frequency: f64) -> f64` |
| `photon_momentum` | `(frequency: f64) -> f64` |
| `pair_production_threshold_energy` | `(mass: f64) -> f64` |
| `cross_section_thomson` | `() -> f64` |
| `neutrino_mass_upper_bound` | `() -> f64` |
| `neutrino_de_broglie_wavelength` | `(mass: f64, velocity: f64) -> f64` |
| `classical_electron_radius` | `() -> f64` |
| `bohr_velocity` | `() -> f64` |
| `schwinger_critical_field` | `() -> f64` |

---

## Acoustics API (48 functions)

### Submodules
- `absorption` — sound absorption coefficients, acoustic impedance
- `doppler` — Doppler shift, observer/source velocity effects
- `propagation` — wave propagation, attenuation, speed in media
- `resonance` — standing waves, resonance frequencies, Q-factor

### absorption (10 functions)

| Function | Signature |
|----------|-----------|
| `absorption_coefficient` | `(frequency: f64, humidity: f64) -> f64` |
| `intensity_after_absorption` | `(i0: f64, alpha: f64, distance: f64) -> f64` |
| `atmospheric_absorption` | `(frequency: f64, temperature: f64, humidity: f64) -> f64` |
| `noise_reduction_coefficient` | `(alphas: &[f64]) -> f64` |
| `sound_transmission_class` | `(tl_values: &[f64]) -> f64` |
| `mass_law_transmission_loss` | `(surface_density: f64, frequency: f64) -> f64` |
| `porous_absorber_flow_resistivity` | `(porosity: f64, fiber_diameter: f64) -> f64` |
| `a_weighting` | `(frequency: f64) -> f64` |
| `decibel_addition` | `(levels: &[f64]) -> f64` |
| `room_constant` | `(total_absorption: f64, surface_area: f64) -> f64` |

### doppler (8 functions)

| Function | Signature |
|----------|-----------|
| `doppler_approaching` | `(f0: f64, v_sound: f64, v_observer: f64, v_source: f64) -> f64` |
| `doppler_receding` | `(f0: f64, v_sound: f64, v_observer: f64, v_source: f64) -> f64` |
| `doppler_general` | `(f0: f64, v_sound: f64, v_observer: f64, v_source: f64) -> f64` |
| `relativistic_doppler` | `(f0: f64, v: f64) -> f64` |
| `mach_cone_angle` | `(v: f64, v_sound: f64) -> f64` |
| `doppler_shift_wavelength` | `(lambda0: f64, v_source: f64, v_sound: f64) -> f64` |
| `doppler_radar_velocity` | `(f_shift: f64, f_transmitted: f64, v_sound: f64) -> f64` |
| `sonic_boom_pressure` | `(altitude: f64, mach: f64, aircraft_length: f64) -> f64` |

### propagation (10 functions)

| Function | Signature |
|----------|-----------|
| `speed_of_sound_gas` | `(gamma: f64, pressure: f64, density: f64) -> f64` |
| `speed_of_sound_solid` | `(e: f64, density: f64) -> f64` |
| `wavelength` | `(speed: f64, frequency: f64) -> f64` |
| `intensity` | `(power: f64, area: f64) -> f64` |
| `intensity_level_db` | `(intensity: f64, reference: f64) -> f64` |
| `sound_pressure_level` | `(pressure: f64, reference: f64) -> f64` |
| `inverse_square_law` | `(i0: f64, r0: f64, r: f64) -> f64` |
| `acoustic_impedance` | `(density: f64, speed: f64) -> f64` |
| `transmission_coefficient` | `(z1: f64, z2: f64) -> f64` |
| `reflection_coefficient` | `(z1: f64, z2: f64) -> f64` |

### resonance (11 functions)

| Function | Signature |
|----------|-----------|
| `fundamental_frequency_string` | `(length: f64, tension: f64, linear_density: f64) -> f64` |
| `harmonic_frequency` | `(fundamental: f64, n: u32) -> f64` |
| `resonant_frequency_pipe_open` | `(length: f64, speed: f64, n: u32) -> f64` |
| `resonant_frequency_pipe_closed` | `(length: f64, speed: f64, n: u32) -> f64` |
| `quality_factor` | `(f_resonant: f64, bandwidth: f64) -> f64` |
| `helmholtz_resonator` | `(speed: f64, neck_area: f64, neck_length: f64, volume: f64) -> f64` |
| `beat_frequency` | `(f1: f64, f2: f64) -> f64` |
| `standing_wave_nodes` | `(length: f64, n: u32) -> Vec<f64>` |
| `reverberation_time_sabine` | `(volume: f64, total_absorption: f64) -> f64` |
| `room_mode_frequency` | `(nx: u32, ny: u32, nz: u32, lx: f64, ly: f64, lz: f64, speed: f64) -> f64` |
| `schroeder_frequency` | `(rt60: f64, volume: f64) -> f64` |

---

## Electrodynamics API (36 functions + struct `RlcCircuit`)

### Submodules
- `circuits` — RC/RL/RLC, impedance, power
- `fields` — electric/magnetic field computations
- `waves` — electromagnetic wave properties, Poynting vector

### circuits (18 functions + struct)

| Function | Signature |
|----------|-----------|
| `RlcCircuit::new` | `(r: f64, l: f64, c: f64) -> Self` |
| `RlcCircuit::resonant_frequency` | `(&self) -> f64` |
| `RlcCircuit::quality_factor` | `(&self) -> f64` |
| `RlcCircuit::impedance` | `(&self, omega: f64) -> (f64, f64)` |
| `RlcCircuit::impedance_magnitude` | `(&self, omega: f64) -> f64` |
| `RlcCircuit::phase_angle` | `(&self, omega: f64) -> f64` |
| `RlcCircuit::damping_ratio` | `(&self) -> f64` |
| `RlcCircuit::bandwidth` | `(&self) -> f64` |
| `RlcCircuit::transient_response` | `(&self, t: f64, v0: f64) -> f64` |
| `rc_time_constant` | `(r: f64, c: f64) -> f64` |
| `rl_time_constant` | `(r: f64, l: f64) -> f64` |
| `rc_charging` | `(v_source: f64, r: f64, c: f64, t: f64) -> f64` |
| `rc_discharging` | `(v0: f64, r: f64, c: f64, t: f64) -> f64` |
| `power_dissipated` | `(v: f64, r: f64) -> f64` |
| `parallel_resistance` | `(resistances: &[f64]) -> f64` |
| `series_resistance` | `(resistances: &[f64]) -> f64` |
| `parallel_capacitance` | `(capacitances: &[f64]) -> f64` |
| `series_capacitance` | `(capacitances: &[f64]) -> f64` |
| `voltage_divider` | `(v_in: f64, r1: f64, r2: f64) -> f64` |
| `wheatstone_bridge_balance` | `(r1: f64, r2: f64, r3: f64) -> f64` |
| `energy_capacitor` | `(c: f64, v: f64) -> f64` |
| `energy_inductor` | `(l: f64, i: f64) -> f64` |
| `mutual_inductance_coupling` | `(k: f64, l1: f64, l2: f64) -> f64` |
| `transformer_ratio` | `(n_primary: f64, n_secondary: f64, v_primary: f64) -> f64` |

### fields (17 functions)

| Function | Signature |
|----------|-----------|
| `electric_field_point_charge` | `(q: f64, r: [f64; 3]) -> [f64; 3]` |
| `electric_potential_point` | `(q: f64, r: f64) -> f64` |
| `magnetic_field_wire` | `(current: f64, r: f64) -> f64` |
| `magnetic_field_solenoid` | `(n_per_length: f64, current: f64) -> f64` |
| `magnetic_field_loop` | `(current: f64, radius: f64, z: f64) -> f64` |
| `biot_savart_segment` | `(current: f64, dl: [f64; 3], r: [f64; 3]) -> [f64; 3]` |
| `lorentz_force` | `(q: f64, v: [f64; 3], e: [f64; 3], b: [f64; 3]) -> [f64; 3]` |
| `poynting_vector` | `(e: [f64; 3], b: [f64; 3]) -> [f64; 3]` |
| `energy_density_em` | `(e: [f64; 3], b: [f64; 3]) -> f64` |
| `electric_dipole_field` | `(p: [f64; 3], r: [f64; 3]) -> [f64; 3]` |
| `magnetic_dipole_field` | `(m: [f64; 3], r: [f64; 3]) -> [f64; 3]` |
| `capacitance_parallel_plate` | `(area: f64, distance: f64, epsilon_r: f64) -> f64` |
| `inductance_solenoid` | `(n_turns: f64, length: f64, area: f64) -> f64` |
| `cyclotron_frequency` | `(charge: f64, mass: f64, b: f64) -> f64` |
| `larmor_radius` | `(mass: f64, v_perp: f64, charge: f64, b: f64) -> f64` |
| `plasma_frequency` | `(number_density: f64, mass: f64, charge: f64) -> f64` |
| `debye_length` | `(temperature: f64, number_density: f64, charge: f64) -> f64` |

### waves (17 functions)

| Function | Signature |
|----------|-----------|
| `wave_impedance_free_space` | `() -> f64` |
| `wave_number` | `(frequency: f64) -> f64` |
| `wavelength` | `(frequency: f64) -> f64` |
| `phase_velocity` | `(epsilon_r: f64, mu_r: f64) -> f64` |
| `group_velocity_dispersive` | `(v_phase: f64, omega: f64, dv_domega: f64) -> f64` |
| `skin_depth` | `(frequency: f64, conductivity: f64, mu_r: f64) -> f64` |
| `fresnel_rs` | `(n1: f64, n2: f64, theta_i: f64) -> f64` |
| `fresnel_rp` | `(n1: f64, n2: f64, theta_i: f64) -> f64` |
| `brewster_angle` | `(n1: f64, n2: f64) -> f64` |
| `critical_angle` | `(n1: f64, n2: f64) -> Option<f64>` |
| `snell` | `(n1: f64, theta1: f64, n2: f64) -> f64` |
| `radiation_pressure_absorbed` | `(intensity: f64) -> f64` |
| `radiation_pressure_reflected` | `(intensity: f64) -> f64` |
| `larmor_radiation_power` | `(charge: f64, accel: f64) -> f64` |
| `antenna_radiation_resistance_dipole` | `(length: f64, wavelength: f64) -> f64` |
| `fdtd_1d` | `(ez: &mut [f64], hy: &mut [f64], steps: usize)` |
| `waveguide_cutoff_te` | `(m: u32, n: u32, a: f64, b: f64) -> f64` |

---

## Electronics API (42 functions)

### Submodules
- `amplifiers` — gain, bandwidth, noise figure
- `circuits` — Kirchhoff laws, node analysis, thevenin/norton
- `digital` — logic gates, binary operations
- `semiconductor_devices` — diodes, transistors, MOSFET

### amplifiers (15 functions)

| Function | Signature |
|----------|-----------|
| `inverting_gain` | `(r_f: f64, r_in: f64) -> f64` |
| `non_inverting_gain` | `(r_f: f64, r_in: f64) -> f64` |
| `differential_gain` | `(r_f: f64, r_in: f64) -> f64` |
| `summing_amplifier` | `(v_inputs: &[f64], r_inputs: &[f64], r_f: f64) -> f64` |
| `integrator_output` | `(v_in: f64, r: f64, c: f64, t: f64) -> f64` |
| `differentiator_output` | `(dv_dt: f64, r: f64, c: f64) -> f64` |
| `gain_bandwidth_product` | `(gain: f64, bandwidth: f64) -> f64` |
| `common_emitter_voltage_gain` | `(gm: f64, r_c: f64) -> f64` |
| `transconductance` | `(i_c: f64, v_t: f64) -> f64` |
| `thermal_voltage` | `(temperature_k: f64) -> f64` |
| `decibel_voltage` | `(v_out: f64, v_in: f64) -> f64` |
| `decibel_power` | `(p_out: f64, p_in: f64) -> f64` |
| `cascaded_gain` | `(gains_db: &[f64]) -> f64` |
| `noise_figure` | `(snr_in: f64, snr_out: f64) -> f64` |
| `friis_noise_factor` | `(factors: &[f64], gains: &[f64]) -> f64` |

### circuits (24 functions)

| Function | Signature |
|----------|-----------|
| `ohm_voltage` | `(i: f64, r: f64) -> f64` |
| `ohm_current` | `(v: f64, r: f64) -> f64` |
| `ohm_resistance` | `(v: f64, i: f64) -> f64` |
| `series_resistance` | `(resistors: &[f64]) -> f64` |
| `parallel_resistance` | `(resistors: &[f64]) -> f64` |
| `voltage_divider` | `(v_in: f64, r1: f64, r2: f64) -> f64` |
| `current_divider` | `(i_total: f64, r_branch: f64, r_total_parallel: f64) -> f64` |
| `power_dc` | `(v: f64, i: f64) -> f64` |
| `rc_charging` | `(v_source: f64, t: f64, r: f64, c: f64) -> f64` |
| `rc_discharging` | `(v0: f64, t: f64, r: f64, c: f64) -> f64` |
| `rl_current_rise` | `(v: f64, r: f64, l: f64, t: f64) -> f64` |
| `rl_current_decay` | `(i0: f64, r: f64, l: f64, t: f64) -> f64` |
| `rlc_resonant_frequency` | `(l: f64, c: f64) -> f64` |
| `rlc_quality_factor` | `(r: f64, l: f64, c: f64) -> f64` |
| `rlc_bandwidth` | `(f0: f64, q: f64) -> f64` |
| `impedance_capacitor` | `(c: f64, freq: f64) -> (f64, f64)` |
| `impedance_inductor` | `(l: f64, freq: f64) -> (f64, f64)` |
| `impedance_magnitude` | `(re: f64, im: f64) -> f64` |
| `impedance_phase` | `(re: f64, im: f64) -> f64` |
| `capacitor_energy` | `(c: f64, v: f64) -> f64` |
| `inductor_energy` | `(l: f64, i: f64) -> f64` |
| `wheatstone_bridge_voltage` | `(v_in: f64, r1: f64, r2: f64, r3: f64, r4: f64) -> f64` |
| `thevenin_voltage` | `(v_oc: f64) -> f64` |
| `thevenin_resistance` | `(v_oc: f64, i_sc: f64) -> f64` |
| `max_power_transfer` | `(v_th: f64, r_th: f64) -> f64` |

### digital (23 functions)

| Function | Signature |
|----------|-----------|
| `and_gate` | `(a: bool, b: bool) -> bool` |
| `or_gate` | `(a: bool, b: bool) -> bool` |
| `not_gate` | `(a: bool) -> bool` |
| `nand_gate` | `(a: bool, b: bool) -> bool` |
| `nor_gate` | `(a: bool, b: bool) -> bool` |
| `xor_gate` | `(a: bool, b: bool) -> bool` |
| `xnor_gate` | `(a: bool, b: bool) -> bool` |
| `half_adder` | `(a: bool, b: bool) -> (bool, bool)` |
| `full_adder` | `(a: bool, b: bool, cin: bool) -> (bool, bool)` |
| `ripple_carry_adder` | `(a: &[bool], b: &[bool]) -> (Vec<bool>, bool)` |
| `multiplexer_2to1` | `(a: bool, b: bool, sel: bool) -> bool` |
| `demultiplexer_1to2` | `(input: bool, sel: bool) -> (bool, bool)` |
| `decoder_2to4` | `(a: bool, b: bool) -> [bool; 4]` |
| `encoder_4to2` | `(inputs: &[bool; 4]) -> (bool, bool)` |
| `sr_latch` | `(s: bool, r: bool, q_prev: bool) -> bool` |
| `d_flip_flop` | `(d: bool, _: bool) -> bool` |
| `jk_flip_flop` | `(j: bool, k: bool, q_prev: bool) -> bool` |
| `binary_to_gray` | `(binary: u32) -> u32` |
| `gray_to_binary` | `(gray: u32) -> u32` |
| `ones_complement` | `(val: u32, bits: u32) -> u32` |
| `twos_complement` | `(val: u32, bits: u32) -> u32` |

### semiconductor_devices (16 functions)

| Function | Signature |
|----------|-----------|
| `diode_shockley` | `(is: f64, v: f64, n: f64, vt: f64) -> f64` |
| `zener_voltage_regulation` | `(v_in: f64, v_zener: f64) -> f64` |
| `bjt_ic_active` | `(beta: f64, ib: f64) -> f64` |
| `bjt_ie` | `(ic: f64, ib: f64) -> f64` |
| `bjt_alpha` | `(beta: f64) -> f64` |
| `mosfet_drain_current_saturation` | `(kn: f64, vgs: f64, vth: f64) -> f64` |
| `mosfet_drain_current_linear` | `(kn: f64, vgs: f64, vth: f64, vds: f64) -> f64` |
| `mosfet_threshold_body_effect` | `(vth0: f64, gamma: f64, vsb: f64, phi: f64) -> f64` |
| `solar_cell_iv` | `(i_photo: f64, i0: f64, v: f64, n: f64, vt: f64, r_s: f64) -> f64` |
| `led_resistor` | `(v_supply: f64, v_led: f64, i_led: f64) -> f64` |
| `photodiode_responsivity` | `(i_photo: f64, p_optical: f64) -> f64` |
| `tunnel_diode_current` | `(ip: f64, iv: f64, vp: f64, vv: f64, v: f64) -> f64` |
| `pn_junction_capacitance` | `(c0: f64, v: f64, v_bi: f64, m: f64) -> f64` |
| `early_effect` | `(ic0: f64, vce: f64, va: f64) -> f64` |
| `drain_induced_barrier_lowering` | `(vth0: f64, sigma: f64, vds: f64) -> f64` |

---

## Fluid Mechanics API (36 functions)

### Submodules
- `boundary_layer` — Blasius, Reynolds, displacement/momentum thickness
- `flow` — continuity, Bernoulli, pipe flow, Poiseuille
- `turbulence` — Reynolds stress, turbulence intensity, mixing length
- `waves` — surface waves, dispersion, wave speed

### boundary_layer (12 functions)

| Function | Signature |
|----------|-----------|
| `blasius_thickness` | `(x: f64, re_x: f64) -> f64` |
| `displacement_thickness` | `(x: f64, re_x: f64) -> f64` |
| `momentum_thickness` | `(x: f64, re_x: f64) -> f64` |
| `shape_factor` | `(displacement: f64, momentum: f64) -> f64` |
| `skin_friction_laminar` | `(re_x: f64) -> f64` |
| `skin_friction_turbulent` | `(re_x: f64) -> f64` |
| `turbulent_bl_thickness` | `(x: f64, re_x: f64) -> f64` |
| `separation_criterion` | `(dp_dx: f64) -> bool` |
| `falkner_skan_beta` | `(m: f64) -> f64` |
| `thermal_bl_thickness` | `(delta: f64, pr: f64) -> f64` |
| `nusselt_flat_plate_laminar` | `(re: f64, pr: f64) -> f64` |
| `stanton_number` | `(nu: f64, re: f64, pr: f64) -> f64` |

### flow (12 functions)

| Function | Signature |
|----------|-----------|
| `reynolds_number` | `(rho: f64, v: f64, l: f64, mu: f64) -> f64` |
| `bernoulli_pressure` | `(rho: f64, v1: f64, p1: f64, v2: f64) -> f64` |
| `bernoulli_height` | `(rho: f64, v1: f64, p1: f64, h1: f64, v2: f64, p2: f64, g: f64) -> f64` |
| `hagen_poiseuille` | `(delta_p: f64, r: f64, l: f64, mu: f64) -> f64` |
| `continuity_velocity` | `(a1: f64, v1: f64, a2: f64) -> f64` |
| `drag_force` | `(cd: f64, rho: f64, v: f64, a: f64) -> f64` |
| `lift_force` | `(cl: f64, rho: f64, v: f64, a: f64) -> f64` |
| `stokes_drag` | `(mu: f64, r: f64, v: f64) -> f64` |
| `terminal_velocity_sphere` | `(rho_p: f64, rho_f: f64, r: f64, mu: f64, g: f64) -> f64` |
| `torricelli` | `(g: f64, h: f64) -> f64` |
| `hydraulic_diameter` | `(area: f64, perimeter: f64) -> f64` |
| `darcy_weisbach` | `(f: f64, l: f64, d: f64, rho: f64, v: f64) -> f64` |

### turbulence (12 functions)

| Function | Signature |
|----------|-----------|
| `turbulent_kinetic_energy` | `(u_prime: f64, v_prime: f64, w_prime: f64) -> f64` |
| `kolmogorov_length_scale` | `(nu: f64, epsilon: f64) -> f64` |
| `kolmogorov_time_scale` | `(nu: f64, epsilon: f64) -> f64` |
| `kolmogorov_velocity_scale` | `(nu: f64, epsilon: f64) -> f64` |
| `taylor_microscale` | `(u_rms: f64, epsilon: f64, nu: f64) -> f64` |
| `integral_length_scale` | `(tke: f64, epsilon: f64) -> f64` |
| `friction_velocity` | `(tau_wall: f64, rho: f64) -> f64` |
| `law_of_wall` | `(u_tau: f64, y: f64, nu: f64) -> f64` |
| `mixing_length` | `(kappa: f64, y: f64) -> f64` |
| `eddy_viscosity` | `(mixing_length: f64, du_dy: f64) -> f64` |
| `turbulence_intensity` | `(u_rms: f64, u_mean: f64) -> f64` |
| `energy_spectrum_kolmogorov` | `(c_k: f64, epsilon: f64, k: f64) -> f64` |

### waves (11 functions)

| Function | Signature |
|----------|-----------|
| `shallow_water_speed` | `(g: f64, depth: f64) -> f64` |
| `deep_water_speed` | `(g: f64, wavelength: f64) -> f64` |
| `wave_number` | `(wavelength: f64) -> f64` |
| `wave_frequency` | `(period: f64) -> f64` |
| `froude_number` | `(v: f64, g: f64, depth: f64) -> f64` |
| `mach_number` | `(v: f64, c: f64) -> f64` |
| `sound_speed_ideal_gas` | `(gamma: f64, r: f64, t: f64, m: f64) -> f64` |
| `water_hammer_pressure` | `(rho: f64, c: f64, delta_v: f64) -> f64` |
| `capillary_number` | `(mu: f64, v: f64, sigma: f64) -> f64` |
| `weber_number` | `(rho: f64, v: f64, l: f64, sigma: f64) -> f64` |
| `wave_energy_density` | `(rho: f64, g: f64, amplitude: f64) -> f64` |

---

## Materials API (45 functions)

### Submodules
- `crystallography` — lattice parameters, Miller indices, diffraction
- `diffusion` — Fick's laws, diffusion coefficient
- `phases` — phase diagrams, lever rule, Gibbs phase rule
- `semiconductors` — carrier density, bandgap, doping

### crystallography (11 functions)

| Function | Signature |
|----------|-----------|
| `bragg_angle` | `(d: f64, wavelength: f64, n: i32) -> f64` |
| `d_spacing_cubic` | `(a: f64, h: i32, k: i32, l: i32) -> f64` |
| `miller_planar_density_cubic` | `(a: f64, atoms_per_plane: f64, h: i32, k: i32, l: i32) -> f64` |
| `packing_fraction_bcc` | `() -> f64` |
| `packing_fraction_fcc` | `() -> f64` |
| `lattice_parameter_from_density` | `(m: f64, z: f64, rho: f64) -> f64` |
| `structure_factor_bcc` | `(h: i32, k: i32, l: i32) -> f64` |
| `structure_factor_fcc` | `(h: i32, k: i32, l: i32) -> f64` |
| `debye_temperature` | `(theta_d: f64, t: f64) -> f64` |
| `specific_heat_debye` | `(t: f64, theta_d: f64) -> f64` |
| `vacancy_concentration` | `(ev: f64, t: f64) -> f64` |

### diffusion (10 functions)

| Function | Signature |
|----------|-----------|
| `fick_first_law` | `(d: f64, dc_dx: f64) -> f64` |
| `fick_second_law_solution` | `(c0: f64, cs: f64, x: f64, d: f64, t: f64) -> f64` |
| `diffusion_coefficient` | `(d0: f64, q: f64, t: f64) -> f64` |
| `diffusion_length` | `(d: f64, t: f64) -> f64` |
| `interdiffusion_coefficient` | `(d_a: f64, d_b: f64, x_a: f64) -> f64` |
| `kirkendall_velocity` | `(d_a: f64, d_b: f64, dc_dx: f64, c_total: f64) -> f64` |
| `grain_boundary_diffusion` | `(d_gb: f64, delta: f64, d_l: f64, grain_size: f64) -> f64` |
| `permeability` | `(d: f64, s: f64) -> f64` |
| `carburization_depth` | `(d: f64, t: f64) -> f64` |
| `mean_free_path` | `(d: f64, n_density: f64) -> f64` |

### phases (12 functions)

| Function | Signature |
|----------|-----------|
| `lever_rule` | `(c0: f64, c_alpha: f64, c_beta: f64) -> (f64, f64)` |
| `gibbs_phase_rule` | `(c: u32, p: u32) -> i32` |
| `clausius_clapeyron_slope` | `(delta_h: f64, t: f64, delta_v: f64) -> f64` |
| `regular_solution_gibbs` | `(xa: f64, omega: f64, t: f64) -> f64` |
| `spinodal_temperature` | `(omega: f64) -> f64` |
| `nucleation_barrier` | `(gamma: f64, delta_gv: f64) -> f64` |
| `critical_nucleus_radius` | `(gamma: f64, delta_gv: f64) -> f64` |
| `nucleation_rate` | `(n0: f64, delta_g_star: f64, t: f64) -> f64` |
| `coarsening_rate` | `(k: f64, t: f64, t0: f64) -> f64` |
| `jmak` | `(k: f64, t: f64, n: f64) -> f64` |
| `partition_coefficient` | `(c_solid: f64, c_liquid: f64) -> f64` |
| `scheil_equation` | `(c0: f64, k: f64, fs: f64) -> f64` |

### semiconductors (11 functions)

| Function | Signature |
|----------|-----------|
| `fermi_energy` | `(n: f64, m_eff: f64) -> f64` |
| `fermi_dirac` | `(e: f64, ef: f64, t: f64) -> f64` |
| `intrinsic_carrier_concentration` | `(eg: f64, t: f64, nc: f64, nv: f64) -> f64` |
| `conductivity_semiconductor` | `(n: f64, mu_e: f64, p: f64, mu_h: f64) -> f64` |
| `hall_coefficient` | `(n: f64, p: f64, mu_e: f64, mu_h: f64) -> f64` |
| `drift_velocity` | `(mu: f64, e_field: f64) -> f64` |
| `depletion_width` | `(epsilon: f64, v_bi: f64, na: f64, nd: f64) -> f64` |
| `built_in_potential` | `(na: f64, nd: f64, ni: f64, t: f64) -> f64` |
| `diode_current` | `(is: f64, v: f64, t: f64, n: f64) -> f64` |
| `band_gap_temperature` | `(eg0: f64, alpha: f64, beta: f64, t: f64) -> f64` |
| `doping_ionization` | `(nd: f64, ed: f64, t: f64) -> f64` |

---

## Nucleosynthesis API (97 functions + structs `Nuclide`, `StellarCore`, enum `ProcessType`)

### Submodules
- `decay` — radioactive decay, half-life, activity
- `fusion` — pp-chain, CNO cycle, plasma physics, stellar winds
- `nuclide` — binding energy, nuclear radius, separation energy
- `processes` — burning stages, r/s-process, nuclear statistical equilibrium
- `reactions` — Q-value, cross-section, Gamow peak, reaction rates
- `stellar` — Chandrasekhar limit, Eddington luminosity, compact objects

### decay (18 functions)

| Function | Signature |
|----------|-----------|
| `decay_remaining` | `(n0: f64, half_life: f64, time: f64) -> f64` |
| `bateman_chain` | `(n0: f64, lambdas: &[f64], time: f64) -> Vec<f64>` |
| `specific_activity` | `(decay_constant: f64, molar_mass: f64) -> f64` |
| `half_life_from_constant` | `(decay_constant: f64) -> f64` |
| `mean_lifetime` | `(half_life: f64) -> f64` |
| `decay_constant_from_half_life` | `(half_life: f64) -> f64` |
| `activity_becquerel` | `(n_atoms: f64, half_life: f64) -> f64` |
| `activity_curie` | `(n_atoms: f64, half_life: f64) -> f64` |
| `secular_equilibrium_activity` | `(parent_activity: f64) -> f64` |
| `transient_equilibrium_ratio` | `(lambda_parent: f64, lambda_daughter: f64) -> f64` |
| `branching_ratio_effective` | `(partial_constants: &[f64]) -> Vec<f64>` |
| `geiger_nuttall` | `(z: u32, energy_mev: f64) -> f64` |
| `alpha_decay_q` | `(parent_mass_mev: f64, daughter_mass_mev: f64, alpha_mass_mev: f64) -> f64` |
| `beta_minus_q` | `(parent_mass_amu: f64, daughter_mass_amu: f64) -> f64` |
| `beta_plus_q` | `(parent_mass_amu: f64, daughter_mass_amu: f64) -> f64` |
| `dose_rate_point_source` | `(activity_bq: f64, gamma_constant: f64, distance_m: f64) -> f64` |
| `radioactive_dating_age` | `(ratio_daughter_parent: f64, half_life: f64) -> f64` |
| `decay_chain_equilibrium_time` | `(lambda_parent: f64, lambda_daughter: f64) -> f64` |

### fusion (26 functions)

| Function | Signature |
|----------|-----------|
| `pp_chain_rate` | `(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64) -> f64` |
| `cno_cycle_rate` | `(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64, cno_fraction: f64) -> f64` |
| `triple_alpha_rate` | `(temperature_k: f64, density_kg_m3: f64, helium_fraction: f64) -> f64` |
| `nuclear_energy_generation` | `(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64, helium_fraction: f64, metal_fraction: f64) -> f64` |
| `thermal_pressure` | `(electron_density: f64, temperature_k: f64) -> f64` |
| `magnetic_pressure` | `(magnetic_field: f64) -> f64` |
| `plasma_beta` | `(electron_density: f64, temperature_k: f64, magnetic_field: f64) -> f64` |
| `alfven_speed` | `(magnetic_field: f64, density: f64) -> f64` |
| `plasma_frequency` | `(electron_density: f64) -> f64` |
| `debye_length` | `(electron_density: f64, temperature_k: f64) -> f64` |
| `radiative_loss_rate` | `(electron_density: f64, temperature_k: f64) -> f64` |
| `thermal_conduction_flux` | `(temperature_k: f64, length_scale: f64) -> f64` |
| `sound_speed_plasma` | `(temperature_k: f64, mean_particle_mass: f64) -> f64` |
| `gyrofrequency` | `(charge: f64, magnetic_field: f64, mass: f64) -> f64` |
| `larmor_radius` | `(mass: f64, velocity_perp: f64, charge: f64, magnetic_field: f64) -> f64` |
| `reconnection_rate_sweet_parker` | `(alfven_speed_val: f64, lundquist_number: f64) -> f64` |
| `lundquist_number` | `(alfven_speed_val: f64, length_scale: f64, magnetic_diffusivity: f64) -> f64` |
| `coronal_loop_temperature` | `(loop_length: f64, heating_rate: f64) -> f64` |
| `coronal_loop_density` | `(heating_rate: f64, loop_length: f64, temperature_k: f64) -> f64` |
| `solar_flare_energy` | `(magnetic_field: f64, volume: f64) -> f64` |
| `cme_kinetic_energy` | `(mass: f64, velocity: f64) -> f64` |
| `sunspot_temperature` | `(photosphere_temp: f64, suppression_factor: f64) -> f64` |
| `differential_rotation_rate` | `(equatorial_rate: f64, latitude: f64, a2: f64, a4: f64) -> f64` |
| `stellar_wind_mass_loss_si` | `(luminosity: f64, escape_velocity: f64) -> f64` |
| `convective_envelope_depth` | `(stellar_mass_solar: f64) -> f64` |
| `mixing_length_velocity` | `(convective_flux: f64, density: f64, temperature: f64, pressure: f64, mixing_length: f64, g_local: f64, cp: f64) -> f64` |
| `opacity_kramers` | `(density: f64, temperature: f64, hydrogen_fraction: f64, metal_fraction: f64) -> f64` |
| `radiative_temperature_gradient` | `(opacity: f64, luminosity: f64, mass_enclosed: f64, pressure: f64, temperature: f64) -> f64` |

### nuclide (struct `Nuclide` + 20 functions)

| Function | Signature |
|----------|-----------|
| `Nuclide::n` | `(&self) -> u32` |
| `Nuclide::atomic_mass_amu` | `(&self) -> f64` |
| `Nuclide::binding_energy` | `(&self) -> f64` |
| `Nuclide::is_stable` | `(&self) -> bool` |
| `Nuclide::decay_constant` | `(&self) -> Option<f64>` |
| `Nuclide::activity_bq` | `(&self, num_atoms: f64) -> Option<f64>` |
| `semi_empirical_mass` | `(z: u32, a: u32) -> f64` |
| `binding_energy_per_nucleon_semf` | `(z: u32, a: u32) -> f64` |
| `nuclear_radius_fm` | `(a: u32) -> f64` |
| `nuclear_density_kg_m3` | `() -> f64` |
| `separation_energy_proton` | `(z: u32, a: u32) -> f64` |
| `separation_energy_neutron` | `(z: u32, a: u32) -> f64` |
| `separation_energy_alpha` | `(z: u32, a: u32) -> f64` |
| `valley_of_stability_z` | `(a: u32) -> f64` |
| `neutron_excess` | `(z: u32, a: u32) -> i32` |
| `isospin_z` | `(z: u32, a: u32) -> f64` |
| `liquid_drop_fission_parameter` | `(z: u32, a: u32) -> f64` |
| `fission_barrier_estimate_mev` | `(z: u32, a: u32) -> f64` |
| `nuclear_skin_thickness_fm` | `(z: u32, a: u32) -> f64` |
| `weizsacker_mass_excess_mev` | `(z: u32, a: u32) -> f64` |
| `proton_drip_line_a` | `(z: u32) -> u32` |
| `neutron_drip_line_a` | `(z: u32) -> u32` |
| `magic_number_nearest` | `(n: u32) -> u32` |
| `is_doubly_magic` | `(z: u32, n: u32) -> bool` |

### processes (enum `ProcessType` + 14 functions)

| Function | Signature |
|----------|-----------|
| `ProcessType::threshold_temperature_k` | `(&self) -> f64` |
| `ProcessType::energy_released_mev` | `(&self) -> f64` |
| `ProcessType::is_active_at` | `(&self, temperature_k: f64) -> bool` |
| `active_processes` | `(temperature_k: f64) -> Vec<ProcessType>` |
| `dominant_process_at` | `(temperature_k: f64) -> Option<ProcessType>` |
| `process_timescale_years` | `(process: &ProcessType, mass_solar: f64) -> f64` |
| `process_fuel` | `(process: &ProcessType) -> &'static str` |
| `process_product` | `(process: &ProcessType) -> &'static str` |
| `nuclear_statistical_equilibrium_temp` | `() -> f64` |
| `neutron_capture_rate_estimate` | `(neutron_density_per_cm3: f64, cross_section_barn: f64, velocity_cm_s: f64) -> f64` |
| `s_process_neutron_exposure` | `(tau: f64, sigma_times_flux: f64) -> f64` |
| `r_process_waiting_point_z` | `(neutron_separation_energy_mev: f64, temperature_gk: f64) -> f64` |
| `pp_chain_branches` | `(temperature_k: f64) -> (f64, f64, f64)` |
| `cno_cycle_is_dominant` | `(temperature_k: f64, metallicity: f64) -> bool` |

### reactions (21 functions)

| Function | Signature |
|----------|-----------|
| `q_value` | `(reactants: &[&Nuclide], products: &[&Nuclide]) -> f64` |
| `coulomb_barrier` | `(z1: u32, z2: u32, a1: u32, a2: u32) -> f64` |
| `gamow_peak` | `(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) -> f64` |
| `gamow_window_width` | `(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) -> f64` |
| `reduced_mass_amu` | `(m1: f64, m2: f64) -> f64` |
| `astrophysical_s_factor` | `(cross_section_barn: f64, energy_kev: f64, z1: u32, z2: u32, mu_amu: f64) -> f64` |
| `sommerfeld_parameter` | `(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) -> f64` |
| `penetration_factor` | `(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) -> f64` |
| `thermonuclear_rate` | `(s_factor_kev_barn: f64, z1: u32, z2: u32, mu_amu: f64, temperature_k: f64) -> f64` |
| `pp_rate_estimate` | `(temperature_k: f64, density_g_cm3: f64, x_h: f64) -> f64` |
| `triple_alpha_rate_estimate` | `(temperature_k: f64, density_g_cm3: f64, y_he: f64) -> f64` |
| `c12_alpha_rate_estimate` | `(temperature_k: f64) -> f64` |
| `reaction_mean_free_path` | `(cross_section_barn: f64, number_density_per_cm3: f64) -> f64` |
| `nuclear_reaction_lifetime` | `(cross_section_barn: f64, number_density_per_cm3: f64, velocity_cm_s: f64) -> f64` |
| `maxwell_averaged_velocity` | `(temperature_k: f64, mu_amu: f64) -> f64` |
| `cross_section_barn_to_si` | `(sigma_barn: f64) -> f64` |
| `nuclear_radius_fermi` | `(a: u32) -> f64` |
| `nuclear_volume` | `(a: u32) -> f64` |
| `q_value_to_joules` | `(q_mev: f64) -> f64` |
| `geometric_cross_section` | `(a1: u32, a2: u32) -> f64` |
| `geometric_cross_section_barn` | `(a1: u32, a2: u32) -> f64` |

### stellar (struct `StellarCore` + 18 functions)

| Function | Signature |
|----------|-----------|
| `StellarCore::new` | `(mass_solar: f64, temperature_k: f64, density_kg_m3: f64) -> Self` |
| `StellarCore::luminosity_solar` | `(&self) -> f64` |
| `StellarCore::main_sequence_lifetime_years` | `(&self) -> f64` |
| `StellarCore::active_processes` | `(&self) -> Vec<ProcessType>` |
| `StellarCore::dominant_process` | `(&self) -> Option<ProcessType>` |
| `StellarCore::evolve_step` | `(&mut self, dt_years: f64)` |
| `chandrasekhar_limit` | `() -> f64` |
| `tolman_oppenheimer_volkoff_limit` | `() -> f64` |
| `neutron_drip_density` | `() -> f64` |
| `iron_peak_binding_energy` | `() -> f64` |
| `eddington_luminosity_solar` | `(mass_solar: f64) -> f64` |
| `kelvin_helmholtz_timescale_years` | `(mass_solar: f64, radius_solar: f64, luminosity_solar: f64) -> f64` |
| `jeans_mass_solar` | `(temperature_k: f64, density_kg_m3: f64) -> f64` |
| `schwarzschild_radius_km` | `(mass_solar: f64) -> f64` |
| `nuclear_timescale_years` | `(mass_solar: f64, luminosity_solar: f64, efficiency: f64) -> f64` |
| `core_collapse_min_mass_solar` | `() -> f64` |
| `white_dwarf_radius_km` | `(mass_solar: f64) -> f64` |
| `electron_degeneracy_pressure` | `(density_kg_m3: f64) -> f64` |
| `neutron_star_radius_km` | `(mass_solar: f64) -> f64` |
| `luminosity_radius_temperature` | `(radius_solar: f64, temperature_k: f64) -> f64` |
| `stellar_wind_mass_loss` | `(luminosity_solar: f64, escape_velocity_km_s: f64) -> f64` |

---

## Optics API (60 functions)

### Submodules
- `diffraction` — single/double slit, grating equation
- `interference` — thin film, Michelson, Young's experiment
- `polarization` — Malus's law, Brewster angle, Stokes parameters
- `refraction` — Snell's law, critical angle, refractive index dispersion
- `scattering` — Rayleigh, Mie, atmospheric optics

### diffraction (10 functions)

| Function | Signature |
|----------|-----------|
| `single_slit_intensity` | `(theta: f64, a: f64, wavelength: f64) -> f64` |
| `double_slit_intensity` | `(theta: f64, d: f64, wavelength: f64) -> f64` |
| `diffraction_grating_maxima` | `(d: f64, wavelength: f64, order: i32) -> f64` |
| `resolving_power_grating` | `(order: i32, n_slits: u32) -> f64` |
| `rayleigh_criterion` | `(wavelength: f64, aperture: f64) -> f64` |
| `airy_disk_radius` | `(wavelength: f64, f_number: f64) -> f64` |
| `fraunhofer_distance` | `(aperture: f64, wavelength: f64) -> f64` |
| `grating_dispersion` | `(order: i32, d: f64, theta: f64) -> f64` |
| `bragg_condition` | `(d_spacing: f64, theta: f64, wavelength: f64) -> f64` |
| `circular_aperture_first_zero` | `(wavelength: f64, diameter: f64) -> f64` |

### interference (12 functions)

| Function | Signature |
|----------|-----------|
| `two_beam_intensity` | `(i1: f64, i2: f64, delta_phase: f64) -> f64` |
| `thin_film_phase_shift` | `(n: f64, thickness: f64, wavelength: f64, theta: f64) -> f64` |
| `constructive_condition` | `(optical_path_diff: f64, wavelength: f64) -> bool` |
| `fringe_spacing` | `(wavelength: f64, d: f64, l: f64) -> f64` |
| `visibility` | `(i_max: f64, i_min: f64) -> f64` |
| `coherence_length` | `(wavelength: f64, delta_wavelength: f64) -> f64` |
| `coherence_time` | `(delta_nu: f64) -> f64` |
| `fabry_perot_transmittance` | `(r: f64, delta: f64) -> f64` |
| `fabry_perot_finesse` | `(r: f64) -> f64` |
| `free_spectral_range` | `(d: f64, n: f64) -> f64` |
| `michelson_path_difference` | `(mirror_displacement: f64) -> f64` |
| `newton_ring_radius` | `(m: u32, wavelength: f64, r: f64) -> f64` |

### polarization (11 functions)

| Function | Signature |
|----------|-----------|
| `malus_law` | `(i0: f64, theta: f64) -> f64` |
| `brewster_reflectance` | `(n1: f64, n2: f64) -> f64` |
| `stokes_parameters` | `(ex: f64, ey: f64, delta: f64) -> [f64; 4]` |
| `degree_of_polarization` | `(s: &[f64; 4]) -> f64` |
| `jones_rotation` | `(theta: f64) -> [[f64; 2]; 2]` |
| `quarter_wave_plate_phase` | `(wavelength: f64, n_fast: f64, n_slow: f64) -> f64` |
| `specific_rotation` | `(observed: f64, l: f64, c: f64) -> f64` |
| `ellipticity` | `(major: f64, minor: f64) -> f64` |
| `circular_dichroism` | `(a_left: f64, a_right: f64) -> f64` |
| `birefringence` | `(n_extraordinary: f64, n_ordinary: f64) -> f64` |
| `retardance` | `(birefringence: f64, thickness: f64, wavelength: f64) -> f64` |

### refraction (12 functions)

| Function | Signature |
|----------|-----------|
| `snell` | `(n1: f64, theta1: f64, n2: f64) -> f64` |
| `critical_angle` | `(n1: f64, n2: f64) -> f64` |
| `brewster_angle` | `(n1: f64, n2: f64) -> f64` |
| `fresnel_reflectance_s` | `(n1: f64, theta_i: f64, n2: f64, theta_t: f64) -> f64` |
| `fresnel_reflectance_p` | `(n1: f64, theta_i: f64, n2: f64, theta_t: f64) -> f64` |
| `thin_lens_equation` | `(focal: f64, object_dist: f64) -> f64` |
| `magnification` | `(image_dist: f64, object_dist: f64) -> f64` |
| `lensmaker_equation` | `(n: f64, r1: f64, r2: f64) -> f64` |
| `numerical_aperture` | `(n: f64, half_angle: f64) -> f64` |
| `optical_path_length` | `(n: f64, d: f64) -> f64` |
| `cauchy_dispersion` | `(a: f64, b: f64, wavelength: f64) -> f64` |
| `abbe_number` | `(nd: f64, nf: f64, nc: f64) -> f64` |

### scattering (22 functions)

| Function | Signature |
|----------|-----------|
| `rayleigh_cross_section` | `(wavelength: f64, refractive_index: f64, depolarization: f64) -> f64` |
| `rayleigh_scattering_coefficient` | `(number_density: f64, wavelength: f64, refractive_index: f64, depolarization: f64) -> f64` |
| `rayleigh_phase_function` | `(cos_theta: f64) -> f64` |
| `mie_extinction_efficiency` | `(size_parameter: f64, refractive_index_real: f64) -> f64` |
| `mie_scattering_coefficient` | `(number_density: f64, particle_radius: f64, wavelength: f64, refractive_index_real: f64) -> f64` |
| `henyey_greenstein` | `(cos_theta: f64, g: f64) -> f64` |
| `optical_depth_integral` | `(scattering_coefficient: f64, scale_height: f64, altitude_start: f64, altitude_end: f64) -> f64` |
| `transmittance` | `(optical_depth: f64) -> f64` |
| `single_scattering_albedo` | `(scattering_coeff: f64, absorption_coeff: f64) -> f64` |
| `atmospheric_refraction` | `(zenith_angle: f64, pressure_pa: f64, temperature_k: f64) -> f64` |
| `color_temperature_to_rgb` | `(temperature_k: f64) -> (f64, f64, f64)` |
| `planck_spectral_radiance` | `(wavelength: f64, temperature: f64) -> f64` |
| `rayleigh_sky_color` | `(wavelength_r: f64, wavelength_g: f64, wavelength_b: f64, optical_depth_zenith: f64, cos_zenith: f64) -> (f64, f64, f64)` |
| `limb_darkening` | `(cos_angle: f64, coefficient: f64) -> f64` |
| `absorption_coefficient_gas` | `(cross_section: f64, number_density: f64) -> f64` |
| `chapman_function` | `(zenith_angle: f64, scale_height_ratio: f64) -> f64` |
| `glory_angle` | `(particle_radius: f64, wavelength: f64) -> f64` |
| `rainbow_angle` | `(refractive_index: f64) -> f64` |
| `wavelength_to_energy_ev` | `(wavelength_nm: f64) -> f64` |
| `energy_ev_to_wavelength_nm` | `(energy_ev: f64) -> f64` |
| `wavelength_angstrom_to_m` | `(wavelength_angstrom: f64) -> f64` |
| `wavelength_m_to_angstrom` | `(wavelength_m: f64) -> f64` |
| `photon_energy_joule_to_ev` | `(energy_j: f64) -> f64` |
| `size_parameter` | `(radius: f64, wavelength: f64) -> f64` |

---

## Quantum API (struct `Complex` + 200+ functions)

### Submodules
- `angular` — angular momentum, Clebsch-Gordan, ladder operators
- `information` — density matrix, entanglement entropy, fidelity, quantum gates
- `operators` — expectation values, commutators, Hermitian operators
- `perturbation` — first/second-order energy corrections, variational, WKB
- `spin` — spin operators, Pauli matrices, Dirac gamma matrices
- `systems` — harmonic oscillator, hydrogen atom, particle in a box, Landau levels
- `wavefunctions` — normalization, probability density, time evolution

### angular (6 functions)

| Function | Signature |
|----------|-----------|
| `associated_legendre` | `(l: u32, m: i32, x: f64) -> f64` |
| `spherical_harmonic` | `(l: u32, m: i32, theta: f64, phi: f64) -> Complex` |
| `clebsch_gordan` | `(j1: f64, m1: f64, j2: f64, m2: f64, j: f64, m: f64) -> f64` |
| `wigner_3j` | `(j1: f64, j2: f64, j3: f64, m1: f64, m2: f64, m3: f64) -> f64` |
| `spherical_harmonic_real` | `(l: u32, m: i32, theta: f64, phi: f64) -> f64` |
| `angular_momentum_coupling` | `(j1: f64, j2: f64) -> Vec<(f64, f64, f64)>` |

### information (27 functions)

| Function | Signature |
|----------|-----------|
| `von_neumann_entropy` | `(rho: &[Vec<Complex>]) -> f64` |
| `purity` | `(rho: &[Vec<Complex>]) -> f64` |
| `fidelity_pure` | `(psi: &[Complex], phi: &[Complex]) -> f64` |
| `fidelity_mixed` | `(rho: &[Vec<Complex>], sigma: &[Vec<Complex>]) -> f64` |
| `concurrence_2qubit` | `(rho: &[Vec<Complex>]) -> f64` |
| `bell_phi_plus` | `() -> Vec<Complex>` |
| `bell_phi_minus` | `() -> Vec<Complex>` |
| `bell_psi_plus` | `() -> Vec<Complex>` |
| `bell_psi_minus` | `() -> Vec<Complex>` |
| `hadamard_gate` | `() -> Vec<Vec<Complex>>` |
| `phase_gate` | `(phi: f64) -> Vec<Vec<Complex>>` |
| `t_gate` | `() -> Vec<Vec<Complex>>` |
| `s_gate` | `() -> Vec<Vec<Complex>>` |
| `rx_gate` | `(theta: f64) -> Vec<Vec<Complex>>` |
| `ry_gate` | `(theta: f64) -> Vec<Vec<Complex>>` |
| `rz_gate` | `(theta: f64) -> Vec<Vec<Complex>>` |
| `cnot_gate` | `() -> Vec<Vec<Complex>>` |
| `swap_gate` | `() -> Vec<Vec<Complex>>` |
| `cz_gate` | `() -> Vec<Vec<Complex>>` |
| `toffoli_gate` | `() -> Vec<Vec<Complex>>` |
| `apply_gate` | `(gate: &[Vec<Complex>], state: &[Complex]) -> Vec<Complex>` |
| `apply_single_qubit_gate` | `(gate: &[Vec<Complex>], state: &mut [Complex], target: usize, n_qubits: usize)` |
| `measure_probabilities` | `(state: &[Complex]) -> Vec<f64>` |
| `bloch_vector` | `(state: &[Complex; 2]) -> (f64, f64, f64)` |
| `entanglement_entropy` | `(state: &[Complex], dim_a: usize) -> f64` |

### operators (11 functions)

| Function | Signature |
|----------|-----------|
| `commutator` | `(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>>` |
| `anticommutator` | `(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>>` |
| `expectation_value` | `(operator: &[Vec<Complex>], state: &[Complex]) -> Complex` |
| `variance` | `(operator: &[Vec<Complex>], state: &[Complex]) -> f64` |
| `uncertainty_product` | `(op_a: &[Vec<Complex>], op_b: &[Vec<Complex>], state: &[Complex]) -> f64` |
| `is_hermitian` | `(m: &[Vec<Complex>]) -> bool` |
| `is_unitary` | `(m: &[Vec<Complex>]) -> bool` |
| `tensor_product` | `(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>>` |
| `density_matrix` | `(state: &[Complex]) -> Vec<Vec<Complex>>` |
| `trace_complex` | `(m: &[Vec<Complex>]) -> Complex` |
| `partial_trace_b` | `(rho: &[Vec<Complex>], dim_a: usize, dim_b: usize) -> Vec<Vec<Complex>>` |

### perturbation (9 functions)

| Function | Signature |
|----------|-----------|
| `first_order_energy` | `(v_matrix: &[Vec<Complex>], state: &[Complex]) -> f64` |
| `second_order_energy` | `(h0_energies: &[f64], v_matrix: &[Vec<Complex>], states: &[Vec<Complex>], n_index: usize) -> f64` |
| `first_order_state` | `(h0_energies: &[f64], v_matrix: &[Vec<Complex>], states: &[Vec<Complex>], n_index: usize) -> Vec<Complex>` |
| `variational_energy` | `(hamiltonian: &[Vec<Complex>], trial: &[Complex]) -> f64` |
| `variational_minimize` | `(hamiltonian: &[Vec<Complex>], trial_fn: impl Fn(f64) -> Vec<Complex>, param_min: f64, param_max: f64, n_points: usize) -> (f64, f64)` |
| `wkb_phase` | `(v: impl Fn(f64) -> f64, energy: f64, x1: f64, x2: f64, mass: f64) -> f64` |
| `wkb_transmission` | `(v: impl Fn(f64) -> f64, energy: f64, x1: f64, x2: f64, mass: f64) -> f64` |
| `wkb_quantization` | `(v: impl Fn(f64) -> f64, mass: f64, x_min: f64, x_max: f64, n_quantum: u32) -> f64` |
| `fermi_golden_rule` | `(v_matrix: &[Vec<Complex>], initial: &[Complex], finals: &[Vec<Complex>], density_of_states: f64) -> f64` |

### spin (19 functions)

| Function | Signature |
|----------|-----------|
| `pauli_x` | `() -> [[Complex; 2]; 2]` |
| `pauli_y` | `() -> [[Complex; 2]; 2]` |
| `pauli_z` | `() -> [[Complex; 2]; 2]` |
| `identity_2` | `() -> [[Complex; 2]; 2]` |
| `spin_up` | `() -> [Complex; 2]` |
| `spin_down` | `() -> [Complex; 2]` |
| `spin_plus_x` | `() -> [Complex; 2]` |
| `spin_minus_x` | `() -> [Complex; 2]` |
| `spin_plus_y` | `() -> [Complex; 2]` |
| `spin_minus_y` | `() -> [Complex; 2]` |
| `dirac_gamma0` | `() -> [[Complex; 4]; 4]` |
| `dirac_gamma1` | `() -> [[Complex; 4]; 4]` |
| `dirac_gamma2` | `() -> [[Complex; 4]; 4]` |
| `dirac_gamma3` | `() -> [[Complex; 4]; 4]` |
| `gamma5` | `() -> [[Complex; 4]; 4]` |
| `rotation_operator` | `(angle: f64, pauli: &[[Complex; 2]; 2]) -> [[Complex; 2]; 2]` |
| `ladder_operator_plus` | `(j: f64) -> Vec<Vec<Complex>>` |
| `ladder_operator_minus` | `(j: f64) -> Vec<Vec<Complex>>` |
| `jz_operator` | `(j: f64) -> Vec<Vec<Complex>>` |

### systems (35 functions)

| Function | Signature |
|----------|-----------|
| `hydrogen_energy` | `(n: u32) -> f64` |
| `hydrogen_radial_r10` | `(r: f64) -> f64` |
| `hydrogen_radial_r20` | `(r: f64) -> f64` |
| `hydrogen_radial_r21` | `(r: f64) -> f64` |
| `harmonic_oscillator_energy` | `(n: u32, omega: f64) -> f64` |
| `harmonic_oscillator_wf` | `(n: u32, x: f64, mass: f64, omega: f64) -> f64` |
| `infinite_well_energy` | `(n: u32, length: f64, mass: f64) -> f64` |
| `infinite_well_wf` | `(n: u32, x: f64, length: f64) -> f64` |
| `tunneling_coefficient` | `(energy: f64, v0: f64, width: f64, mass: f64) -> f64` |
| `bohr_radius` | `() -> f64` |
| `landau_levels` | `(n: u32, b_field: f64, mass: f64, charge: f64) -> f64` |
| `zeeman_splitting` | `(m_l: i32, b_field: f64) -> f64` |
| `bohr_radius_nth` | `(n: u32, z_eff: f64) -> f64` |
| `hydrogen_energy_level` | `(n: u32) -> f64` |
| `hydrogen_energy_level_z` | `(n: u32, z: f64) -> f64` |
| `rydberg_wavelength` | `(n1: u32, n2: u32) -> f64` |
| `compton_wavelength_shift` | `(theta: f64) -> f64` |
| `cyclotron_frequency` | `(b_field: f64) -> f64` |
| `cyclotron_frequency_particle` | `(charge: f64, mass: f64, b_field: f64) -> f64` |
| `larmor_radius` | `(v_perp: f64, b_field: f64) -> f64` |
| `larmor_radius_particle` | `(mass: f64, v_perp: f64, charge: f64, b_field: f64) -> f64` |
| `nuclear_zeeman_splitting` | `(m_i: f64, b_field: f64) -> f64` |
| `anomalous_zeeman_splitting` | `(m_j: f64, g_j: f64, b_field: f64) -> f64` |
| `muonic_hydrogen_energy` | `(n: u32) -> f64` |
| `muonic_bohr_radius` | `() -> f64` |
| `tau_lepton_mass` | `() -> f64` |
| `muon_mass_kg` | `() -> f64` |
| `proton_gyromagnetic_ratio` | `(b_field: f64) -> f64` |
| `neutron_mass` | `() -> f64` |
| `proton_mass` | `() -> f64` |
| `reduced_mass` | `(m1: f64, m2: f64) -> f64` |
| `de_broglie_wavelength` | `(mass: f64, velocity: f64) -> f64` |
| `magnetic_moment_orbital` | `(m_l: i32) -> f64` |
| `fine_structure_splitting` | `(n: u32, j: f64, z: f64) -> f64` |
| `hyperfine_splitting_hydrogen` | `() -> f64` |

### wavefunctions (9 functions)

| Function | Signature |
|----------|-----------|
| `plane_wave` | `(x: f64, k: f64, omega: f64, t: f64) -> Complex` |
| `gaussian_packet` | `(x: f64, x0: f64, sigma: f64, k0: f64) -> Complex` |
| `normalize` | `(psi: &mut [Complex], dx: f64)` |
| `probability_density` | `(psi: &[Complex]) -> Vec<f64>` |
| `expectation_position` | `(psi: &[Complex], x: &[f64], dx: f64) -> f64` |
| `expectation_momentum` | `(psi: &[Complex], dx: f64) -> f64` |
| `time_evolve_split_step` | `(psi: &mut [Complex], v: &[f64], dx: f64, dt: f64, mass: f64, steps: usize)` |
| `inner_product` | `(psi: &[Complex], phi: &[Complex], dx: f64) -> Complex` |
| `transition_probability` | `(psi_initial: &[Complex], psi_final: &[Complex], dx: f64) -> f64` |

---

## Relativity API (45 functions)

### Submodules
- `accretion` — black hole physics, accretion disks, jets
- `dynamics` — relativistic energy, momentum, four-vectors
- `kinematics` — time dilation, length contraction, velocity addition
- `lorentz` — Lorentz factor, boosts, transformations

### accretion (24 functions)

| Function | Signature |
|----------|-----------|
| `schwarzschild_radius` | `(mass: f64) -> f64` |
| `kerr_event_horizon` | `(mass: f64, spin_parameter: f64) -> f64` |
| `kerr_ergosphere` | `(mass: f64, spin_parameter: f64, theta: f64) -> f64` |
| `isco_radius` | `(mass: f64, spin_parameter: f64) -> f64` |
| `accretion_disk_temperature` | `(mass: f64, accretion_rate: f64, r: f64, r_inner: f64) -> f64` |
| `accretion_disk_luminosity` | `(mass: f64, accretion_rate: f64, r_inner: f64) -> f64` |
| `radiative_efficiency` | `(r_isco: f64, mass: f64) -> f64` |
| `eddington_accretion_rate` | `(mass: f64) -> f64` |
| `disk_surface_density` | `(accretion_rate: f64, viscosity: f64, r: f64, r_inner: f64) -> f64` |
| `alpha_viscosity` | `(alpha: f64, sound_speed: f64, scale_height: f64) -> f64` |
| `disk_scale_height` | `(sound_speed: f64, orbital_frequency: f64) -> f64` |
| `orbital_frequency_kerr` | `(mass: f64, r: f64, spin_parameter: f64) -> f64` |
| `gravitational_redshift` | `(mass: f64, r: f64) -> f64` |
| `doppler_beaming_factor` | `(velocity_los: f64, bulk_lorentz: f64) -> f64` |
| `apparent_superluminal_velocity` | `(beta: f64, viewing_angle: f64) -> f64` |
| `blandford_znajek_power` | `(magnetic_field: f64, event_horizon_radius: f64, spin_parameter: f64, mass: f64) -> f64` |
| `jet_lorentz_factor_from_ratio` | `(jet_power: f64, mass_loading_rate: f64) -> f64` |
| `synchrotron_cooling_time` | `(electron_gamma: f64, magnetic_field: f64) -> f64` |
| `photon_ring_radius` | `(mass: f64) -> f64` |
| `shadow_angular_radius` | `(mass: f64, distance: f64) -> f64` |
| `advection_dominated_temperature` | `(proton_mass: f64, r: f64, mass: f64) -> f64` |
| `comptonization_parameter` | `(electron_temperature: f64, optical_depth: f64) -> f64` |
| `bondi_accretion_rate` | `(mass: f64, ambient_density: f64, sound_speed: f64) -> f64` |
| `tidal_disruption_radius` | `(bh_mass: f64, star_mass: f64, star_radius: f64) -> f64` |

### dynamics (13 functions)

| Function | Signature |
|----------|-----------|
| `relativistic_momentum` | `(mass: f64, v: f64) -> f64` |
| `relativistic_energy` | `(mass: f64, v: f64) -> f64` |
| `rest_energy` | `(mass: f64) -> f64` |
| `kinetic_energy_relativistic` | `(mass: f64, v: f64) -> f64` |
| `energy_momentum_relation` | `(mass: f64, momentum: f64) -> f64` |
| `four_momentum` | `(mass: f64, v: [f64; 3]) -> [f64; 4]` |
| `invariant_mass_two_body` | `(p1: [f64; 4], p2: [f64; 4]) -> f64` |
| `mandelstam_s` | `(p1: [f64; 4], p2: [f64; 4]) -> f64` |
| `compton_wavelength_shift` | `(angle: f64) -> f64` |
| `relativistic_kinetic_energy_from_gamma` | `(mass: f64, gamma: f64) -> f64` |
| `threshold_energy` | `(m_target: f64, m_products_sum: f64) -> f64` |
| `synchrotron_power` | `(charge: f64, mass: f64, gamma: f64, radius: f64) -> f64` |
| `bremsstrahlung_power_classical` | `(charge: f64, accel: f64) -> f64` |

### kinematics (8 functions)

| Function | Signature |
|----------|-----------|
| `velocity_addition` | `(u: f64, v: f64) -> f64` |
| `velocity_addition_3d` | `(u: [f64; 3], v: f64, dir: [f64; 3]) -> [f64; 3]` |
| `relativistic_doppler` | `(freq: f64, v: f64, angle: f64) -> f64` |
| `transverse_doppler` | `(freq: f64, v: f64) -> f64` |
| `aberration` | `(theta: f64, v: f64) -> f64` |
| `headlight_effect` | `(theta_rest: f64, v: f64) -> f64` |
| `proper_acceleration_to_coordinate` | `(proper_accel: f64, proper_time: f64) -> (f64, f64)` |
| `twin_paradox_age` | `(v: f64, coord_time: f64) -> f64` |

### lorentz (10 functions)

| Function | Signature |
|----------|-----------|
| `gamma_factor` | `(v: f64) -> f64` |
| `beta` | `(v: f64) -> f64` |
| `time_dilation` | `(proper_time: f64, v: f64) -> f64` |
| `length_contraction` | `(proper_length: f64, v: f64) -> f64` |
| `lorentz_transform` | `(t: f64, x: f64, v: f64) -> (f64, f64)` |
| `inverse_lorentz_transform` | `(t_prime: f64, x_prime: f64, v: f64) -> (f64, f64)` |
| `lorentz_transform_4` | `(event: [f64; 4], v: [f64; 3]) -> [f64; 4]` |
| `boost_matrix` | `(v: [f64; 3]) -> [[f64; 4]; 4]` |
| `rapidity` | `(v: f64) -> f64` |
| `velocity_from_rapidity` | `(phi: f64) -> f64` |

---

## Solid Mechanics API (40 functions)

### Submodules
- `elasticity` — Hooke's law, Young's modulus, Poisson ratio
- `fracture` — stress intensity factor, fracture toughness, crack growth
- `plasticity` — yield criteria, strain hardening, plastic deformation
- `stress` — stress/strain tensors, principal stresses, von Mises

### elasticity (12 functions)

| Function | Signature |
|----------|-----------|
| `hooke_stress` | `(e: f64, strain: f64) -> f64` |
| `hooke_strain` | `(stress: f64, e: f64) -> f64` |
| `poisson_lateral_strain` | `(axial_strain: f64, nu: f64) -> f64` |
| `shear_modulus` | `(e: f64, nu: f64) -> f64` |
| `bulk_modulus` | `(e: f64, nu: f64) -> f64` |
| `lame_first` | `(e: f64, nu: f64) -> f64` |
| `plane_stress_strain` | `(stress_x: f64, stress_y: f64, e: f64, nu: f64) -> (f64, f64)` |
| `strain_energy_density` | `(stress: f64, strain: f64) -> f64` |
| `thermal_strain` | `(alpha: f64, delta_t: f64) -> f64` |
| `thermal_stress` | `(e: f64, alpha: f64, delta_t: f64) -> f64` |
| `volumetric_strain` | `(ex: f64, ey: f64, ez: f64) -> f64` |
| `hydrostatic_stress` | `(sx: f64, sy: f64, sz: f64) -> f64` |

### fracture (12 functions)

| Function | Signature |
|----------|-----------|
| `stress_intensity_factor` | `(sigma: f64, a: f64, geometry_factor: f64) -> f64` |
| `griffith_critical_stress` | `(e: f64, gamma: f64, a: f64) -> f64` |
| `energy_release_rate` | `(k: f64, e: f64) -> f64` |
| `j_integral` | `(energy_release: f64) -> f64` |
| `crack_tip_plastic_zone` | `(k: f64, sigma_y: f64) -> f64` |
| `paris_law` | `(c: f64, delta_k: f64, m: f64) -> f64` |
| `fatigue_life_basquin` | `(sigma_f: f64, sigma_a: f64, b: f64) -> f64` |
| `fatigue_life_coffin_manson` | `(ef: f64, ea: f64, c: f64) -> f64` |
| `miners_rule` | `(cycles: &[f64], max_cycles: &[f64]) -> f64` |
| `fracture_toughness_plane_strain` | `(kic: f64, sigma_y: f64) -> f64` |
| `stress_corrosion_threshold` | `(k_iscc: f64, sigma: f64, a: f64) -> bool` |
| `crack_opening_displacement` | `(k: f64, sigma_y: f64, e: f64) -> f64` |

### plasticity (12 functions)

| Function | Signature |
|----------|-----------|
| `yield_offset_strain` | `(total_strain: f64, offset: f64) -> f64` |
| `ramberg_osgood` | `(stress: f64, e: f64, k: f64, n: f64) -> f64` |
| `true_stress` | `(engineering_stress: f64, engineering_strain: f64) -> f64` |
| `true_strain` | `(engineering_strain: f64) -> f64` |
| `hardening_power_law` | `(k: f64, strain_plastic: f64, n: f64) -> f64` |
| `von_mises` | `(s1: f64, s2: f64, s3: f64) -> f64` |
| `tresca` | `(s1: f64, s2: f64, s3: f64) -> f64` |
| `isotropic_hardening` | `(yield_0: f64, h: f64, plastic_strain: f64) -> f64` |
| `bauschinger_effect` | `(forward_yield: f64, reverse_yield: f64) -> f64` |
| `plastic_work` | `(stress: &[f64], d_plastic_strain: &[f64]) -> f64` |
| `necking_criterion` | `(n: f64) -> f64` |
| `strain_rate_sensitivity` | `(stress1: f64, stress2: f64, rate1: f64, rate2: f64) -> f64` |

### stress (12 functions)

| Function | Signature |
|----------|-----------|
| `principal_stresses_2d` | `(sx: f64, sy: f64, txy: f64) -> (f64, f64)` |
| `max_shear_stress_2d` | `(sx: f64, sy: f64, txy: f64) -> f64` |
| `mohr_circle_radius` | `(sx: f64, sy: f64, txy: f64) -> f64` |
| `mohr_circle_center` | `(sx: f64, sy: f64) -> f64` |
| `stress_rotation_2d` | `(sx: f64, sy: f64, txy: f64, theta: f64) -> (f64, f64, f64)` |
| `deviatoric_stress` | `(sx: f64, sy: f64, sz: f64) -> (f64, f64, f64)` |
| `stress_invariant_j2` | `(s1: f64, s2: f64, s3: f64) -> f64` |
| `beam_bending_stress` | `(m: f64, y: f64, i: f64) -> f64` |
| `beam_deflection_cantilever` | `(p: f64, l: f64, e: f64, i: f64) -> f64` |
| `torsion_shear_stress` | `(t: f64, r: f64, j: f64) -> f64` |
| `column_euler_buckling` | `(e: f64, i: f64, l: f64) -> f64` |
| `hertz_contact_pressure` | `(force: f64, r1: f64, r2: f64, e_star: f64) -> f64` |

---

## Thermodynamics API (55 functions)

### Submodules
- `equations` — equations of state (ideal gas, van der Waals, Redlich-Kwong)
- `processes` — isothermal, adiabatic, isobaric, isochoric work/heat
- `statistical` — Boltzmann factor, partition function, entropy

### equations (29 functions)

| Function | Signature |
|----------|-----------|
| `ideal_gas_pressure` | `(n_moles: f64, temperature: f64, volume: f64) -> f64` |
| `ideal_gas_volume` | `(n_moles: f64, temperature: f64, pressure: f64) -> f64` |
| `ideal_gas_temperature` | `(pressure: f64, volume: f64, n_moles: f64) -> f64` |
| `van_der_waals_pressure` | `(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) -> f64` |
| `redlich_kwong_pressure` | `(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) -> f64` |
| `virial_eos` | `(pressure: f64, temperature: f64, b2: f64, b3: f64) -> f64` |
| `compressibility_factor` | `(pressure: f64, volume: f64, n_moles: f64, temperature: f64) -> f64` |
| `internal_energy_ideal` | `(n_moles: f64, cv: f64, temperature: f64) -> f64` |
| `enthalpy_ideal` | `(n_moles: f64, cp: f64, temperature: f64) -> f64` |
| `entropy_ideal_gas` | `(n_moles: f64, cv: f64, t1: f64, t2: f64, v1: f64, v2: f64) -> f64` |
| `gibbs_free_energy` | `(enthalpy: f64, temperature: f64, entropy: f64) -> f64` |
| `helmholtz_free_energy` | `(internal_energy: f64, temperature: f64, entropy: f64) -> f64` |
| `chemical_potential_ideal_gas` | `(mu0: f64, temperature: f64, pressure: f64, p0: f64) -> f64` |
| `clausius_clapeyron` | `(p1: f64, t1: f64, t2: f64, delta_h_vap: f64) -> f64` |
| `heat_capacity_ratio` | `(cp: f64, cv: f64) -> f64` |
| `speed_of_sound_ideal` | `(gamma: f64, temperature: f64, molar_mass: f64) -> f64` |
| `maxwell_speed_distribution` | `(v: f64, mass: f64, temperature: f64) -> f64` |
| `most_probable_speed` | `(mass: f64, temperature: f64) -> f64` |
| `mean_speed` | `(mass: f64, temperature: f64) -> f64` |
| `rms_speed` | `(mass: f64, temperature: f64) -> f64` |
| `mean_free_path` | `(number_density: f64, cross_section: f64) -> f64` |
| `pressure_atm_to_pascal` | `(p_atm: f64) -> f64` |
| `pressure_pascal_to_atm` | `(p_pa: f64) -> f64` |
| `pressure_bar_to_pascal` | `(p_bar: f64) -> f64` |
| `energy_calories_to_joules` | `(cal: f64) -> f64` |
| `energy_joules_to_calories` | `(j: f64) -> f64` |
| `energy_kwh_to_joules` | `(kwh: f64) -> f64` |
| `plasma_temperature_kev_to_kelvin` | `(t_kev: f64) -> f64` |
| `ideal_gas_pressure_atm` | `(n_moles: f64, temperature: f64, volume_liters: f64) -> f64` |

### processes (18 functions)

| Function | Signature |
|----------|-----------|
| `carnot_efficiency` | `(t_hot: f64, t_cold: f64) -> f64` |
| `carnot_cop_heating` | `(t_hot: f64, t_cold: f64) -> f64` |
| `carnot_cop_cooling` | `(t_hot: f64, t_cold: f64) -> f64` |
| `isothermal_work` | `(n_moles: f64, temperature: f64, v1: f64, v2: f64) -> f64` |
| `adiabatic_work` | `(n_moles: f64, cv: f64, t1: f64, t2: f64) -> f64` |
| `isobaric_work` | `(pressure: f64, v1: f64, v2: f64) -> f64` |
| `adiabatic_relation_tv` | `(t1: f64, v1: f64, v2: f64, gamma: f64) -> f64` |
| `adiabatic_relation_pv` | `(p1: f64, v1: f64, v2: f64, gamma: f64) -> f64` |
| `otto_efficiency` | `(compression_ratio: f64, gamma: f64) -> f64` |
| `diesel_efficiency` | `(compression_ratio: f64, cutoff_ratio: f64, gamma: f64) -> f64` |
| `joule_thomson_coefficient` | `(cp: f64, v_molar: f64, temperature: f64, dv_dt_p: f64) -> f64` |
| `throttling_temperature_change` | `(mu_jt: f64, dp: f64) -> f64` |
| `heat_conduction_rate` | `(k: f64, area: f64, dt: f64, dx: f64) -> f64` |
| `thermal_diffusion_1d` | `(t: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize)` |
| `mixing_entropy` | `(mole_fractions: &[f64]) -> f64` |
| `reaction_gibbs` | `(delta_g0: f64, temperature: f64, q: f64) -> f64` |
| `equilibrium_constant` | `(delta_g0: f64, temperature: f64) -> f64` |
| `vant_hoff` | `(k1: f64, delta_h: f64, t1: f64, t2: f64) -> f64` |

### statistical (15 functions)

| Function | Signature |
|----------|-----------|
| `boltzmann_distribution` | `(energy: f64, temperature: f64) -> f64` |
| `partition_function_discrete` | `(energies: &[f64], temperature: f64) -> f64` |
| `partition_function_harmonic` | `(omega: f64, temperature: f64) -> f64` |
| `mean_energy_canonical` | `(energies: &[f64], temperature: f64) -> f64` |
| `entropy_canonical` | `(energies: &[f64], temperature: f64) -> f64` |
| `fermi_dirac` | `(energy: f64, mu: f64, temperature: f64) -> f64` |
| `bose_einstein` | `(energy: f64, mu: f64, temperature: f64) -> f64` |
| `planck_radiation` | `(frequency: f64, temperature: f64) -> f64` |
| `planck_radiation_wavelength` | `(wavelength: f64, temperature: f64) -> f64` |
| `wien_displacement` | `(temperature: f64) -> f64` |
| `stefan_boltzmann_power` | `(temperature: f64, area: f64) -> f64` |
| `debye_heat_capacity` | `(temperature: f64, debye_temp: f64, n_atoms: f64) -> f64` |
| `einstein_heat_capacity` | `(temperature: f64, einstein_temp: f64, n_atoms: f64) -> f64` |
| `sackur_tetrode` | `(n_moles: f64, temperature: f64, volume: f64, mass: f64) -> f64` |
| `fermi_energy` | `(mass: f64, number_density: f64) -> f64` |

---

## Hub dispatch mapping

All physics functions are wired through:

- `src/hub/engine/dispatch/physics.rs`

```rust
use sciforge::hub::prelude::*;

let exp = Experiment::new(DomainType::Physics, "lorentz_factor")
        .param("velocity", ParameterValue::Scalar(2.0e8));

let out = ExperimentRunner::new().run(&exp)?;
```
