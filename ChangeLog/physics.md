# Physics ChangeLog

## v0.0.3

### 1️⃣ New Submodule: particle.rs — Particle Physics & Fundamental Constants (35 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `planck_energy` | `fn planck_energy() → f64` | $E_P = \sqrt{\frac{\hbar c^5}{G}}$ | `physics::particle` |
| `planck_density` | `fn planck_density() → f64` | $\rho_P = \frac{c^5}{\hbar G^2}$ | `physics::particle` |
| `planck_force` | `fn planck_force() → f64` | $F_P = \frac{c^4}{G}$ | `physics::particle` |
| `planck_pressure` | `fn planck_pressure() → f64` | $P_P = \frac{c^7}{\hbar G^2}$ | `physics::particle` |
| `planck_temperature` | `fn planck_temperature() → f64` | $T_P = \sqrt{\frac{\hbar c^5}{G k_B^2}}$ | `physics::particle` |
| `planck_charge` | `fn planck_charge() → f64` | $q_P = \sqrt{4\pi\varepsilon_0 \hbar c}$ | `physics::particle` |
| `planck_impedance` | `fn planck_impedance() → f64` | Planck impedance unit | `physics::particle` |
| `planck_angular_frequency` | `fn planck_angular_frequency() → f64` | $\omega_P = \sqrt{\frac{c^5}{\hbar G}}$ | `physics::particle` |
| `schwarzschild_radius_planck` | `fn schwarzschild_radius_planck(mass_planck_units: f64) → f64` | Schwarzschild radius in Planck units | `physics::particle` |
| `hawking_temperature` | `fn hawking_temperature(mass: f64) → f64` | $T_H = \frac{\hbar c^3}{8\pi G M k_B}$ | `physics::particle` |
| `hawking_luminosity` | `fn hawking_luminosity(mass: f64) → f64` | Hawking radiation luminosity | `physics::particle` |
| `hawking_evaporation_time` | `fn hawking_evaporation_time(mass: f64) → f64` | $t_{evap} = \frac{5120\pi G^2 M^3}{\hbar c^4}$ | `physics::particle` |
| `unruh_temperature` | `fn unruh_temperature(acceleration: f64) → f64` | $T = \frac{\hbar a}{2\pi c k_B}$ | `physics::particle` |
| `fermi_coupling_constant` | `fn fermi_coupling_constant() → f64` | $G_F$ Fermi coupling constant | `physics::particle` |
| `weak_decay_rate` | `fn weak_decay_rate(energy_gev: f64) → f64` | Weak decay rate estimate | `physics::particle` |
| `muon_decay_width` | `fn muon_decay_width() → f64` | Muon decay width | `physics::particle` |
| `fine_structure_constant` | `fn fine_structure_constant() → f64` | $\alpha = \frac{e^2}{4\pi\varepsilon_0 \hbar c} \approx 1/137$ | `physics::particle` |
| `strong_coupling_constant` | `fn strong_coupling_constant() → f64` | $\alpha_s(M_Z)$ strong coupling | `physics::particle` |
| `qcd_running_coupling` | `fn qcd_running_coupling(q_gev: f64) → f64` | QCD running coupling $\alpha_s(Q)$ | `physics::particle` |
| `electromagnetic_coupling_running` | `fn electromagnetic_coupling_running(q_gev: f64) → f64` | Running electromagnetic coupling $\alpha(Q)$ | `physics::particle` |
| `weak_mixing_angle` | `fn weak_mixing_angle() → f64` | $\sin^2\theta_W$ (Weinberg angle) | `physics::particle` |
| `w_boson_mass_gev` | `fn w_boson_mass_gev() → f64` | $M_W \approx 80.4$ GeV | `physics::particle` |
| `z_boson_mass_gev` | `fn z_boson_mass_gev() → f64` | $M_Z \approx 91.2$ GeV | `physics::particle` |
| `higgs_vev_gev` | `fn higgs_vev_gev() → f64` | $v \approx 246$ GeV (Higgs VEV) | `physics::particle` |
| `compton_time` | `fn compton_time(mass: f64) → f64` | $t_C = \frac{\hbar}{mc^2}$ | `physics::particle` |
| `gravitational_coupling` | `fn gravitational_coupling(m1: f64, m2: f64) → f64` | $\alpha_G = \frac{G m_1 m_2}{\hbar c}$ | `physics::particle` |
| `photon_energy` | `fn photon_energy(frequency: f64) → f64` | $E = h\nu$ | `physics::particle` |
| `photon_momentum` | `fn photon_momentum(frequency: f64) → f64` | $p = \frac{h\nu}{c}$ | `physics::particle` |
| `pair_production_threshold_energy` | `fn pair_production_threshold_energy() → f64` | $E_{threshold} = 2m_e c^2$ | `physics::particle` |
| `cross_section_thomson` | `fn cross_section_thomson() → f64` | $\sigma_T = \frac{8\pi}{3}\left(\frac{e^2}{m_e c^2}\right)^2$ | `physics::particle` |
| `neutrino_mass_upper_bound` | `fn neutrino_mass_upper_bound() → f64` | Current upper bound on neutrino mass | `physics::particle` |
| `neutrino_de_broglie_wavelength` | `fn neutrino_de_broglie_wavelength(energy_ev: f64) → f64` | $\lambda = \frac{h}{p}$ for neutrino | `physics::particle` |
| `classical_electron_radius` | `fn classical_electron_radius() → f64` | $r_e = \frac{e^2}{4\pi\varepsilon_0 m_e c^2}$ | `physics::particle` |
| `bohr_velocity` | `fn bohr_velocity() → f64` | $v_B = \alpha c$ | `physics::particle` |
| `schwinger_critical_field` | `fn schwinger_critical_field() → f64` | $E_S = \frac{m_e^2 c^3}{e\hbar}$ | `physics::particle` |

### 2️⃣ New File: optics/scattering.rs — Light Scattering & Atmospheric Optics (24 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `rayleigh_cross_section` | `fn rayleigh_cross_section(wavelength: f64, refractive_index: f64, depolarization: f64) → f64` | $\sigma_R \propto \lambda^{-4}$ (Rayleigh cross section) | `physics::optics::scattering` |
| `rayleigh_scattering_coefficient` | `fn rayleigh_scattering_coefficient(number_density: f64, wavelength: f64, refractive_index: f64, depolarization: f64) → f64` | $\beta_R = N \cdot \sigma_R$ | `physics::optics::scattering` |
| `rayleigh_phase_function` | `fn rayleigh_phase_function(cos_theta: f64) → f64` | $P(\theta) = \frac{3}{4}(1 + \cos^2\theta)$ | `physics::optics::scattering` |
| `mie_extinction_efficiency` | `fn mie_extinction_efficiency(size_parameter: f64, refractive_index_real: f64) → f64` | $Q_{ext}$ Mie extinction efficiency | `physics::optics::scattering` |
| `mie_scattering_coefficient` | `fn mie_scattering_coefficient(number_density: f64, particle_radius: f64, wavelength: f64, refractive_index_real: f64) → f64` | Mie scattering coefficient | `physics::optics::scattering` |
| `henyey_greenstein` | `fn henyey_greenstein(cos_theta: f64, g: f64) → f64` | $P(\cos\theta) = \frac{1-g^2}{(1+g^2-2g\cos\theta)^{3/2}}$ | `physics::optics::scattering` |
| `optical_depth_integral` | `fn optical_depth_integral(scattering_coefficient: f64, scale_height: f64, altitude_start: f64, altitude_end: f64) → f64` | $\tau = \int \beta \, ds$ | `physics::optics::scattering` |
| `transmittance` | `fn transmittance(optical_depth: f64) → f64` | $T = e^{-\tau}$ | `physics::optics::scattering` |
| `single_scattering_albedo` | `fn single_scattering_albedo(scattering_coeff: f64, absorption_coeff: f64) → f64` | $\omega_0 = \frac{\beta_s}{\beta_s + \beta_a}$ | `physics::optics::scattering` |
| `atmospheric_refraction` | `fn atmospheric_refraction(zenith_angle: f64, pressure_pa: f64, temperature_k: f64) → f64` | Atmospheric refraction correction | `physics::optics::scattering` |
| `color_temperature_to_rgb` | `fn color_temperature_to_rgb(temperature_k: f64) → (f64, f64, f64)` | Color temperature to RGB conversion | `physics::optics::scattering` |
| `planck_spectral_radiance` | `fn planck_spectral_radiance(wavelength: f64, temperature: f64) → f64` | $B_\lambda(T)$ Planck spectral radiance | `physics::optics::scattering` |
| `rayleigh_sky_color` | `fn rayleigh_sky_color(wavelength_r: f64, wavelength_g: f64, wavelength_b: f64, optical_depth_zenith: f64, cos_zenith: f64) → (f64, f64, f64)` | Sky color from Rayleigh scattering | `physics::optics::scattering` |
| `limb_darkening` | `fn limb_darkening(cos_angle: f64, coefficient: f64) → f64` | Solar limb darkening | `physics::optics::scattering` |
| `absorption_coefficient_gas` | `fn absorption_coefficient_gas(cross_section: f64, number_density: f64) → f64` | $\beta_a = \sigma \cdot N$ | `physics::optics::scattering` |
| `chapman_function` | `fn chapman_function(zenith_angle: f64, scale_height_ratio: f64) → f64` | Chapman grazing incidence function | `physics::optics::scattering` |
| `glory_angle` | `fn glory_angle(particle_radius: f64, wavelength: f64) → f64` | Glory backscattering angle | `physics::optics::scattering` |
| `rainbow_angle` | `fn rainbow_angle(refractive_index: f64) → f64` | Minimum deviation angle (rainbow) | `physics::optics::scattering` |
| `wavelength_to_energy_ev` | `fn wavelength_to_energy_ev(wavelength_nm: f64) → f64` | $E = \frac{hc}{\lambda}$ | `physics::optics::scattering` |
| `energy_ev_to_wavelength_nm` | `fn energy_ev_to_wavelength_nm(energy_ev: f64) → f64` | $\lambda = \frac{hc}{E}$ | `physics::optics::scattering` |
| `wavelength_angstrom_to_m` | `fn wavelength_angstrom_to_m(wavelength_angstrom: f64) → f64` | Å → m conversion | `physics::optics::scattering` |
| `wavelength_m_to_angstrom` | `fn wavelength_m_to_angstrom(wavelength_m: f64) → f64` | m → Å conversion | `physics::optics::scattering` |
| `photon_energy_joule_to_ev` | `fn photon_energy_joule_to_ev(energy_j: f64) → f64` | J → eV conversion | `physics::optics::scattering` |
| `size_parameter` | `fn size_parameter(radius: f64, wavelength: f64) → f64` | $x = \frac{2\pi r}{\lambda}$ | `physics::optics::scattering` |

### 3️⃣ New File: nucleosynthesis/fusion.rs — Stellar Fusion & Plasma Physics (28 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `pp_chain_rate` | `fn pp_chain_rate(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64) → f64` | Proton-proton chain energy generation rate | `physics::nucleosynthesis::fusion` |
| `cno_cycle_rate` | `fn cno_cycle_rate(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64, cno_fraction: f64) → f64` | CNO cycle energy generation rate | `physics::nucleosynthesis::fusion` |
| `triple_alpha_rate` | `fn triple_alpha_rate(temperature_k: f64, density_kg_m3: f64, helium_fraction: f64) → f64` | Triple-alpha process rate $3\alpha → {}^{12}C$ | `physics::nucleosynthesis::fusion` |
| `nuclear_energy_generation` | `fn nuclear_energy_generation(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64, helium_fraction: f64, metal_fraction: f64) → f64` | Total nuclear energy generation $\varepsilon_{nuc}$ | `physics::nucleosynthesis::fusion` |
| `thermal_pressure` | `fn thermal_pressure(electron_density: f64, temperature_k: f64) → f64` | $P_{th} = n_e k_B T$ | `physics::nucleosynthesis::fusion` |
| `magnetic_pressure` | `fn magnetic_pressure(magnetic_field: f64) → f64` | $P_B = \frac{B^2}{2\mu_0}$ | `physics::nucleosynthesis::fusion` |
| `plasma_beta` | `fn plasma_beta(electron_density: f64, temperature_k: f64, magnetic_field: f64) → f64` | $\beta = \frac{P_{th}}{P_B}$ | `physics::nucleosynthesis::fusion` |
| `alfven_speed` | `fn alfven_speed(magnetic_field: f64, density: f64) → f64` | $v_A = \frac{B}{\sqrt{\mu_0 \rho}}$ | `physics::nucleosynthesis::fusion` |
| `plasma_frequency` | `fn plasma_frequency(electron_density: f64) → f64` | $\omega_p = \sqrt{\frac{n_e e^2}{m_e \varepsilon_0}}$ | `physics::nucleosynthesis::fusion` |
| `debye_length` | `fn debye_length(electron_density: f64, temperature_k: f64) → f64` | $\lambda_D = \sqrt{\frac{\varepsilon_0 k_B T}{n_e e^2}}$ | `physics::nucleosynthesis::fusion` |
| `radiative_loss_rate` | `fn radiative_loss_rate(electron_density: f64, temperature_k: f64) → f64` | Radiative cooling rate | `physics::nucleosynthesis::fusion` |
| `thermal_conduction_flux` | `fn thermal_conduction_flux(temperature_k: f64, length_scale: f64) → f64` | Spitzer thermal conduction | `physics::nucleosynthesis::fusion` |
| `sound_speed_plasma` | `fn sound_speed_plasma(temperature_k: f64, mean_particle_mass: f64) → f64` | $c_s = \sqrt{\frac{k_B T}{m}}$ | `physics::nucleosynthesis::fusion` |
| `gyrofrequency` | `fn gyrofrequency(charge: f64, magnetic_field: f64, mass: f64) → f64` | $\omega_c = \frac{qB}{m}$ | `physics::nucleosynthesis::fusion` |
| `larmor_radius` | `fn larmor_radius(mass: f64, velocity_perp: f64, charge: f64, magnetic_field: f64) → f64` | $r_L = \frac{mv_\perp}{qB}$ | `physics::nucleosynthesis::fusion` |
| `reconnection_rate_sweet_parker` | `fn reconnection_rate_sweet_parker(alfven_speed_val: f64, lundquist_number: f64) → f64` | $v_R = v_A / \sqrt{S}$ (Sweet-Parker) | `physics::nucleosynthesis::fusion` |
| `lundquist_number` | `fn lundquist_number(alfven_speed_val: f64, length_scale: f64, magnetic_diffusivity: f64) → f64` | $S = \frac{v_A L}{\eta}$ | `physics::nucleosynthesis::fusion` |
| `coronal_loop_temperature` | `fn coronal_loop_temperature(loop_length: f64, heating_rate: f64) → f64` | RTV coronal loop scaling law | `physics::nucleosynthesis::fusion` |
| `coronal_loop_density` | `fn coronal_loop_density(heating_rate: f64, loop_length: f64, temperature_k: f64) → f64` | Coronal loop density from heating | `physics::nucleosynthesis::fusion` |
| `solar_flare_energy` | `fn solar_flare_energy(magnetic_field: f64, volume: f64) → f64` | $E = \frac{B^2}{2\mu_0} V$ | `physics::nucleosynthesis::fusion` |
| `cme_kinetic_energy` | `fn cme_kinetic_energy(mass: f64, velocity: f64) → f64` | $E_k = \frac{1}{2}mv^2$ (CME) | `physics::nucleosynthesis::fusion` |
| `sunspot_temperature` | `fn sunspot_temperature(photosphere_temp: f64, suppression_factor: f64) → f64` | Sunspot umbral temperature | `physics::nucleosynthesis::fusion` |
| `differential_rotation_rate` | `fn differential_rotation_rate(equatorial_rate: f64, latitude: f64, a2: f64, a4: f64) → f64` | $\Omega(\phi) = \Omega_{eq} - A_2\sin^2\phi - A_4\sin^4\phi$ | `physics::nucleosynthesis::fusion` |
| `stellar_wind_mass_loss_si` | `fn stellar_wind_mass_loss_si(luminosity: f64, escape_velocity: f64) → f64` | Stellar wind mass loss (SI) | `physics::nucleosynthesis::fusion` |
| `convective_envelope_depth` | `fn convective_envelope_depth(stellar_mass_solar: f64) → f64` | Convective envelope depth estimate | `physics::nucleosynthesis::fusion` |
| `mixing_length_velocity` | `fn mixing_length_velocity(convective_flux: f64, density: f64, temperature: f64, pressure: f64, mixing_length: f64, g_local: f64, cp: f64) → f64` | MLT convective velocity | `physics::nucleosynthesis::fusion` |
| `opacity_kramers` | `fn opacity_kramers(density: f64, temperature: f64, hydrogen_fraction: f64, metal_fraction: f64) → f64` | $\kappa \propto \rho T^{-3.5}$ (Kramers) | `physics::nucleosynthesis::fusion` |
| `radiative_temperature_gradient` | `fn radiative_temperature_gradient(opacity: f64, luminosity: f64, mass_enclosed: f64, pressure: f64, temperature: f64) → f64` | $\nabla_{rad} = \frac{3\kappa L P}{16\pi ac G M T^4}$ | `physics::nucleosynthesis::fusion` |

### 4️⃣ New File: relativity/accretion.rs — Black Hole Accretion & Relativistic Jets (24 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `schwarzschild_radius` | `fn schwarzschild_radius(mass: f64) → f64` | $r_s = \frac{2GM}{c^2}$ | `physics::relativity::accretion` |
| `kerr_event_horizon` | `fn kerr_event_horizon(mass: f64, spin_parameter: f64) → f64` | $r_+ = \frac{GM}{c^2}\left(1 + \sqrt{1-a^2}\right)$ | `physics::relativity::accretion` |
| `kerr_ergosphere` | `fn kerr_ergosphere(mass: f64, spin_parameter: f64, theta: f64) → f64` | Ergosphere outer boundary (Kerr) | `physics::relativity::accretion` |
| `isco_radius` | `fn isco_radius(mass: f64, spin_parameter: f64) → f64` | Innermost stable circular orbit | `physics::relativity::accretion` |
| `accretion_disk_temperature` | `fn accretion_disk_temperature(mass: f64, accretion_rate: f64, r: f64, r_inner: f64) → f64` | $T(r)$ Shakura-Sunyaev disk temperature | `physics::relativity::accretion` |
| `accretion_disk_luminosity` | `fn accretion_disk_luminosity(mass: f64, accretion_rate: f64, r_inner: f64) → f64` | $L = \frac{GM\dot{M}}{2r_{in}}$ | `physics::relativity::accretion` |
| `radiative_efficiency` | `fn radiative_efficiency(r_isco: f64, mass: f64) → f64` | $\eta = 1 - \sqrt{1 - \frac{2GM}{r_{isco}c^2}}$ | `physics::relativity::accretion` |
| `eddington_accretion_rate` | `fn eddington_accretion_rate(mass: f64) → f64` | $\dot{M}_{Edd}$ Eddington accretion rate | `physics::relativity::accretion` |
| `disk_surface_density` | `fn disk_surface_density(accretion_rate: f64, viscosity: f64, r: f64, r_inner: f64) → f64` | $\Sigma$ disk surface density | `physics::relativity::accretion` |
| `alpha_viscosity` | `fn alpha_viscosity(alpha: f64, sound_speed: f64, scale_height: f64) → f64` | $\nu = \alpha c_s H$ (Shakura-Sunyaev) | `physics::relativity::accretion` |
| `disk_scale_height` | `fn disk_scale_height(sound_speed: f64, orbital_frequency: f64) → f64` | $H = \frac{c_s}{\Omega_K}$ | `physics::relativity::accretion` |
| `orbital_frequency_kerr` | `fn orbital_frequency_kerr(mass: f64, r: f64, spin_parameter: f64) → f64` | Keplerian orbital frequency (Kerr) | `physics::relativity::accretion` |
| `gravitational_redshift` | `fn gravitational_redshift(mass: f64, r: f64) → f64` | $z = \frac{1}{\sqrt{1-r_s/r}} - 1$ | `physics::relativity::accretion` |
| `doppler_beaming_factor` | `fn doppler_beaming_factor(velocity_los: f64, bulk_lorentz: f64) → f64` | Relativistic Doppler beaming $\delta$ | `physics::relativity::accretion` |
| `apparent_superluminal_velocity` | `fn apparent_superluminal_velocity(beta: f64, viewing_angle: f64) → f64` | $\beta_{app} = \frac{\beta\sin\theta}{1-\beta\cos\theta}$ | `physics::relativity::accretion` |
| `blandford_znajek_power` | `fn blandford_znajek_power(magnetic_field: f64, event_horizon_radius: f64, spin_parameter: f64, mass: f64) → f64` | BZ jet power extraction | `physics::relativity::accretion` |
| `jet_lorentz_factor_from_ratio` | `fn jet_lorentz_factor_from_ratio(jet_power: f64, mass_loading_rate: f64) → f64` | $\Gamma = P_j / \dot{M}c^2$ | `physics::relativity::accretion` |
| `synchrotron_cooling_time` | `fn synchrotron_cooling_time(electron_gamma: f64, magnetic_field: f64) → f64` | Synchrotron cooling timescale | `physics::relativity::accretion` |
| `photon_ring_radius` | `fn photon_ring_radius(mass: f64) → f64` | $r_{ph} = 3GM/c^2$ | `physics::relativity::accretion` |
| `shadow_angular_radius` | `fn shadow_angular_radius(mass: f64, distance: f64) → f64` | Black hole shadow angular size | `physics::relativity::accretion` |
| `advection_dominated_temperature` | `fn advection_dominated_temperature(proton_mass: f64, r: f64, mass: f64) → f64` | ADAF ion temperature | `physics::relativity::accretion` |
| `comptonization_parameter` | `fn comptonization_parameter(electron_temperature: f64, optical_depth: f64) → f64` | $y = 4\frac{k_BT_e}{m_ec^2}\max(\tau,\tau^2)$ | `physics::relativity::accretion` |
| `bondi_accretion_rate` | `fn bondi_accretion_rate(mass: f64, ambient_density: f64, sound_speed: f64) → f64` | $\dot{M}_B = 4\pi\rho_\infty \frac{(GM)^2}{c_s^3}$ | `physics::relativity::accretion` |
| `tidal_disruption_radius` | `fn tidal_disruption_radius(bh_mass: f64, star_mass: f64, star_radius: f64) → f64` | $r_T = R_* \left(\frac{M_{BH}}{M_*}\right)^{1/3}$ | `physics::relativity::accretion` |

### 5️⃣ Changes to Existing Submodules

15 source files modified across all existing submodules (acoustics, electrodynamics, electronics, fluid_mechanics, materials, mechanics, nucleosynthesis, optics, quantum, relativity, solid_mechanics, thermodynamics).

### 6️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 38 → 74 (+36) |
| Test files | 6 → 17 |
| Total suite | 814 tests, 0 warnings |

Test files: `physics/main.rs`, `physics/acoustics.rs`, `physics/electrodynamics.rs`, `physics/electronics.rs`, `physics/fluid_mechanics.rs`, `physics/materials.rs`, `physics/mechanics.rs`, `physics/nucleosynthesis.rs`, `physics/optics.rs`, `physics/particle.rs`, `physics/quantum.rs`, `physics/relativity.rs`, `physics/solid_mechanics.rs`, `physics/thermodynamics.rs`, `physics/dispatch.rs`

---

## v0.0.2

### 1️⃣ Changes

All 11 submodules rewritten (all files modified).

### 2️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 0 → 38 |
| Test files | 6 |

| File | Tests |
|---|---|
| `physics/electromagnetism.rs` | 10 |
| `physics/relativity.rs` | 8 |
| `physics/mechanics.rs` | 7 |
| `physics/dispatch.rs` | 6 |
| `physics/thermodynamics.rs` | 6 |
| `physics/main.rs` | 1 |

---

## v0.0.1

Module: `src/physics/` — 11 submodules, 57 files, 820+ pub fn

### 1️⃣ relativity/ — Special & General Relativity

#### lorentz.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gamma_factor` | `fn gamma_factor(v: f64) → f64` | $\gamma = \frac{1}{\sqrt{1-v^2/c^2}}$ | `physics::relativity::lorentz` |
| `beta` | `fn beta(v: f64) → f64` | $\beta = v/c$ | `physics::relativity::lorentz` |
| `time_dilation` | `fn time_dilation(proper_time: f64, v: f64) → f64` | $\Delta t = \gamma \Delta\tau$ | `physics::relativity::lorentz` |
| `length_contraction` | `fn length_contraction(proper_length: f64, v: f64) → f64` | $L = L_0 / \gamma$ | `physics::relativity::lorentz` |
| `lorentz_transform` | `fn lorentz_transform(t: f64, x: f64, v: f64) → (f64, f64)` | $(t', x')$ Lorentz transformation | `physics::relativity::lorentz` |
| `inverse_lorentz_transform` | `fn inverse_lorentz_transform(t_prime: f64, x_prime: f64, v: f64) → (f64, f64)` | Inverse Lorentz transformation | `physics::relativity::lorentz` |
| `lorentz_transform_4` | `fn lorentz_transform_4(event: [f64; 4], v: [f64; 3]) → [f64; 4]` | 4D Lorentz boost | `physics::relativity::lorentz` |
| `boost_matrix` | `fn boost_matrix(v: [f64; 3]) → [[f64; 4]; 4]` | $\Lambda$ boost matrix | `physics::relativity::lorentz` |
| `rapidity` | `fn rapidity(v: f64) → f64` | $\phi = \text{atanh}(\beta)$ | `physics::relativity::lorentz` |
| `velocity_from_rapidity` | `fn velocity_from_rapidity(phi: f64) → f64` | $v = c\tanh\phi$ | `physics::relativity::lorentz` |

#### kinematics.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `velocity_addition` | `fn velocity_addition(u: f64, v: f64) → f64` | $w = \frac{u+v}{1+uv/c^2}$ | `physics::relativity::kinematics` |
| `velocity_addition_3d` | `fn velocity_addition_3d(u: [f64; 3], v: f64, dir: [f64; 3]) → [f64; 3]` | 3D relativistic velocity addition | `physics::relativity::kinematics` |
| `relativistic_doppler` | `fn relativistic_doppler(freq: f64, v: f64, angle: f64) → f64` | $f' = f\frac{\sqrt{1-\beta^2}}{1-\beta\cos\theta}$ | `physics::relativity::kinematics` |
| `transverse_doppler` | `fn transverse_doppler(freq: f64, v: f64) → f64` | $f' = f/\gamma$ | `physics::relativity::kinematics` |
| `aberration` | `fn aberration(theta: f64, v: f64) → f64` | Relativistic aberration of light | `physics::relativity::kinematics` |
| `headlight_effect` | `fn headlight_effect(theta_rest: f64, v: f64) → f64` | Relativistic beaming / headlight effect | `physics::relativity::kinematics` |
| `proper_acceleration_to_coordinate` | `fn proper_acceleration_to_coordinate(proper_accel: f64, proper_time: f64) → (f64, f64)` | Uniformly accelerated motion $(x, t)$ | `physics::relativity::kinematics` |
| `twin_paradox_age` | `fn twin_paradox_age(v: f64, coord_time: f64) → f64` | Twin paradox proper time | `physics::relativity::kinematics` |

#### dynamics.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `relativistic_momentum` | `fn relativistic_momentum(mass: f64, v: f64) → f64` | $p = \gamma mv$ | `physics::relativity::dynamics` |
| `relativistic_energy` | `fn relativistic_energy(mass: f64, v: f64) → f64` | $E = \gamma mc^2$ | `physics::relativity::dynamics` |
| `rest_energy` | `fn rest_energy(mass: f64) → f64` | $E_0 = mc^2$ | `physics::relativity::dynamics` |
| `kinetic_energy_relativistic` | `fn kinetic_energy_relativistic(mass: f64, v: f64) → f64` | $K = (\gamma - 1)mc^2$ | `physics::relativity::dynamics` |
| `energy_momentum_relation` | `fn energy_momentum_relation(mass: f64, momentum: f64) → f64` | $E^2 = (pc)^2 + (mc^2)^2$ | `physics::relativity::dynamics` |
| `four_momentum` | `fn four_momentum(mass: f64, v: [f64; 3]) → [f64; 4]` | $p^\mu = (\gamma mc, \gamma m\vec{v})$ | `physics::relativity::dynamics` |
| `invariant_mass_two_body` | `fn invariant_mass_two_body(p1: [f64; 4], p2: [f64; 4]) → f64` | $M^2 = -(p_1+p_2)^\mu(p_1+p_2)_\mu$ | `physics::relativity::dynamics` |
| `mandelstam_s` | `fn mandelstam_s(p1: [f64; 4], p2: [f64; 4]) → f64` | $s = (p_1+p_2)^2$ Mandelstam variable | `physics::relativity::dynamics` |
| `compton_wavelength_shift` | `fn compton_wavelength_shift(angle: f64) → f64` | $\Delta\lambda = \frac{h}{m_ec}(1-\cos\theta)$ | `physics::relativity::dynamics` |
| `relativistic_kinetic_energy_from_gamma` | `fn relativistic_kinetic_energy_from_gamma(mass: f64, gamma: f64) → f64` | $K = (\gamma-1)mc^2$ from $\gamma$ | `physics::relativity::dynamics` |
| `threshold_energy` | `fn threshold_energy(m_target: f64, m_products_sum: f64) → f64` | Threshold energy for particle production | `physics::relativity::dynamics` |
| `synchrotron_power` | `fn synchrotron_power(charge: f64, mass: f64, gamma: f64, radius: f64) → f64` | $P = \frac{q^2c\gamma^4}{6\pi\varepsilon_0 R^2}$ | `physics::relativity::dynamics` |
| `bremsstrahlung_power_classical` | `fn bremsstrahlung_power_classical(charge: f64, accel: f64) → f64` | $P = \frac{q^2a^2}{6\pi\varepsilon_0 c^3}$ (Larmor) | `physics::relativity::dynamics` |

### 2️⃣ optics/ — Wave Optics

#### refraction.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `snell` | `fn snell(n1: f64, theta1: f64, n2: f64) → f64` | $n_1\sin\theta_1 = n_2\sin\theta_2$ | `physics::optics::refraction` |
| `critical_angle` | `fn critical_angle(n1: f64, n2: f64) → f64` | $\theta_c = \arcsin(n_2/n_1)$ | `physics::optics::refraction` |
| `brewster_angle` | `fn brewster_angle(n1: f64, n2: f64) → f64` | $\theta_B = \arctan(n_2/n_1)$ | `physics::optics::refraction` |
| `fresnel_reflectance_s` | `fn fresnel_reflectance_s(n1: f64, theta_i: f64, n2: f64, theta_t: f64) → f64` | Fresnel $r_s$ coefficient | `physics::optics::refraction` |
| `fresnel_reflectance_p` | `fn fresnel_reflectance_p(n1: f64, theta_i: f64, n2: f64, theta_t: f64) → f64` | Fresnel $r_p$ coefficient | `physics::optics::refraction` |
| `thin_lens_equation` | `fn thin_lens_equation(focal: f64, object_dist: f64) → f64` | $\frac{1}{f} = \frac{1}{d_o} + \frac{1}{d_i}$ | `physics::optics::refraction` |
| `magnification` | `fn magnification(image_dist: f64, object_dist: f64) → f64` | $M = -d_i/d_o$ | `physics::optics::refraction` |
| `lensmaker_equation` | `fn lensmaker_equation(n: f64, r1: f64, r2: f64) → f64` | $\frac{1}{f} = (n-1)\left(\frac{1}{R_1}-\frac{1}{R_2}\right)$ | `physics::optics::refraction` |
| `numerical_aperture` | `fn numerical_aperture(n: f64, half_angle: f64) → f64` | $NA = n\sin\theta$ | `physics::optics::refraction` |
| `optical_path_length` | `fn optical_path_length(n: f64, d: f64) → f64` | $OPL = nd$ | `physics::optics::refraction` |
| `cauchy_dispersion` | `fn cauchy_dispersion(a: f64, b: f64, wavelength: f64) → f64` | $n(\lambda) = A + B/\lambda^2$ | `physics::optics::refraction` |
| `abbe_number` | `fn abbe_number(nd: f64, nf: f64, nc: f64) → f64` | $V = \frac{n_d - 1}{n_F - n_C}$ | `physics::optics::refraction` |

#### diffraction.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `single_slit_intensity` | `fn single_slit_intensity(theta: f64, a: f64, wavelength: f64) → f64` | $I = I_0\left(\frac{\sin\alpha}{\alpha}\right)^2$ | `physics::optics::diffraction` |
| `double_slit_intensity` | `fn double_slit_intensity(theta: f64, d: f64, wavelength: f64) → f64` | $I = 4I_0\cos^2\delta$ | `physics::optics::diffraction` |
| `diffraction_grating_maxima` | `fn diffraction_grating_maxima(d: f64, wavelength: f64, order: i32) → f64` | $d\sin\theta = m\lambda$ | `physics::optics::diffraction` |
| `resolving_power_grating` | `fn resolving_power_grating(order: i32, n_slits: u32) → f64` | $R = mN$ | `physics::optics::diffraction` |
| `rayleigh_criterion` | `fn rayleigh_criterion(wavelength: f64, aperture: f64) → f64` | $\theta = 1.22\lambda/D$ | `physics::optics::diffraction` |
| `airy_disk_radius` | `fn airy_disk_radius(wavelength: f64, f_number: f64) → f64` | $r = 1.22\lambda f/\#$ | `physics::optics::diffraction` |
| `fraunhofer_distance` | `fn fraunhofer_distance(aperture: f64, wavelength: f64) → f64` | $z_F = 2D^2/\lambda$ | `physics::optics::diffraction` |
| `grating_dispersion` | `fn grating_dispersion(order: i32, d: f64, theta: f64) → f64` | $\frac{d\theta}{d\lambda} = \frac{m}{d\cos\theta}$ | `physics::optics::diffraction` |
| `bragg_condition` | `fn bragg_condition(d_spacing: f64, theta: f64, wavelength: f64) → f64` | $2d\sin\theta = n\lambda$ | `physics::optics::diffraction` |
| `circular_aperture_first_zero` | `fn circular_aperture_first_zero(wavelength: f64, diameter: f64) → f64` | First zero of Airy pattern | `physics::optics::diffraction` |

#### interference.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `two_beam_intensity` | `fn two_beam_intensity(i1: f64, i2: f64, delta_phase: f64) → f64` | $I = I_1 + I_2 + 2\sqrt{I_1 I_2}\cos\delta$ | `physics::optics::interference` |
| `thin_film_phase_shift` | `fn thin_film_phase_shift(n: f64, thickness: f64, wavelength: f64, theta: f64) → f64` | $\delta = \frac{4\pi nt\cos\theta}{\lambda}$ | `physics::optics::interference` |
| `constructive_condition` | `fn constructive_condition(optical_path_diff: f64, wavelength: f64) → bool` | $\Delta = m\lambda$ condition | `physics::optics::interference` |
| `fringe_spacing` | `fn fringe_spacing(wavelength: f64, d: f64, l: f64) → f64` | $\Delta y = \frac{\lambda L}{d}$ | `physics::optics::interference` |
| `visibility` | `fn visibility(i_max: f64, i_min: f64) → f64` | $V = \frac{I_{max}-I_{min}}{I_{max}+I_{min}}$ | `physics::optics::interference` |
| `coherence_length` | `fn coherence_length(wavelength: f64, delta_wavelength: f64) → f64` | $l_c = \lambda^2/\Delta\lambda$ | `physics::optics::interference` |
| `coherence_time` | `fn coherence_time(delta_nu: f64) → f64` | $\tau_c = 1/\Delta\nu$ | `physics::optics::interference` |
| `fabry_perot_transmittance` | `fn fabry_perot_transmittance(r: f64, delta: f64) → f64` | Airy function (Fabry-Pérot) | `physics::optics::interference` |
| `fabry_perot_finesse` | `fn fabry_perot_finesse(r: f64) → f64` | $\mathcal{F} = \frac{\pi\sqrt{R}}{1-R}$ | `physics::optics::interference` |
| `free_spectral_range` | `fn free_spectral_range(d: f64, n: f64) → f64` | $FSR = \frac{c}{2nd}$ | `physics::optics::interference` |
| `michelson_path_difference` | `fn michelson_path_difference(mirror_displacement: f64) → f64` | $\Delta = 2d$ | `physics::optics::interference` |
| `newton_ring_radius` | `fn newton_ring_radius(m: u32, wavelength: f64, r: f64) → f64` | $r_m = \sqrt{m\lambda R}$ | `physics::optics::interference` |

#### polarization.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `malus_law` | `fn malus_law(i0: f64, theta: f64) → f64` | $I = I_0\cos^2\theta$ | `physics::optics::polarization` |
| `brewster_reflectance` | `fn brewster_reflectance(n1: f64, n2: f64) → f64` | Reflectance at Brewster angle | `physics::optics::polarization` |
| `stokes_parameters` | `fn stokes_parameters(ex: f64, ey: f64, delta: f64) → [f64; 4]` | $(S_0, S_1, S_2, S_3)$ Stokes vector | `physics::optics::polarization` |
| `degree_of_polarization` | `fn degree_of_polarization(s: &[f64; 4]) → f64` | $DOP = \frac{\sqrt{S_1^2+S_2^2+S_3^2}}{S_0}$ | `physics::optics::polarization` |
| `jones_rotation` | `fn jones_rotation(theta: f64) → [[f64; 2]; 2]` | Jones rotation matrix | `physics::optics::polarization` |
| `quarter_wave_plate_phase` | `fn quarter_wave_plate_phase(wavelength: f64, n_fast: f64, n_slow: f64) → f64` | Quarter-wave plate retardation | `physics::optics::polarization` |
| `specific_rotation` | `fn specific_rotation(observed: f64, l: f64, c: f64) → f64` | $[\alpha] = \frac{\alpha_{obs}}{l \cdot c}$ | `physics::optics::polarization` |
| `ellipticity` | `fn ellipticity(major: f64, minor: f64) → f64` | $e = b/a$ | `physics::optics::polarization` |
| `circular_dichroism` | `fn circular_dichroism(a_left: f64, a_right: f64) → f64` | $CD = A_L - A_R$ | `physics::optics::polarization` |
| `birefringence` | `fn birefringence(n_extraordinary: f64, n_ordinary: f64) → f64` | $\Delta n = n_e - n_o$ | `physics::optics::polarization` |
| `retardance` | `fn retardance(birefringence: f64, thickness: f64, wavelength: f64) → f64` | $\Gamma = \frac{2\pi \Delta n \cdot d}{\lambda}$ | `physics::optics::polarization` |

### 3️⃣ electronics/ — Circuit Design & Digital Logic

#### circuits.rs (25 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ohm_voltage` | `fn ohm_voltage(i: f64, r: f64) → f64` | $V = IR$ | `physics::electronics::circuits` |
| `ohm_current` | `fn ohm_current(v: f64, r: f64) → f64` | $I = V/R$ | `physics::electronics::circuits` |
| `ohm_resistance` | `fn ohm_resistance(v: f64, i: f64) → f64` | $R = V/I$ | `physics::electronics::circuits` |
| `series_resistance` | `fn series_resistance(resistors: &[f64]) → f64` | $R_{total} = \sum R_i$ | `physics::electronics::circuits` |
| `parallel_resistance` | `fn parallel_resistance(resistors: &[f64]) → f64` | $\frac{1}{R} = \sum\frac{1}{R_i}$ | `physics::electronics::circuits` |
| `voltage_divider` | `fn voltage_divider(v_in: f64, r1: f64, r2: f64) → f64` | $V_{out} = V_{in}\frac{R_2}{R_1+R_2}$ | `physics::electronics::circuits` |
| `current_divider` | `fn current_divider(i_total: f64, r_branch: f64, r_total_parallel: f64) → f64` | $I_k = I_{total}\frac{R_{total}}{R_k}$ | `physics::electronics::circuits` |
| `power_dc` | `fn power_dc(v: f64, i: f64) → f64` | $P = VI$ | `physics::electronics::circuits` |
| `rc_charging` | `fn rc_charging(v_source: f64, t: f64, r: f64, c: f64) → f64` | $V(t) = V_s(1 - e^{-t/RC})$ | `physics::electronics::circuits` |
| `rc_discharging` | `fn rc_discharging(v0: f64, t: f64, r: f64, c: f64) → f64` | $V(t) = V_0 e^{-t/RC}$ | `physics::electronics::circuits` |
| `rl_current_rise` | `fn rl_current_rise(v: f64, r: f64, l: f64, t: f64) → f64` | $I(t) = \frac{V}{R}(1-e^{-Rt/L})$ | `physics::electronics::circuits` |
| `rl_current_decay` | `fn rl_current_decay(i0: f64, r: f64, l: f64, t: f64) → f64` | $I(t) = I_0 e^{-Rt/L}$ | `physics::electronics::circuits` |
| `rlc_resonant_frequency` | `fn rlc_resonant_frequency(l: f64, c: f64) → f64` | $f_0 = \frac{1}{2\pi\sqrt{LC}}$ | `physics::electronics::circuits` |
| `rlc_quality_factor` | `fn rlc_quality_factor(r: f64, l: f64, c: f64) → f64` | $Q = \frac{1}{R}\sqrt{\frac{L}{C}}$ | `physics::electronics::circuits` |
| `rlc_bandwidth` | `fn rlc_bandwidth(f0: f64, q: f64) → f64` | $BW = f_0/Q$ | `physics::electronics::circuits` |
| `impedance_capacitor` | `fn impedance_capacitor(c: f64, freq: f64) → (f64, f64)` | $Z_C = \frac{1}{j\omega C}$ | `physics::electronics::circuits` |
| `impedance_inductor` | `fn impedance_inductor(l: f64, freq: f64) → (f64, f64)` | $Z_L = j\omega L$ | `physics::electronics::circuits` |
| `impedance_magnitude` | `fn impedance_magnitude(re: f64, im: f64) → f64` | $|Z| = \sqrt{R^2 + X^2}$ | `physics::electronics::circuits` |
| `impedance_phase` | `fn impedance_phase(re: f64, im: f64) → f64` | $\phi = \arctan(X/R)$ | `physics::electronics::circuits` |
| `capacitor_energy` | `fn capacitor_energy(c: f64, v: f64) → f64` | $E = \frac{1}{2}CV^2$ | `physics::electronics::circuits` |
| `inductor_energy` | `fn inductor_energy(l: f64, i: f64) → f64` | $E = \frac{1}{2}LI^2$ | `physics::electronics::circuits` |
| `wheatstone_bridge_voltage` | `fn wheatstone_bridge_voltage(v_in: f64, r1: f64, r2: f64, r3: f64, r4: f64) → f64` | Wheatstone bridge output | `physics::electronics::circuits` |
| `thevenin_voltage` | `fn thevenin_voltage(v_oc: f64) → f64` | $V_{th} = V_{oc}$ | `physics::electronics::circuits` |
| `thevenin_resistance` | `fn thevenin_resistance(v_oc: f64, i_sc: f64) → f64` | $R_{th} = V_{oc}/I_{sc}$ | `physics::electronics::circuits` |
| `max_power_transfer` | `fn max_power_transfer(v_th: f64, r_th: f64) → f64` | $P_{max} = \frac{V_{th}^2}{4R_{th}}$ | `physics::electronics::circuits` |

#### digital.rs (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `and_gate` | `fn and_gate(a: bool, b: bool) → bool` | AND logic gate | `physics::electronics::digital` |
| `or_gate` | `fn or_gate(a: bool, b: bool) → bool` | OR logic gate | `physics::electronics::digital` |
| `not_gate` | `fn not_gate(a: bool) → bool` | NOT logic gate | `physics::electronics::digital` |
| `nand_gate` | `fn nand_gate(a: bool, b: bool) → bool` | NAND logic gate | `physics::electronics::digital` |
| `nor_gate` | `fn nor_gate(a: bool, b: bool) → bool` | NOR logic gate | `physics::electronics::digital` |
| `xor_gate` | `fn xor_gate(a: bool, b: bool) → bool` | XOR logic gate | `physics::electronics::digital` |
| `xnor_gate` | `fn xnor_gate(a: bool, b: bool) → bool` | XNOR logic gate | `physics::electronics::digital` |
| `half_adder` | `fn half_adder(a: bool, b: bool) → (bool, bool)` | Half adder (sum, carry) | `physics::electronics::digital` |
| `full_adder` | `fn full_adder(a: bool, b: bool, cin: bool) → (bool, bool)` | Full adder (sum, carry) | `physics::electronics::digital` |
| `ripple_carry_adder` | `fn ripple_carry_adder(a: &[bool], b: &[bool]) → (Vec<bool>, bool)` | N-bit ripple carry adder | `physics::electronics::digital` |
| `multiplexer_2to1` | `fn multiplexer_2to1(a: bool, b: bool, sel: bool) → bool` | 2:1 multiplexer | `physics::electronics::digital` |
| `demultiplexer_1to2` | `fn demultiplexer_1to2(input: bool, sel: bool) → (bool, bool)` | 1:2 demultiplexer | `physics::electronics::digital` |
| `decoder_2to4` | `fn decoder_2to4(a: bool, b: bool) → [bool; 4]` | 2-to-4 decoder | `physics::electronics::digital` |
| `encoder_4to2` | `fn encoder_4to2(inputs: &[bool; 4]) → (bool, bool)` | 4-to-2 priority encoder | `physics::electronics::digital` |
| `sr_latch` | `fn sr_latch(s: bool, r: bool, q_prev: bool) → bool` | SR latch | `physics::electronics::digital` |
| `d_flip_flop` | `fn d_flip_flop(d: bool, _: bool) → bool` | D flip-flop | `physics::electronics::digital` |
| `jk_flip_flop` | `fn jk_flip_flop(j: bool, k: bool, q_prev: bool) → bool` | JK flip-flop | `physics::electronics::digital` |
| `binary_to_gray` | `fn binary_to_gray(binary: u32) → u32` | Binary to Gray code | `physics::electronics::digital` |
| `gray_to_binary` | `fn gray_to_binary(gray: u32) → u32` | Gray code to binary | `physics::electronics::digital` |
| `ones_complement` | `fn ones_complement(val: u32, bits: u32) → u32` | One's complement | `physics::electronics::digital` |
| `twos_complement` | `fn twos_complement(val: u32, bits: u32) → u32` | Two's complement | `physics::electronics::digital` |

#### semiconductor_devices.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `diode_shockley` | `fn diode_shockley(is: f64, v: f64, n: f64, vt: f64) → f64` | $I = I_s(e^{V/nV_T} - 1)$ | `physics::electronics::semiconductor_devices` |
| `zener_voltage_regulation` | `fn zener_voltage_regulation(v_in: f64, v_zener: f64) → f64` | Zener regulation output | `physics::electronics::semiconductor_devices` |
| `bjt_ic_active` | `fn bjt_ic_active(beta: f64, ib: f64) → f64` | $I_C = \beta I_B$ | `physics::electronics::semiconductor_devices` |
| `bjt_ie` | `fn bjt_ie(ic: f64, ib: f64) → f64` | $I_E = I_C + I_B$ | `physics::electronics::semiconductor_devices` |
| `bjt_alpha` | `fn bjt_alpha(beta: f64) → f64` | $\alpha = \frac{\beta}{\beta+1}$ | `physics::electronics::semiconductor_devices` |
| `mosfet_drain_current_saturation` | `fn mosfet_drain_current_saturation(kn: f64, vgs: f64, vth: f64) → f64` | $I_D = \frac{k_n}{2}(V_{GS}-V_{th})^2$ | `physics::electronics::semiconductor_devices` |
| `mosfet_drain_current_linear` | `fn mosfet_drain_current_linear(kn: f64, vgs: f64, vth: f64, vds: f64) → f64` | $I_D = k_n[(V_{GS}-V_{th})V_{DS} - V_{DS}^2/2]$ | `physics::electronics::semiconductor_devices` |
| `mosfet_threshold_body_effect` | `fn mosfet_threshold_body_effect(vth0: f64, gamma: f64, vsb: f64, phi: f64) → f64` | $V_{th} = V_{th0} + \gamma(\sqrt{2\phi+V_{SB}}-\sqrt{2\phi})$ | `physics::electronics::semiconductor_devices` |
| `solar_cell_iv` | `fn solar_cell_iv(i_photo: f64, i0: f64, v: f64, n: f64, vt: f64, r_s: f64) → f64` | Solar cell I-V characteristic | `physics::electronics::semiconductor_devices` |
| `led_resistor` | `fn led_resistor(v_supply: f64, v_led: f64, i_led: f64) → f64` | $R = (V_s - V_{LED})/I_{LED}$ | `physics::electronics::semiconductor_devices` |
| `photodiode_responsivity` | `fn photodiode_responsivity(i_photo: f64, p_optical: f64) → f64` | $\mathcal{R} = I_{ph}/P_{opt}$ | `physics::electronics::semiconductor_devices` |
| `tunnel_diode_current` | `fn tunnel_diode_current(ip: f64, iv: f64, vp: f64, vv: f64, v: f64) → f64` | Tunnel diode characteristic | `physics::electronics::semiconductor_devices` |
| `pn_junction_capacitance` | `fn pn_junction_capacitance(c0: f64, v: f64, v_bi: f64, m: f64) → f64` | $C = C_0(1-V/V_{bi})^{-m}$ | `physics::electronics::semiconductor_devices` |
| `early_effect` | `fn early_effect(ic0: f64, vce: f64, va: f64) → f64` | $I_C = I_{C0}(1 + V_{CE}/V_A)$ | `physics::electronics::semiconductor_devices` |
| `drain_induced_barrier_lowering` | `fn drain_induced_barrier_lowering(vth0: f64, sigma: f64, vds: f64) → f64` | $V_{th} = V_{th0} - \sigma V_{DS}$ | `physics::electronics::semiconductor_devices` |

#### amplifiers.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `inverting_gain` | `fn inverting_gain(r_f: f64, r_in: f64) → f64` | $A_v = -R_f/R_{in}$ | `physics::electronics::amplifiers` |
| `non_inverting_gain` | `fn non_inverting_gain(r_f: f64, r_in: f64) → f64` | $A_v = 1 + R_f/R_{in}$ | `physics::electronics::amplifiers` |
| `differential_gain` | `fn differential_gain(r_f: f64, r_in: f64) → f64` | $A_d = R_f/R_{in}$ | `physics::electronics::amplifiers` |
| `summing_amplifier` | `fn summing_amplifier(v_inputs: &[f64], r_inputs: &[f64], r_f: f64) → f64` | Inverting summing amplifier output | `physics::electronics::amplifiers` |
| `integrator_output` | `fn integrator_output(v_in: f64, r: f64, c: f64, t: f64) → f64` | $V_{out} = -\frac{1}{RC}\int V_{in}dt$ | `physics::electronics::amplifiers` |
| `differentiator_output` | `fn differentiator_output(dv_dt: f64, r: f64, c: f64) → f64` | $V_{out} = -RC\frac{dV_{in}}{dt}$ | `physics::electronics::amplifiers` |
| `gain_bandwidth_product` | `fn gain_bandwidth_product(gain: f64, bandwidth: f64) → f64` | $GBW = A \cdot BW$ | `physics::electronics::amplifiers` |
| `common_emitter_voltage_gain` | `fn common_emitter_voltage_gain(gm: f64, r_c: f64) → f64` | $A_v = -g_m R_C$ | `physics::electronics::amplifiers` |
| `transconductance` | `fn transconductance(i_c: f64, v_t: f64) → f64` | $g_m = I_C/V_T$ | `physics::electronics::amplifiers` |
| `thermal_voltage` | `fn thermal_voltage(temperature_k: f64) → f64` | $V_T = k_BT/q$ | `physics::electronics::amplifiers` |
| `decibel_voltage` | `fn decibel_voltage(v_out: f64, v_in: f64) → f64` | $dB = 20\log_{10}(V_{out}/V_{in})$ | `physics::electronics::amplifiers` |
| `decibel_power` | `fn decibel_power(p_out: f64, p_in: f64) → f64` | $dB = 10\log_{10}(P_{out}/P_{in})$ | `physics::electronics::amplifiers` |
| `cascaded_gain` | `fn cascaded_gain(gains_db: &[f64]) → f64` | $A_{total} = \sum A_i$ (dB) | `physics::electronics::amplifiers` |
| `noise_figure` | `fn noise_figure(snr_in: f64, snr_out: f64) → f64` | $NF = SNR_{in}/SNR_{out}$ | `physics::electronics::amplifiers` |
| `friis_noise_factor` | `fn friis_noise_factor(factors: &[f64], gains: &[f64]) → f64` | Friis cascaded noise formula | `physics::electronics::amplifiers` |

### 4️⃣ electrodynamics/ — Fields, Waves & Circuits

#### fields.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `electric_field_point_charge` | `fn electric_field_point_charge(q: f64, r: [f64; 3]) → [f64; 3]` | $\vec{E} = \frac{q}{4\pi\varepsilon_0 r^2}\hat{r}$ | `physics::electrodynamics::fields` |
| `electric_potential_point` | `fn electric_potential_point(q: f64, r: f64) → f64` | $V = \frac{q}{4\pi\varepsilon_0 r}$ | `physics::electrodynamics::fields` |
| `magnetic_field_wire` | `fn magnetic_field_wire(current: f64, r: f64) → f64` | $B = \frac{\mu_0 I}{2\pi r}$ | `physics::electrodynamics::fields` |
| `magnetic_field_solenoid` | `fn magnetic_field_solenoid(n_per_length: f64, current: f64) → f64` | $B = \mu_0 nI$ | `physics::electrodynamics::fields` |
| `magnetic_field_loop` | `fn magnetic_field_loop(current: f64, radius: f64, z: f64) → f64` | On-axis field of circular loop | `physics::electrodynamics::fields` |
| `biot_savart_segment` | `fn biot_savart_segment(current: f64, dl: [f64; 3], r: [f64; 3]) → [f64; 3]` | $d\vec{B} = \frac{\mu_0 I}{4\pi}\frac{d\vec{l}\times\hat{r}}{r^2}$ | `physics::electrodynamics::fields` |
| `lorentz_force` | `fn lorentz_force(q: f64, v: [f64; 3], e: [f64; 3], b: [f64; 3]) → [f64; 3]` | $\vec{F} = q(\vec{E} + \vec{v}\times\vec{B})$ | `physics::electrodynamics::fields` |
| `poynting_vector` | `fn poynting_vector(e: [f64; 3], b: [f64; 3]) → [f64; 3]` | $\vec{S} = \frac{1}{\mu_0}\vec{E}\times\vec{B}$ | `physics::electrodynamics::fields` |
| `energy_density_em` | `fn energy_density_em(e: [f64; 3], b: [f64; 3]) → f64` | $u = \frac{\varepsilon_0 E^2}{2} + \frac{B^2}{2\mu_0}$ | `physics::electrodynamics::fields` |
| `electric_dipole_field` | `fn electric_dipole_field(p: [f64; 3], r: [f64; 3]) → [f64; 3]` | Electric dipole far field | `physics::electrodynamics::fields` |
| `magnetic_dipole_field` | `fn magnetic_dipole_field(m: [f64; 3], r: [f64; 3]) → [f64; 3]` | Magnetic dipole far field | `physics::electrodynamics::fields` |
| `capacitance_parallel_plate` | `fn capacitance_parallel_plate(area: f64, distance: f64, epsilon_r: f64) → f64` | $C = \varepsilon_0\varepsilon_r A/d$ | `physics::electrodynamics::fields` |
| `inductance_solenoid` | `fn inductance_solenoid(n_turns: f64, length: f64, area: f64) → f64` | $L = \mu_0 N^2 A/l$ | `physics::electrodynamics::fields` |
| `cyclotron_frequency` | `fn cyclotron_frequency(charge: f64, mass: f64, b: f64) → f64` | $\omega_c = qB/m$ | `physics::electrodynamics::fields` |
| `larmor_radius` | `fn larmor_radius(mass: f64, v_perp: f64, charge: f64, b: f64) → f64` | $r_L = mv_\perp/qB$ | `physics::electrodynamics::fields` |
| `plasma_frequency` | `fn plasma_frequency(number_density: f64, mass: f64, charge: f64) → f64` | $\omega_p = \sqrt{nq^2/m\varepsilon_0}$ | `physics::electrodynamics::fields` |
| `debye_length` | `fn debye_length(temperature: f64, number_density: f64, charge: f64) → f64` | $\lambda_D = \sqrt{\varepsilon_0 k_BT/nq^2}$ | `physics::electrodynamics::fields` |

#### waves.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `wave_impedance_free_space` | `fn wave_impedance_free_space() → f64` | $Z_0 = \sqrt{\mu_0/\varepsilon_0} \approx 377\Omega$ | `physics::electrodynamics::waves` |
| `wave_number` | `fn wave_number(frequency: f64) → f64` | $k = \omega/c$ | `physics::electrodynamics::waves` |
| `wavelength` | `fn wavelength(frequency: f64) → f64` | $\lambda = c/f$ | `physics::electrodynamics::waves` |
| `phase_velocity` | `fn phase_velocity(epsilon_r: f64, mu_r: f64) → f64` | $v_p = c/\sqrt{\varepsilon_r\mu_r}$ | `physics::electrodynamics::waves` |
| `group_velocity_dispersive` | `fn group_velocity_dispersive(v_phase: f64, omega: f64, dv_domega: f64) → f64` | $v_g = v_p + \omega\frac{dv_p}{d\omega}$ | `physics::electrodynamics::waves` |
| `skin_depth` | `fn skin_depth(frequency: f64, conductivity: f64, mu_r: f64) → f64` | $\delta = \sqrt{\frac{2}{\omega\mu\sigma}}$ | `physics::electrodynamics::waves` |
| `fresnel_rs` | `fn fresnel_rs(n1: f64, n2: f64, theta_i: f64) → f64` | Fresnel $r_s$ | `physics::electrodynamics::waves` |
| `fresnel_rp` | `fn fresnel_rp(n1: f64, n2: f64, theta_i: f64) → f64` | Fresnel $r_p$ | `physics::electrodynamics::waves` |
| `brewster_angle` | `fn brewster_angle(n1: f64, n2: f64) → f64` | $\theta_B = \arctan(n_2/n_1)$ | `physics::electrodynamics::waves` |
| `critical_angle` | `fn critical_angle(n1: f64, n2: f64) → Option<f64>` | $\theta_c = \arcsin(n_2/n_1)$ | `physics::electrodynamics::waves` |
| `snell` | `fn snell(n1: f64, theta1: f64, n2: f64) → f64` | $n_1\sin\theta_1 = n_2\sin\theta_2$ | `physics::electrodynamics::waves` |
| `radiation_pressure_absorbed` | `fn radiation_pressure_absorbed(intensity: f64) → f64` | $P = I/c$ | `physics::electrodynamics::waves` |
| `radiation_pressure_reflected` | `fn radiation_pressure_reflected(intensity: f64) → f64` | $P = 2I/c$ | `physics::electrodynamics::waves` |
| `larmor_radiation_power` | `fn larmor_radiation_power(charge: f64, accel: f64) → f64` | $P = \frac{q^2a^2}{6\pi\varepsilon_0 c^3}$ | `physics::electrodynamics::waves` |
| `antenna_radiation_resistance_dipole` | `fn antenna_radiation_resistance_dipole(length: f64, wavelength: f64) → f64` | Radiation resistance of dipole | `physics::electrodynamics::waves` |
| `fdtd_1d` | `fn fdtd_1d(ez: &mut [f64], hy: &mut [f64], steps: usize)` | 1D FDTD electromagnetic simulation | `physics::electrodynamics::waves` |
| `waveguide_cutoff_te` | `fn waveguide_cutoff_te(m: u32, n: u32, a: f64, b: f64) → f64` | $f_c = \frac{c}{2}\sqrt{(m/a)^2+(n/b)^2}$ | `physics::electrodynamics::waves` |

#### circuits.rs (24 pub fn + 9 struct methods)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `RlcCircuit::new` | `fn new(r: f64, l: f64, c: f64) → Self` | RLC circuit constructor | `physics::electrodynamics::circuits` |
| `resonant_frequency` | `fn resonant_frequency(&self) → f64` | $f_0 = \frac{1}{2\pi\sqrt{LC}}$ | `physics::electrodynamics::circuits` |
| `quality_factor` | `fn quality_factor(&self) → f64` | $Q = \frac{1}{R}\sqrt{L/C}$ | `physics::electrodynamics::circuits` |
| `impedance` | `fn impedance(&self, omega: f64) → (f64, f64)` | Complex impedance $(R, X)$ | `physics::electrodynamics::circuits` |
| `impedance_magnitude` | `fn impedance_magnitude(&self, omega: f64) → f64` | $|Z|$ | `physics::electrodynamics::circuits` |
| `phase_angle` | `fn phase_angle(&self, omega: f64) → f64` | $\phi = \arctan(X/R)$ | `physics::electrodynamics::circuits` |
| `damping_ratio` | `fn damping_ratio(&self) → f64` | $\zeta = \frac{R}{2}\sqrt{C/L}$ | `physics::electrodynamics::circuits` |
| `bandwidth` | `fn bandwidth(&self) → f64` | $BW = R/L$ | `physics::electrodynamics::circuits` |
| `transient_response` | `fn transient_response(&self, t: f64, v0: f64) → f64` | RLC transient response | `physics::electrodynamics::circuits` |
| `rc_time_constant` | `fn rc_time_constant(r: f64, c: f64) → f64` | $\tau = RC$ | `physics::electrodynamics::circuits` |
| `rl_time_constant` | `fn rl_time_constant(r: f64, l: f64) → f64` | $\tau = L/R$ | `physics::electrodynamics::circuits` |
| `rc_charging` | `fn rc_charging(v_source: f64, r: f64, c: f64, t: f64) → f64` | RC charging | `physics::electrodynamics::circuits` |
| `rc_discharging` | `fn rc_discharging(v0: f64, r: f64, c: f64, t: f64) → f64` | RC discharging | `physics::electrodynamics::circuits` |
| `power_dissipated` | `fn power_dissipated(v: f64, r: f64) → f64` | $P = V^2/R$ | `physics::electrodynamics::circuits` |
| `parallel_resistance` | `fn parallel_resistance(resistances: &[f64]) → f64` | Parallel resistance | `physics::electrodynamics::circuits` |
| `series_resistance` | `fn series_resistance(resistances: &[f64]) → f64` | Series resistance | `physics::electrodynamics::circuits` |
| `parallel_capacitance` | `fn parallel_capacitance(capacitances: &[f64]) → f64` | $C_{total} = \sum C_i$ | `physics::electrodynamics::circuits` |
| `series_capacitance` | `fn series_capacitance(capacitances: &[f64]) → f64` | $\frac{1}{C} = \sum\frac{1}{C_i}$ | `physics::electrodynamics::circuits` |
| `voltage_divider` | `fn voltage_divider(v_in: f64, r1: f64, r2: f64) → f64` | Voltage divider | `physics::electrodynamics::circuits` |
| `wheatstone_bridge_balance` | `fn wheatstone_bridge_balance(r1: f64, r2: f64, r3: f64) → f64` | $R_x = R_3 R_2/R_1$ | `physics::electrodynamics::circuits` |
| `energy_capacitor` | `fn energy_capacitor(c: f64, v: f64) → f64` | $E = \frac{1}{2}CV^2$ | `physics::electrodynamics::circuits` |
| `energy_inductor` | `fn energy_inductor(l: f64, i: f64) → f64` | $E = \frac{1}{2}LI^2$ | `physics::electrodynamics::circuits` |
| `mutual_inductance_coupling` | `fn mutual_inductance_coupling(k: f64, l1: f64, l2: f64) → f64` | $M = k\sqrt{L_1 L_2}$ | `physics::electrodynamics::circuits` |
| `transformer_ratio` | `fn transformer_ratio(n_primary: f64, n_secondary: f64, v_primary: f64) → f64` | $V_s = V_p \cdot N_s/N_p$ | `physics::electrodynamics::circuits` |

### 5️⃣ fluid_mechanics/ — Fluid Dynamics

#### flow.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `reynolds_number` | `fn reynolds_number(rho: f64, v: f64, l: f64, mu: f64) → f64` | $Re = \rho vL/\mu$ | `physics::fluid_mechanics::flow` |
| `bernoulli_pressure` | `fn bernoulli_pressure(rho: f64, v1: f64, p1: f64, v2: f64) → f64` | $P + \frac{1}{2}\rho v^2 = \text{const}$ | `physics::fluid_mechanics::flow` |
| `bernoulli_height` | `fn bernoulli_height(rho: f64, v1: f64, p1: f64, h1: f64, v2: f64, p2: f64, g: f64) → f64` | Bernoulli with height term | `physics::fluid_mechanics::flow` |
| `hagen_poiseuille` | `fn hagen_poiseuille(delta_p: f64, r: f64, l: f64, mu: f64) → f64` | $Q = \frac{\pi r^4 \Delta P}{8\mu L}$ | `physics::fluid_mechanics::flow` |
| `continuity_velocity` | `fn continuity_velocity(a1: f64, v1: f64, a2: f64) → f64` | $A_1 v_1 = A_2 v_2$ | `physics::fluid_mechanics::flow` |
| `drag_force` | `fn drag_force(cd: f64, rho: f64, v: f64, a: f64) → f64` | $F_D = \frac{1}{2}C_D\rho v^2 A$ | `physics::fluid_mechanics::flow` |
| `lift_force` | `fn lift_force(cl: f64, rho: f64, v: f64, a: f64) → f64` | $F_L = \frac{1}{2}C_L\rho v^2 A$ | `physics::fluid_mechanics::flow` |
| `stokes_drag` | `fn stokes_drag(mu: f64, r: f64, v: f64) → f64` | $F = 6\pi\mu rv$ | `physics::fluid_mechanics::flow` |
| `terminal_velocity_sphere` | `fn terminal_velocity_sphere(rho_p: f64, rho_f: f64, r: f64, mu: f64, g: f64) → f64` | $v_t = \frac{2r^2(\rho_p-\rho_f)g}{9\mu}$ | `physics::fluid_mechanics::flow` |
| `torricelli` | `fn torricelli(g: f64, h: f64) → f64` | $v = \sqrt{2gh}$ | `physics::fluid_mechanics::flow` |
| `hydraulic_diameter` | `fn hydraulic_diameter(area: f64, perimeter: f64) → f64` | $D_h = 4A/P$ | `physics::fluid_mechanics::flow` |
| `darcy_weisbach` | `fn darcy_weisbach(f: f64, l: f64, d: f64, rho: f64, v: f64) → f64` | $\Delta P = f\frac{L}{D}\frac{\rho v^2}{2}$ | `physics::fluid_mechanics::flow` |

#### turbulence.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `turbulent_kinetic_energy` | `fn turbulent_kinetic_energy(u_prime: f64, v_prime: f64, w_prime: f64) → f64` | $k = \frac{1}{2}(u'^2+v'^2+w'^2)$ | `physics::fluid_mechanics::turbulence` |
| `kolmogorov_length_scale` | `fn kolmogorov_length_scale(nu: f64, epsilon: f64) → f64` | $\eta = (\nu^3/\varepsilon)^{1/4}$ | `physics::fluid_mechanics::turbulence` |
| `kolmogorov_time_scale` | `fn kolmogorov_time_scale(nu: f64, epsilon: f64) → f64` | $\tau_\eta = (\nu/\varepsilon)^{1/2}$ | `physics::fluid_mechanics::turbulence` |
| `kolmogorov_velocity_scale` | `fn kolmogorov_velocity_scale(nu: f64, epsilon: f64) → f64` | $v_\eta = (\nu\varepsilon)^{1/4}$ | `physics::fluid_mechanics::turbulence` |
| `taylor_microscale` | `fn taylor_microscale(u_rms: f64, epsilon: f64, nu: f64) → f64` | $\lambda = u_{rms}\sqrt{15\nu/\varepsilon}$ | `physics::fluid_mechanics::turbulence` |
| `integral_length_scale` | `fn integral_length_scale(tke: f64, epsilon: f64) → f64` | $L = k^{3/2}/\varepsilon$ | `physics::fluid_mechanics::turbulence` |
| `friction_velocity` | `fn friction_velocity(tau_wall: f64, rho: f64) → f64` | $u_\tau = \sqrt{\tau_w/\rho}$ | `physics::fluid_mechanics::turbulence` |
| `law_of_wall` | `fn law_of_wall(u_tau: f64, y: f64, nu: f64) → f64` | $u^+ = y^+$ (viscous sublayer) | `physics::fluid_mechanics::turbulence` |
| `mixing_length` | `fn mixing_length(kappa: f64, y: f64) → f64` | $l_m = \kappa y$ (Prandtl) | `physics::fluid_mechanics::turbulence` |
| `eddy_viscosity` | `fn eddy_viscosity(mixing_length: f64, du_dy: f64) → f64` | $\nu_t = l_m^2|du/dy|$ | `physics::fluid_mechanics::turbulence` |
| `turbulence_intensity` | `fn turbulence_intensity(u_rms: f64, u_mean: f64) → f64` | $I = u_{rms}/U$ | `physics::fluid_mechanics::turbulence` |
| `energy_spectrum_kolmogorov` | `fn energy_spectrum_kolmogorov(c_k: f64, epsilon: f64, k: f64) → f64` | $E(k) = C_K\varepsilon^{2/3}k^{-5/3}$ | `physics::fluid_mechanics::turbulence` |

#### boundary_layer.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `blasius_thickness` | `fn blasius_thickness(x: f64, re_x: f64) → f64` | $\delta = 5x/\sqrt{Re_x}$ | `physics::fluid_mechanics::boundary_layer` |
| `displacement_thickness` | `fn displacement_thickness(x: f64, re_x: f64) → f64` | $\delta^* = 1.72x/\sqrt{Re_x}$ | `physics::fluid_mechanics::boundary_layer` |
| `momentum_thickness` | `fn momentum_thickness(x: f64, re_x: f64) → f64` | $\theta = 0.664x/\sqrt{Re_x}$ | `physics::fluid_mechanics::boundary_layer` |
| `shape_factor` | `fn shape_factor(displacement: f64, momentum: f64) → f64` | $H = \delta^*/\theta$ | `physics::fluid_mechanics::boundary_layer` |
| `skin_friction_laminar` | `fn skin_friction_laminar(re_x: f64) → f64` | $C_f = 0.664/\sqrt{Re_x}$ | `physics::fluid_mechanics::boundary_layer` |
| `skin_friction_turbulent` | `fn skin_friction_turbulent(re_x: f64) → f64` | $C_f = 0.027/Re_x^{1/7}$ | `physics::fluid_mechanics::boundary_layer` |
| `turbulent_bl_thickness` | `fn turbulent_bl_thickness(x: f64, re_x: f64) → f64` | $\delta = 0.37x/Re_x^{1/5}$ | `physics::fluid_mechanics::boundary_layer` |
| `separation_criterion` | `fn separation_criterion(dp_dx: f64) → bool` | Adverse pressure gradient criterion | `physics::fluid_mechanics::boundary_layer` |
| `falkner_skan_beta` | `fn falkner_skan_beta(m: f64) → f64` | $\beta = 2m/(m+1)$ | `physics::fluid_mechanics::boundary_layer` |
| `thermal_bl_thickness` | `fn thermal_bl_thickness(delta: f64, pr: f64) → f64` | $\delta_T = \delta/Pr^{1/3}$ | `physics::fluid_mechanics::boundary_layer` |
| `nusselt_flat_plate_laminar` | `fn nusselt_flat_plate_laminar(re: f64, pr: f64) → f64` | $Nu = 0.332 Re^{1/2} Pr^{1/3}$ | `physics::fluid_mechanics::boundary_layer` |
| `stanton_number` | `fn stanton_number(nu: f64, re: f64, pr: f64) → f64` | $St = Nu/(Re \cdot Pr)$ | `physics::fluid_mechanics::boundary_layer` |

#### waves.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `shallow_water_speed` | `fn shallow_water_speed(g: f64, depth: f64) → f64` | $c = \sqrt{gd}$ | `physics::fluid_mechanics::waves` |
| `deep_water_speed` | `fn deep_water_speed(g: f64, wavelength: f64) → f64` | $c = \sqrt{g\lambda/2\pi}$ | `physics::fluid_mechanics::waves` |
| `wave_number` | `fn wave_number(wavelength: f64) → f64` | $k = 2\pi/\lambda$ | `physics::fluid_mechanics::waves` |
| `wave_frequency` | `fn wave_frequency(period: f64) → f64` | $f = 1/T$ | `physics::fluid_mechanics::waves` |
| `froude_number` | `fn froude_number(v: f64, g: f64, depth: f64) → f64` | $Fr = v/\sqrt{gd}$ | `physics::fluid_mechanics::waves` |
| `mach_number` | `fn mach_number(v: f64, c: f64) → f64` | $Ma = v/c$ | `physics::fluid_mechanics::waves` |
| `sound_speed_ideal_gas` | `fn sound_speed_ideal_gas(gamma: f64, r: f64, t: f64, m: f64) → f64` | $c = \sqrt{\gamma RT/M}$ | `physics::fluid_mechanics::waves` |
| `water_hammer_pressure` | `fn water_hammer_pressure(rho: f64, c: f64, delta_v: f64) → f64` | $\Delta P = \rho c \Delta v$ | `physics::fluid_mechanics::waves` |
| `capillary_number` | `fn capillary_number(mu: f64, v: f64, sigma: f64) → f64` | $Ca = \mu v/\sigma$ | `physics::fluid_mechanics::waves` |
| `weber_number` | `fn weber_number(rho: f64, v: f64, l: f64, sigma: f64) → f64` | $We = \rho v^2 L/\sigma$ | `physics::fluid_mechanics::waves` |
| `wave_energy_density` | `fn wave_energy_density(rho: f64, g: f64, amplitude: f64) → f64` | $E = \frac{1}{2}\rho g a^2$ | `physics::fluid_mechanics::waves` |

### 6️⃣ solid_mechanics/ — Stress, Elasticity, Plasticity & Fracture

#### stress.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `principal_stresses_2d` | `fn principal_stresses_2d(sx: f64, sy: f64, txy: f64) → (f64, f64)` | $\sigma_{1,2} = \frac{\sigma_x+\sigma_y}{2} \pm \sqrt{\left(\frac{\sigma_x-\sigma_y}{2}\right)^2+\tau_{xy}^2}$ | `physics::solid_mechanics::stress` |
| `max_shear_stress_2d` | `fn max_shear_stress_2d(sx: f64, sy: f64, txy: f64) → f64` | $\tau_{max} = \frac{\sigma_1-\sigma_2}{2}$ | `physics::solid_mechanics::stress` |
| `mohr_circle_radius` | `fn mohr_circle_radius(sx: f64, sy: f64, txy: f64) → f64` | Mohr circle radius | `physics::solid_mechanics::stress` |
| `mohr_circle_center` | `fn mohr_circle_center(sx: f64, sy: f64) → f64` | $C = (\sigma_x+\sigma_y)/2$ | `physics::solid_mechanics::stress` |
| `stress_rotation_2d` | `fn stress_rotation_2d(sx: f64, sy: f64, txy: f64, theta: f64) → (f64, f64, f64)` | 2D stress rotation | `physics::solid_mechanics::stress` |
| `deviatoric_stress` | `fn deviatoric_stress(sx: f64, sy: f64, sz: f64) → (f64, f64, f64)` | $s_i = \sigma_i - \sigma_m$ | `physics::solid_mechanics::stress` |
| `stress_invariant_j2` | `fn stress_invariant_j2(s1: f64, s2: f64, s3: f64) → f64` | $J_2$ stress invariant | `physics::solid_mechanics::stress` |
| `beam_bending_stress` | `fn beam_bending_stress(m: f64, y: f64, i: f64) → f64` | $\sigma = My/I$ | `physics::solid_mechanics::stress` |
| `beam_deflection_cantilever` | `fn beam_deflection_cantilever(p: f64, l: f64, e: f64, i: f64) → f64` | $\delta = PL^3/(3EI)$ | `physics::solid_mechanics::stress` |
| `torsion_shear_stress` | `fn torsion_shear_stress(t: f64, r: f64, j: f64) → f64` | $\tau = Tr/J$ | `physics::solid_mechanics::stress` |
| `column_euler_buckling` | `fn column_euler_buckling(e: f64, i: f64, l: f64) → f64` | $P_{cr} = \frac{\pi^2 EI}{L^2}$ | `physics::solid_mechanics::stress` |
| `hertz_contact_pressure` | `fn hertz_contact_pressure(force: f64, r1: f64, r2: f64, e_star: f64) → f64` | Hertzian contact pressure | `physics::solid_mechanics::stress` |

#### elasticity.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hooke_stress` | `fn hooke_stress(e: f64, strain: f64) → f64` | $\sigma = E\varepsilon$ | `physics::solid_mechanics::elasticity` |
| `hooke_strain` | `fn hooke_strain(stress: f64, e: f64) → f64` | $\varepsilon = \sigma/E$ | `physics::solid_mechanics::elasticity` |
| `poisson_lateral_strain` | `fn poisson_lateral_strain(axial_strain: f64, nu: f64) → f64` | $\varepsilon_{lat} = -\nu\varepsilon_{axial}$ | `physics::solid_mechanics::elasticity` |
| `shear_modulus` | `fn shear_modulus(e: f64, nu: f64) → f64` | $G = E/[2(1+\nu)]$ | `physics::solid_mechanics::elasticity` |
| `bulk_modulus` | `fn bulk_modulus(e: f64, nu: f64) → f64` | $K = E/[3(1-2\nu)]$ | `physics::solid_mechanics::elasticity` |
| `lame_first` | `fn lame_first(e: f64, nu: f64) → f64` | $\lambda = \frac{E\nu}{(1+\nu)(1-2\nu)}$ | `physics::solid_mechanics::elasticity` |
| `plane_stress_strain` | `fn plane_stress_strain(stress_x: f64, stress_y: f64, e: f64, nu: f64) → (f64, f64)` | Plane stress → strain conversion | `physics::solid_mechanics::elasticity` |
| `strain_energy_density` | `fn strain_energy_density(stress: f64, strain: f64) → f64` | $u = \frac{1}{2}\sigma\varepsilon$ | `physics::solid_mechanics::elasticity` |
| `thermal_strain` | `fn thermal_strain(alpha: f64, delta_t: f64) → f64` | $\varepsilon_T = \alpha\Delta T$ | `physics::solid_mechanics::elasticity` |
| `thermal_stress` | `fn thermal_stress(e: f64, alpha: f64, delta_t: f64) → f64` | $\sigma_T = E\alpha\Delta T$ | `physics::solid_mechanics::elasticity` |
| `volumetric_strain` | `fn volumetric_strain(ex: f64, ey: f64, ez: f64) → f64` | $\varepsilon_v = \varepsilon_x+\varepsilon_y+\varepsilon_z$ | `physics::solid_mechanics::elasticity` |
| `hydrostatic_stress` | `fn hydrostatic_stress(sx: f64, sy: f64, sz: f64) → f64` | $\sigma_m = (\sigma_x+\sigma_y+\sigma_z)/3$ | `physics::solid_mechanics::elasticity` |

#### plasticity.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `yield_offset_strain` | `fn yield_offset_strain(total_strain: f64, offset: f64) → f64` | 0.2% offset yield criterion | `physics::solid_mechanics::plasticity` |
| `ramberg_osgood` | `fn ramberg_osgood(stress: f64, e: f64, k: f64, n: f64) → f64` | $\varepsilon = \sigma/E + (\sigma/K)^{1/n}$ | `physics::solid_mechanics::plasticity` |
| `true_stress` | `fn true_stress(engineering_stress: f64, engineering_strain: f64) → f64` | $\sigma_T = \sigma_E(1+\varepsilon_E)$ | `physics::solid_mechanics::plasticity` |
| `true_strain` | `fn true_strain(engineering_strain: f64) → f64` | $\varepsilon_T = \ln(1+\varepsilon_E)$ | `physics::solid_mechanics::plasticity` |
| `hardening_power_law` | `fn hardening_power_law(k: f64, strain_plastic: f64, n: f64) → f64` | $\sigma = K\varepsilon_p^n$ (Hollomon) | `physics::solid_mechanics::plasticity` |
| `von_mises` | `fn von_mises(s1: f64, s2: f64, s3: f64) → f64` | $\sigma_{VM} = \sqrt{\frac{(s_1-s_2)^2+(s_2-s_3)^2+(s_3-s_1)^2}{2}}$ | `physics::solid_mechanics::plasticity` |
| `tresca` | `fn tresca(s1: f64, s2: f64, s3: f64) → f64` | $\sigma_T = \max(|s_1-s_2|, |s_2-s_3|, |s_3-s_1|)$ | `physics::solid_mechanics::plasticity` |
| `isotropic_hardening` | `fn isotropic_hardening(yield_0: f64, h: f64, plastic_strain: f64) → f64` | $\sigma_y = \sigma_{y0} + H\varepsilon_p$ | `physics::solid_mechanics::plasticity` |
| `bauschinger_effect` | `fn bauschinger_effect(forward_yield: f64, reverse_yield: f64) → f64` | Bauschinger stress reduction | `physics::solid_mechanics::plasticity` |
| `plastic_work` | `fn plastic_work(stress: &[f64], d_plastic_strain: &[f64]) → f64` | $W_p = \int\sigma\,d\varepsilon_p$ | `physics::solid_mechanics::plasticity` |
| `necking_criterion` | `fn necking_criterion(n: f64) → f64` | Necking criterion $\varepsilon = n$ | `physics::solid_mechanics::plasticity` |
| `strain_rate_sensitivity` | `fn strain_rate_sensitivity(stress1: f64, stress2: f64, rate1: f64, rate2: f64) → f64` | $m = \frac{\ln(\sigma_2/\sigma_1)}{\ln(\dot\varepsilon_2/\dot\varepsilon_1)}$ | `physics::solid_mechanics::plasticity` |

#### fracture.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `stress_intensity_factor` | `fn stress_intensity_factor(sigma: f64, a: f64, geometry_factor: f64) → f64` | $K = Y\sigma\sqrt{\pi a}$ | `physics::solid_mechanics::fracture` |
| `griffith_critical_stress` | `fn griffith_critical_stress(e: f64, gamma: f64, a: f64) → f64` | $\sigma_c = \sqrt{\frac{2E\gamma}{\pi a}}$ | `physics::solid_mechanics::fracture` |
| `energy_release_rate` | `fn energy_release_rate(k: f64, e: f64) → f64` | $G = K^2/E$ | `physics::solid_mechanics::fracture` |
| `j_integral` | `fn j_integral(energy_release: f64) → f64` | $J$-integral | `physics::solid_mechanics::fracture` |
| `crack_tip_plastic_zone` | `fn crack_tip_plastic_zone(k: f64, sigma_y: f64) → f64` | $r_p = \frac{1}{2\pi}\left(\frac{K}{\sigma_y}\right)^2$ | `physics::solid_mechanics::fracture` |
| `paris_law` | `fn paris_law(c: f64, delta_k: f64, m: f64) → f64` | $\frac{da}{dN} = C(\Delta K)^m$ | `physics::solid_mechanics::fracture` |
| `fatigue_life_basquin` | `fn fatigue_life_basquin(sigma_f: f64, sigma_a: f64, b: f64) → f64` | $\sigma_a = \sigma_f'(2N_f)^b$ (Basquin) | `physics::solid_mechanics::fracture` |
| `fatigue_life_coffin_manson` | `fn fatigue_life_coffin_manson(ef: f64, ea: f64, c: f64) → f64` | Coffin-Manson low-cycle fatigue | `physics::solid_mechanics::fracture` |
| `miners_rule` | `fn miners_rule(cycles: &[f64], max_cycles: &[f64]) → f64` | $D = \sum n_i/N_i$ (Miner) | `physics::solid_mechanics::fracture` |
| `fracture_toughness_plane_strain` | `fn fracture_toughness_plane_strain(kic: f64, sigma_y: f64) → f64` | Plane strain fracture toughness | `physics::solid_mechanics::fracture` |
| `stress_corrosion_threshold` | `fn stress_corrosion_threshold(k_iscc: f64, sigma: f64, a: f64) → bool` | $K_{ISCC}$ stress corrosion criterion | `physics::solid_mechanics::fracture` |
| `crack_opening_displacement` | `fn crack_opening_displacement(k: f64, sigma_y: f64, e: f64) → f64` | $CTOD = K^2/(E\sigma_y)$ | `physics::solid_mechanics::fracture` |

### 7️⃣ nucleosynthesis/ — Nuclear Physics & Stellar Evolution

#### nuclide.rs (24 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `Nuclide::n` | `fn n(&self) → u32` | Neutron number $N = A - Z$ | `physics::nucleosynthesis::nuclide` |
| `Nuclide::atomic_mass_amu` | `fn atomic_mass_amu(&self) → f64` | Atomic mass in AMU | `physics::nucleosynthesis::nuclide` |
| `Nuclide::binding_energy` | `fn binding_energy(&self) → f64` | Binding energy (SEMF) | `physics::nucleosynthesis::nuclide` |
| `Nuclide::is_stable` | `fn is_stable(&self) → bool` | Stability check | `physics::nucleosynthesis::nuclide` |
| `Nuclide::decay_constant` | `fn decay_constant(&self) → Option<f64>` | $\lambda = \ln 2/t_{1/2}$ | `physics::nucleosynthesis::nuclide` |
| `Nuclide::activity_bq` | `fn activity_bq(&self, num_atoms: f64) → Option<f64>` | $A = \lambda N$ | `physics::nucleosynthesis::nuclide` |
| `semi_empirical_mass` | `fn semi_empirical_mass(z: u32, a: u32) → f64` | Weizsäcker semi-empirical mass formula | `physics::nucleosynthesis::nuclide` |
| `binding_energy_per_nucleon_semf` | `fn binding_energy_per_nucleon_semf(z: u32, a: u32) → f64` | $B/A$ from SEMF | `physics::nucleosynthesis::nuclide` |
| `nuclear_radius_fm` | `fn nuclear_radius_fm(a: u32) → f64` | $R = r_0 A^{1/3}$ | `physics::nucleosynthesis::nuclide` |
| `nuclear_density_kg_m3` | `fn nuclear_density_kg_m3() → f64` | $\rho_{nuc} \approx 2.3\times10^{17}$ kg/m³ | `physics::nucleosynthesis::nuclide` |
| `separation_energy_proton` | `fn separation_energy_proton(z: u32, a: u32) → f64` | Proton separation energy $S_p$ | `physics::nucleosynthesis::nuclide` |
| `separation_energy_neutron` | `fn separation_energy_neutron(z: u32, a: u32) → f64` | Neutron separation energy $S_n$ | `physics::nucleosynthesis::nuclide` |
| `separation_energy_alpha` | `fn separation_energy_alpha(z: u32, a: u32) → f64` | Alpha separation energy $S_\alpha$ | `physics::nucleosynthesis::nuclide` |
| `valley_of_stability_z` | `fn valley_of_stability_z(a: u32) → f64` | Most stable $Z$ for given $A$ | `physics::nucleosynthesis::nuclide` |
| `neutron_excess` | `fn neutron_excess(z: u32, a: u32) → i32` | $N - Z$ | `physics::nucleosynthesis::nuclide` |
| `isospin_z` | `fn isospin_z(z: u32, a: u32) → f64` | $T_z = (N-Z)/2$ | `physics::nucleosynthesis::nuclide` |
| `liquid_drop_fission_parameter` | `fn liquid_drop_fission_parameter(z: u32, a: u32) → f64` | $Z^2/A$ fissility parameter | `physics::nucleosynthesis::nuclide` |
| `fission_barrier_estimate_mev` | `fn fission_barrier_estimate_mev(z: u32, a: u32) → f64` | Fission barrier estimate | `physics::nucleosynthesis::nuclide` |
| `nuclear_skin_thickness_fm` | `fn nuclear_skin_thickness_fm(z: u32, a: u32) → f64` | Neutron skin thickness | `physics::nucleosynthesis::nuclide` |
| `weizsacker_mass_excess_mev` | `fn weizsacker_mass_excess_mev(z: u32, a: u32) → f64` | Mass excess from SEMF | `physics::nucleosynthesis::nuclide` |
| `proton_drip_line_a` | `fn proton_drip_line_a(z: u32) → u32` | Proton drip line | `physics::nucleosynthesis::nuclide` |
| `neutron_drip_line_a` | `fn neutron_drip_line_a(z: u32) → u32` | Neutron drip line | `physics::nucleosynthesis::nuclide` |
| `magic_number_nearest` | `fn magic_number_nearest(n: u32) → u32` | Nearest magic number | `physics::nucleosynthesis::nuclide` |
| `is_doubly_magic` | `fn is_doubly_magic(z: u32, n: u32) → bool` | Doubly magic nucleus check | `physics::nucleosynthesis::nuclide` |

#### decay.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `decay_remaining` | `fn decay_remaining(n0: f64, half_life: f64, time: f64) → f64` | $N = N_0 \cdot 2^{-t/t_{1/2}}$ | `physics::nucleosynthesis::decay` |
| `bateman_chain` | `fn bateman_chain(n0: f64, lambdas: &[f64], time: f64) → Vec<f64>` | Bateman equations for decay chains | `physics::nucleosynthesis::decay` |
| `specific_activity` | `fn specific_activity(decay_constant: f64, molar_mass: f64) → f64` | $a = \lambda N_A/M$ | `physics::nucleosynthesis::decay` |
| `half_life_from_constant` | `fn half_life_from_constant(decay_constant: f64) → f64` | $t_{1/2} = \ln 2/\lambda$ | `physics::nucleosynthesis::decay` |
| `mean_lifetime` | `fn mean_lifetime(half_life: f64) → f64` | $\tau = t_{1/2}/\ln 2$ | `physics::nucleosynthesis::decay` |
| `decay_constant_from_half_life` | `fn decay_constant_from_half_life(half_life: f64) → f64` | $\lambda = \ln 2/t_{1/2}$ | `physics::nucleosynthesis::decay` |
| `activity_becquerel` | `fn activity_becquerel(n_atoms: f64, half_life: f64) → f64` | $A = \lambda N$ (Bq) | `physics::nucleosynthesis::decay` |
| `activity_curie` | `fn activity_curie(n_atoms: f64, half_life: f64) → f64` | Activity in Curie | `physics::nucleosynthesis::decay` |
| `secular_equilibrium_activity` | `fn secular_equilibrium_activity(parent_activity: f64) → f64` | $A_d = A_p$ (secular equilibrium) | `physics::nucleosynthesis::decay` |
| `transient_equilibrium_ratio` | `fn transient_equilibrium_ratio(lambda_parent: f64, lambda_daughter: f64) → f64` | Transient equilibrium ratio | `physics::nucleosynthesis::decay` |
| `branching_ratio_effective` | `fn branching_ratio_effective(partial_constants: &[f64]) → Vec<f64>` | Branching ratios from partial constants | `physics::nucleosynthesis::decay` |
| `geiger_nuttall` | `fn geiger_nuttall(z: u32, energy_mev: f64) → f64` | $\log t_{1/2} = a/\sqrt{E} + b$ | `physics::nucleosynthesis::decay` |
| `alpha_decay_q` | `fn alpha_decay_q(parent_mass_mev: f64, daughter_mass_mev: f64, alpha_mass_mev: f64) → f64` | $Q_\alpha$ value | `physics::nucleosynthesis::decay` |
| `beta_minus_q` | `fn beta_minus_q(parent_mass_amu: f64, daughter_mass_amu: f64) → f64` | $Q_{\beta^-}$ value | `physics::nucleosynthesis::decay` |
| `beta_plus_q` | `fn beta_plus_q(parent_mass_amu: f64, daughter_mass_amu: f64) → f64` | $Q_{\beta^+}$ value | `physics::nucleosynthesis::decay` |
| `dose_rate_point_source` | `fn dose_rate_point_source(activity_bq: f64, gamma_constant: f64, distance_m: f64) → f64` | $\dot{D} = A\Gamma/r^2$ | `physics::nucleosynthesis::decay` |
| `radioactive_dating_age` | `fn radioactive_dating_age(ratio_daughter_parent: f64, half_life: f64) → f64` | Radiometric dating age | `physics::nucleosynthesis::decay` |
| `decay_chain_equilibrium_time` | `fn decay_chain_equilibrium_time(lambda_parent: f64, lambda_daughter: f64) → f64` | Time to secular equilibrium | `physics::nucleosynthesis::decay` |

#### reactions.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `q_value` | `fn q_value(reactants: &[&Nuclide], products: &[&Nuclide]) → f64` | $Q = (M_r - M_p)c^2$ | `physics::nucleosynthesis::reactions` |
| `coulomb_barrier` | `fn coulomb_barrier(z1: u32, z2: u32, a1: u32, a2: u32) → f64` | $V_C = \frac{Z_1 Z_2 e^2}{4\pi\varepsilon_0(R_1+R_2)}$ | `physics::nucleosynthesis::reactions` |
| `gamow_peak` | `fn gamow_peak(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) → f64` | $E_0 = (bk_BT/2)^{2/3}$ (Gamow peak) | `physics::nucleosynthesis::reactions` |
| `gamow_window_width` | `fn gamow_window_width(z1: u32, z2: u32, reduced_mass_amu: f64, temperature_k: f64) → f64` | $\Delta E_0$ Gamow window | `physics::nucleosynthesis::reactions` |
| `reduced_mass_amu` | `fn reduced_mass_amu(m1: f64, m2: f64) → f64` | $\mu = m_1 m_2/(m_1+m_2)$ | `physics::nucleosynthesis::reactions` |
| `astrophysical_s_factor` | `fn astrophysical_s_factor(cross_section_barn: f64, energy_kev: f64, z1: u32, z2: u32, mu_amu: f64) → f64` | $S(E) = \sigma E e^{2\pi\eta}$ | `physics::nucleosynthesis::reactions` |
| `sommerfeld_parameter` | `fn sommerfeld_parameter(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) → f64` | Sommerfeld parameter $\eta$ | `physics::nucleosynthesis::reactions` |
| `penetration_factor` | `fn penetration_factor(z1: u32, z2: u32, energy_kev: f64, mu_amu: f64) → f64` | $P = e^{-2\pi\eta}$ (Gamow) | `physics::nucleosynthesis::reactions` |
| `thermonuclear_rate` | `fn thermonuclear_rate(s_factor_kev_barn: f64, z1: u32, z2: u32, mu_amu: f64, temperature_k: f64) → f64` | Thermonuclear reaction rate $\langle\sigma v\rangle$ | `physics::nucleosynthesis::reactions` |
| `pp_rate_estimate` | `fn pp_rate_estimate(temperature_k: f64, density_g_cm3: f64, x_h: f64) → f64` | pp chain rate estimate | `physics::nucleosynthesis::reactions` |
| `triple_alpha_rate_estimate` | `fn triple_alpha_rate_estimate(temperature_k: f64, density_g_cm3: f64, y_he: f64) → f64` | Triple-alpha rate estimate | `physics::nucleosynthesis::reactions` |
| `c12_alpha_rate_estimate` | `fn c12_alpha_rate_estimate(temperature_k: f64) → f64` | $^{12}C(\alpha,\gamma)^{16}O$ rate | `physics::nucleosynthesis::reactions` |
| `reaction_mean_free_path` | `fn reaction_mean_free_path(cross_section_barn: f64, number_density_per_cm3: f64) → f64` | $\lambda = 1/(n\sigma)$ | `physics::nucleosynthesis::reactions` |
| `nuclear_reaction_lifetime` | `fn nuclear_reaction_lifetime(cross_section_barn: f64, number_density_per_cm3: f64, velocity_cm_s: f64) → f64` | $\tau = 1/(n\sigma v)$ | `physics::nucleosynthesis::reactions` |
| `maxwell_averaged_velocity` | `fn maxwell_averaged_velocity(temperature_k: f64, mu_amu: f64) → f64` | $\langle v\rangle = \sqrt{8k_BT/\pi\mu}$ | `physics::nucleosynthesis::reactions` |
| `cross_section_barn_to_si` | `fn cross_section_barn_to_si(sigma_barn: f64) → f64` | barn → m² conversion | `physics::nucleosynthesis::reactions` |
| `nuclear_radius_fermi` | `fn nuclear_radius_fermi(a: u32) → f64` | $R = r_0 A^{1/3}$ | `physics::nucleosynthesis::reactions` |
| `nuclear_volume` | `fn nuclear_volume(a: u32) → f64` | $V = \frac{4}{3}\pi R^3$ | `physics::nucleosynthesis::reactions` |
| `q_value_to_joules` | `fn q_value_to_joules(q_mev: f64) → f64` | MeV → J conversion | `physics::nucleosynthesis::reactions` |
| `geometric_cross_section` | `fn geometric_cross_section(a1: u32, a2: u32) → f64` | $\sigma_{geo} = \pi(R_1+R_2)^2$ | `physics::nucleosynthesis::reactions` |

#### stellar.rs (15 free pub fn + struct)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `StellarCore::new` | `fn new(mass_solar: f64, temperature_k: f64, density_kg_m3: f64) → Self` | Stellar core constructor | `physics::nucleosynthesis::stellar` |
| `StellarCore::luminosity_solar` | `fn luminosity_solar(&self) → f64` | Luminosity in solar units | `physics::nucleosynthesis::stellar` |
| `StellarCore::main_sequence_lifetime_years` | `fn main_sequence_lifetime_years(&self) → f64` | MS lifetime estimate | `physics::nucleosynthesis::stellar` |
| `StellarCore::active_processes` | `fn active_processes(&self) → Vec<ProcessType>` | Active nuclear processes | `physics::nucleosynthesis::stellar` |
| `StellarCore::dominant_process` | `fn dominant_process(&self) → Option<ProcessType>` | Dominant nuclear process | `physics::nucleosynthesis::stellar` |
| `StellarCore::evolve_step` | `fn evolve_step(&mut self, dt_years: f64)` | Time evolution step | `physics::nucleosynthesis::stellar` |
| `chandrasekhar_limit` | `fn chandrasekhar_limit() → f64` | $M_{Ch} \approx 1.4 M_\odot$ | `physics::nucleosynthesis::stellar` |
| `tolman_oppenheimer_volkoff_limit` | `fn tolman_oppenheimer_volkoff_limit() → f64` | $M_{TOV} \approx 2.2 M_\odot$ | `physics::nucleosynthesis::stellar` |
| `neutron_drip_density` | `fn neutron_drip_density() → f64` | Neutron drip density | `physics::nucleosynthesis::stellar` |
| `iron_peak_binding_energy` | `fn iron_peak_binding_energy() → f64` | $^{56}$Fe peak binding energy | `physics::nucleosynthesis::stellar` |
| `eddington_luminosity_solar` | `fn eddington_luminosity_solar(mass_solar: f64) → f64` | $L_{Edd} = 3.2\times10^4 (M/M_\odot) L_\odot$ | `physics::nucleosynthesis::stellar` |
| `kelvin_helmholtz_timescale_years` | `fn kelvin_helmholtz_timescale_years(mass_solar: f64, radius_solar: f64, luminosity_solar: f64) → f64` | $t_{KH} = GM^2/(2RL)$ | `physics::nucleosynthesis::stellar` |
| `jeans_mass_solar` | `fn jeans_mass_solar(temperature_k: f64, density_kg_m3: f64) → f64` | $M_J = \left(\frac{5k_BT}{G\mu m_H}\right)^{3/2}\left(\frac{3}{4\pi\rho}\right)^{1/2}$ | `physics::nucleosynthesis::stellar` |
| `schwarzschild_radius_km` | `fn schwarzschild_radius_km(mass_solar: f64) → f64` | Schwarzschild radius in km | `physics::nucleosynthesis::stellar` |
| `nuclear_timescale_years` | `fn nuclear_timescale_years(mass_solar: f64, luminosity_solar: f64, efficiency: f64) → f64` | Nuclear burning timescale | `physics::nucleosynthesis::stellar` |
| `core_collapse_min_mass_solar` | `fn core_collapse_min_mass_solar() → f64` | Minimum mass for core collapse | `physics::nucleosynthesis::stellar` |
| `white_dwarf_radius_km` | `fn white_dwarf_radius_km(mass_solar: f64) → f64` | WD mass-radius relation | `physics::nucleosynthesis::stellar` |
| `electron_degeneracy_pressure` | `fn electron_degeneracy_pressure(density_kg_m3: f64) → f64` | Electron degeneracy pressure | `physics::nucleosynthesis::stellar` |
| `neutron_star_radius_km` | `fn neutron_star_radius_km(mass_solar: f64) → f64` | NS mass-radius relation | `physics::nucleosynthesis::stellar` |
| `luminosity_radius_temperature` | `fn luminosity_radius_temperature(radius_solar: f64, temperature_k: f64) → f64` | $L = 4\pi R^2\sigma T^4$ | `physics::nucleosynthesis::stellar` |
| `stellar_wind_mass_loss` | `fn stellar_wind_mass_loss(luminosity_solar: f64, escape_velocity_km_s: f64) → f64` | Stellar wind mass loss rate | `physics::nucleosynthesis::stellar` |

#### processes.rs (5 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ProcessType::threshold_temperature_k` | `fn threshold_temperature_k(&self) → f64` | Process threshold temperature | `physics::nucleosynthesis::processes` |
| `ProcessType::energy_released_mev` | `fn energy_released_mev(&self) → f64` | Energy released per reaction | `physics::nucleosynthesis::processes` |
| `ProcessType::is_active_at` | `fn is_active_at(&self, temperature_k: f64) → bool` | Active at given temperature? | `physics::nucleosynthesis::processes` |
| `active_processes` | `fn active_processes(temperature_k: f64) → Vec<ProcessType>` | All active processes at $T$ | `physics::nucleosynthesis::processes` |
| `dominant_process_at` | `fn dominant_process_at(temperature_k: f64) → Option<ProcessType>` | Dominant process at $T$ | `physics::nucleosynthesis::processes` |

### 8️⃣ quantum/ — Quantum Mechanics

#### wavefunctions.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `plane_wave` | `fn plane_wave(x: f64, k: f64, omega: f64, t: f64) → Complex` | $\psi = e^{i(kx-\omega t)}$ | `physics::quantum::wavefunctions` |
| `gaussian_packet` | `fn gaussian_packet(x: f64, x0: f64, sigma: f64, k0: f64) → Complex` | Gaussian wave packet | `physics::quantum::wavefunctions` |
| `normalize` | `fn normalize(psi: &mut [Complex], dx: f64)` | $\int|\psi|^2dx = 1$ | `physics::quantum::wavefunctions` |
| `probability_density` | `fn probability_density(psi: &[Complex]) → Vec<f64>` | $\rho(x) = |\psi(x)|^2$ | `physics::quantum::wavefunctions` |
| `expectation_position` | `fn expectation_position(psi: &[Complex], x: &[f64], dx: f64) → f64` | $\langle x\rangle = \int\psi^* x\psi\,dx$ | `physics::quantum::wavefunctions` |
| `expectation_momentum` | `fn expectation_momentum(psi: &[Complex], dx: f64) → f64` | $\langle p\rangle = -i\hbar\int\psi^*\nabla\psi\,dx$ | `physics::quantum::wavefunctions` |
| `time_evolve_split_step` | `fn time_evolve_split_step(psi: &mut [Complex], v: &[f64], dx: f64, dt: f64, mass: f64, steps: usize)` | Split-step Fourier propagation | `physics::quantum::wavefunctions` |
| `inner_product` | `fn inner_product(psi: &[Complex], phi: &[Complex], dx: f64) → Complex` | $\langle\psi|\phi\rangle$ | `physics::quantum::wavefunctions` |
| `transition_probability` | `fn transition_probability(psi_initial: &[Complex], psi_final: &[Complex], dx: f64) → f64` | $P = |\langle f|i\rangle|^2$ | `physics::quantum::wavefunctions` |

#### spin.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `pauli_x` | `fn pauli_x() → [[Complex; 2]; 2]` | $\sigma_x$ Pauli matrix | `physics::quantum::spin` |
| `pauli_y` | `fn pauli_y() → [[Complex; 2]; 2]` | $\sigma_y$ Pauli matrix | `physics::quantum::spin` |
| `pauli_z` | `fn pauli_z() → [[Complex; 2]; 2]` | $\sigma_z$ Pauli matrix | `physics::quantum::spin` |
| `identity_2` | `fn identity_2() → [[Complex; 2]; 2]` | $I_{2\times2}$ identity | `physics::quantum::spin` |
| `spin_up` | `fn spin_up() → [Complex; 2]` | $|\uparrow\rangle$ | `physics::quantum::spin` |
| `spin_down` | `fn spin_down() → [Complex; 2]` | $|\downarrow\rangle$ | `physics::quantum::spin` |
| `spin_plus_x` | `fn spin_plus_x() → [Complex; 2]` | $|+x\rangle$ | `physics::quantum::spin` |
| `spin_minus_x` | `fn spin_minus_x() → [Complex; 2]` | $|-x\rangle$ | `physics::quantum::spin` |
| `spin_plus_y` | `fn spin_plus_y() → [Complex; 2]` | $|+y\rangle$ | `physics::quantum::spin` |
| `spin_minus_y` | `fn spin_minus_y() → [Complex; 2]` | $|-y\rangle$ | `physics::quantum::spin` |
| `dirac_gamma0` | `fn dirac_gamma0() → [[Complex; 4]; 4]` | $\gamma^0$ Dirac matrix | `physics::quantum::spin` |
| `dirac_gamma1` | `fn dirac_gamma1() → [[Complex; 4]; 4]` | $\gamma^1$ | `physics::quantum::spin` |
| `dirac_gamma2` | `fn dirac_gamma2() → [[Complex; 4]; 4]` | $\gamma^2$ | `physics::quantum::spin` |
| `dirac_gamma3` | `fn dirac_gamma3() → [[Complex; 4]; 4]` | $\gamma^3$ | `physics::quantum::spin` |
| `gamma5` | `fn gamma5() → [[Complex; 4]; 4]` | $\gamma^5 = i\gamma^0\gamma^1\gamma^2\gamma^3$ | `physics::quantum::spin` |
| `rotation_operator` | `fn rotation_operator(angle: f64, pauli: &[[Complex; 2]; 2]) → [[Complex; 2]; 2]` | $R(\theta) = e^{-i\theta\sigma/2}$ | `physics::quantum::spin` |

#### operators.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `commutator` | `fn commutator(a: &[Vec<Complex>], b: &[Vec<Complex>]) → Vec<Vec<Complex>>` | $[A, B] = AB - BA$ | `physics::quantum::operators` |
| `anticommutator` | `fn anticommutator(a: &[Vec<Complex>], b: &[Vec<Complex>]) → Vec<Vec<Complex>>` | $\{A, B\} = AB + BA$ | `physics::quantum::operators` |
| `expectation_value` | `fn expectation_value(operator: &[Vec<Complex>], state: &[Complex]) → Complex` | $\langle\hat{O}\rangle = \langle\psi|\hat{O}|\psi\rangle$ | `physics::quantum::operators` |
| `variance` | `fn variance(operator: &[Vec<Complex>], state: &[Complex]) → f64` | $\sigma^2 = \langle\hat{O}^2\rangle - \langle\hat{O}\rangle^2$ | `physics::quantum::operators` |
| `uncertainty_product` | `fn uncertainty_product(op_a: &[Vec<Complex>], op_b: &[Vec<Complex>], state: &[Complex]) → f64` | $\Delta A \cdot \Delta B$ | `physics::quantum::operators` |
| `is_hermitian` | `fn is_hermitian(m: &[Vec<Complex>]) → bool` | $A = A^\dagger$ check | `physics::quantum::operators` |
| `is_unitary` | `fn is_unitary(m: &[Vec<Complex>]) → bool` | $UU^\dagger = I$ check | `physics::quantum::operators` |
| `tensor_product` | `fn tensor_product(a: &[Vec<Complex>], b: &[Vec<Complex>]) → Vec<Vec<Complex>>` | $A \otimes B$ | `physics::quantum::operators` |

#### angular.rs (3 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `associated_legendre` | `fn associated_legendre(l: u32, m: i32, x: f64) → f64` | $P_l^m(x)$ | `physics::quantum::angular` |
| `spherical_harmonic` | `fn spherical_harmonic(l: u32, m: i32, theta: f64, phi: f64) → Complex` | $Y_l^m(\theta, \phi)$ | `physics::quantum::angular` |
| `clebsch_gordan` | `fn clebsch_gordan(j1: f64, m1: f64, j2: f64, m2: f64, j: f64, m: f64) → f64` | $\langle j_1 m_1; j_2 m_2 | j m\rangle$ | `physics::quantum::angular` |

#### perturbation.rs (4 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `first_order_energy` | `fn first_order_energy(v_matrix: &[Vec<Complex>], state: &[Complex]) → f64` | $E_n^{(1)} = \langle n|V|n\rangle$ | `physics::quantum::perturbation` |
| `second_order_energy` | `fn second_order_energy(h0_energies: &[f64], v_matrix: &[Vec<Complex>], states: &[Vec<Complex>], n_index: usize) → f64` | $E_n^{(2)} = \sum_{m\neq n}\frac{|\langle m|V|n\rangle|^2}{E_n^{(0)}-E_m^{(0)}}$ | `physics::quantum::perturbation` |
| `first_order_state` | `fn first_order_state(h0_energies: &[f64], v_matrix: &[Vec<Complex>], states: &[Vec<Complex>], n_index: usize) → Vec<Complex>` | $|n^{(1)}\rangle$ first-order correction | `physics::quantum::perturbation` |
| `variational_energy` | `fn variational_energy(hamiltonian: &[Vec<Complex>], trial: &[Complex]) → f64` | $E_{var} = \frac{\langle\psi|H|\psi\rangle}{\langle\psi|\psi\rangle}$ | `physics::quantum::perturbation` |

#### systems.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hydrogen_energy` | `fn hydrogen_energy(n: u32) → f64` | $E_n = -13.6/n^2$ eV | `physics::quantum::systems` |
| `hydrogen_radial_r10` | `fn hydrogen_radial_r10(r: f64) → f64` | $R_{10}(r)$ hydrogen 1s | `physics::quantum::systems` |
| `hydrogen_radial_r20` | `fn hydrogen_radial_r20(r: f64) → f64` | $R_{20}(r)$ hydrogen 2s | `physics::quantum::systems` |
| `hydrogen_radial_r21` | `fn hydrogen_radial_r21(r: f64) → f64` | $R_{21}(r)$ hydrogen 2p | `physics::quantum::systems` |
| `harmonic_oscillator_energy` | `fn harmonic_oscillator_energy(n: u32, omega: f64) → f64` | $E_n = \hbar\omega(n+\frac{1}{2})$ | `physics::quantum::systems` |
| `harmonic_oscillator_wf` | `fn harmonic_oscillator_wf(n: u32, x: f64, mass: f64, omega: f64) → f64` | QHO wave function $\psi_n(x)$ | `physics::quantum::systems` |
| `infinite_well_energy` | `fn infinite_well_energy(n: u32, length: f64, mass: f64) → f64` | $E_n = \frac{n^2\pi^2\hbar^2}{2mL^2}$ | `physics::quantum::systems` |
| `infinite_well_wf` | `fn infinite_well_wf(n: u32, x: f64, length: f64) → f64` | $\psi_n = \sqrt{2/L}\sin(n\pi x/L)$ | `physics::quantum::systems` |
| `tunneling_coefficient` | `fn tunneling_coefficient(energy: f64, v0: f64, width: f64, mass: f64) → f64` | $T \approx e^{-2\kappa d}$ | `physics::quantum::systems` |
| `bohr_radius` | `fn bohr_radius() → f64` | $a_0 = \frac{4\pi\varepsilon_0\hbar^2}{m_e e^2}$ | `physics::quantum::systems` |
| `landau_levels` | `fn landau_levels(n: u32, b_field: f64, mass: f64, charge: f64) → f64` | $E_n = \hbar\omega_c(n+\frac{1}{2})$ | `physics::quantum::systems` |
| `zeeman_splitting` | `fn zeeman_splitting(m_l: i32, b_field: f64) → f64` | $\Delta E = m_l \mu_B B$ | `physics::quantum::systems` |
| `bohr_radius_nth` | `fn bohr_radius_nth(n: u32, z_eff: f64) → f64` | $a_n = a_0 n^2/Z_{eff}$ | `physics::quantum::systems` |

#### information.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `von_neumann_entropy` | `fn von_neumann_entropy(rho: &[Vec<Complex>]) → f64` | $S = -\text{Tr}(\rho\ln\rho)$ | `physics::quantum::information` |
| `purity` | `fn purity(rho: &[Vec<Complex>]) → f64` | $\gamma = \text{Tr}(\rho^2)$ | `physics::quantum::information` |
| `fidelity_pure` | `fn fidelity_pure(psi: &[Complex], phi: &[Complex]) → f64` | $F = |\langle\psi|\phi\rangle|^2$ | `physics::quantum::information` |
| `fidelity_mixed` | `fn fidelity_mixed(rho: &[Vec<Complex>], sigma: &[Vec<Complex>]) → f64` | $F(\rho,\sigma)$ mixed state fidelity | `physics::quantum::information` |
| `concurrence_2qubit` | `fn concurrence_2qubit(rho: &[Vec<Complex>]) → f64` | Wootters concurrence | `physics::quantum::information` |
| `bell_phi_plus` | `fn bell_phi_plus() → Vec<Complex>` | $|\Phi^+\rangle = \frac{1}{\sqrt{2}}(|00\rangle+|11\rangle)$ | `physics::quantum::information` |
| `bell_phi_minus` | `fn bell_phi_minus() → Vec<Complex>` | $|\Phi^-\rangle = \frac{1}{\sqrt{2}}(|00\rangle-|11\rangle)$ | `physics::quantum::information` |

### 9️⃣ acoustics/ — Sound & Vibrations

#### propagation.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `speed_of_sound_gas` | `fn speed_of_sound_gas(gamma: f64, r: f64, t: f64, m: f64) → f64` | $c = \sqrt{\gamma RT/M}$ | `physics::acoustics::propagation` |
| `speed_of_sound_solid` | `fn speed_of_sound_solid(e: f64, rho: f64) → f64` | $c = \sqrt{E/\rho}$ | `physics::acoustics::propagation` |
| `wavelength` | `fn wavelength(speed: f64, frequency: f64) → f64` | $\lambda = c/f$ | `physics::acoustics::propagation` |
| `intensity` | `fn intensity(power: f64, area: f64) → f64` | $I = P/A$ | `physics::acoustics::propagation` |
| `intensity_level_db` | `fn intensity_level_db(intensity: f64, i_ref: f64) → f64` | $L_I = 10\log_{10}(I/I_0)$ | `physics::acoustics::propagation` |
| `sound_pressure_level` | `fn sound_pressure_level(p: f64, p_ref: f64) → f64` | $SPL = 20\log_{10}(p/p_0)$ | `physics::acoustics::propagation` |
| `inverse_square_law` | `fn inverse_square_law(i0: f64, r0: f64, r: f64) → f64` | $I \propto 1/r^2$ | `physics::acoustics::propagation` |
| `acoustic_impedance` | `fn acoustic_impedance(rho: f64, c: f64) → f64` | $Z = \rho c$ | `physics::acoustics::propagation` |
| `transmission_coefficient` | `fn transmission_coefficient(z1: f64, z2: f64) → f64` | $T = 4Z_1Z_2/(Z_1+Z_2)^2$ | `physics::acoustics::propagation` |
| `reflection_coefficient` | `fn reflection_coefficient(z1: f64, z2: f64) → f64` | $R = (Z_2-Z_1)^2/(Z_1+Z_2)^2$ | `physics::acoustics::propagation` |
| `spherical_spreading` | `fn spherical_spreading(p0: f64, r0: f64, r: f64) → f64` | $p = p_0 r_0/r$ | `physics::acoustics::propagation` |
| `plane_wave_pressure` | `fn plane_wave_pressure(rho: f64, c: f64, v: f64) → f64` | $p = \rho c v$ | `physics::acoustics::propagation` |

#### absorption.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `absorption_coefficient` | `fn absorption_coefficient(i0: f64, i: f64, x: f64) → f64` | $\alpha = \frac{1}{x}\ln(I_0/I)$ | `physics::acoustics::absorption` |
| `intensity_after_absorption` | `fn intensity_after_absorption(i0: f64, alpha: f64, x: f64) → f64` | $I = I_0 e^{-\alpha x}$ | `physics::acoustics::absorption` |
| `atmospheric_absorption` | `fn atmospheric_absorption(f: f64, humidity: f64, temperature: f64) → f64` | Atmospheric absorption coefficient | `physics::acoustics::absorption` |
| `noise_reduction_coefficient` | `fn noise_reduction_coefficient(alphas: &[f64]) → f64` | NRC average | `physics::acoustics::absorption` |
| `sound_transmission_class` | `fn sound_transmission_class(tl_values: &[f64]) → f64` | STC rating | `physics::acoustics::absorption` |
| `mass_law_transmission_loss` | `fn mass_law_transmission_loss(f: f64, surface_density: f64) → f64` | $TL = 20\log_{10}(fm) - 48$ | `physics::acoustics::absorption` |
| `porous_absorber_flow_resistivity` | `fn porous_absorber_flow_resistivity(sigma: f64, thickness: f64, f: f64) → f64` | Porous absorber model | `physics::acoustics::absorption` |
| `a_weighting` | `fn a_weighting(f: f64) → f64` | A-weighting curve | `physics::acoustics::absorption` |
| `decibel_addition` | `fn decibel_addition(levels: &[f64]) → f64` | $L_{total} = 10\log_{10}\sum 10^{L_i/10}$ | `physics::acoustics::absorption` |
| `room_constant` | `fn room_constant(s: f64, alpha_avg: f64) → f64` | $R = \frac{S\alpha}{1-\alpha}$ | `physics::acoustics::absorption` |

#### doppler.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `doppler_approaching` | `fn doppler_approaching(f0: f64, c: f64, v_source: f64) → f64` | $f = f_0\frac{c}{c-v_s}$ | `physics::acoustics::doppler` |
| `doppler_receding` | `fn doppler_receding(f0: f64, c: f64, v_source: f64) → f64` | $f = f_0\frac{c}{c+v_s}$ | `physics::acoustics::doppler` |
| `doppler_general` | `fn doppler_general(f0: f64, c: f64, v_observer: f64, v_source: f64) → f64` | $f = f_0\frac{c+v_o}{c-v_s}$ | `physics::acoustics::doppler` |
| `relativistic_doppler` | `fn relativistic_doppler(f0: f64, v: f64, c: f64) → f64` | Relativistic Doppler | `physics::acoustics::doppler` |
| `mach_cone_angle` | `fn mach_cone_angle(v: f64, c: f64) → f64` | $\sin\alpha = c/v$ (Mach cone) | `physics::acoustics::doppler` |
| `doppler_shift_wavelength` | `fn doppler_shift_wavelength(lambda0: f64, v: f64, c: f64) → f64` | Wavelength Doppler shift | `physics::acoustics::doppler` |
| `doppler_radar_velocity` | `fn doppler_radar_velocity(delta_f: f64, f0: f64, c: f64) → f64` | Doppler radar velocity | `physics::acoustics::doppler` |
| `sonic_boom_pressure` | `fn sonic_boom_pressure(k: f64, l: f64, d: f64, mach: f64) → f64` | Sonic boom overpressure | `physics::acoustics::doppler` |

#### resonance.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fundamental_frequency_string` | `fn fundamental_frequency_string(l: f64, tension: f64, mu: f64) → f64` | $f_1 = \frac{1}{2L}\sqrt{T/\mu}$ | `physics::acoustics::resonance` |
| `harmonic_frequency` | `fn harmonic_frequency(fundamental: f64, n: u32) → f64` | $f_n = nf_1$ | `physics::acoustics::resonance` |
| `resonant_frequency_pipe_open` | `fn resonant_frequency_pipe_open(l: f64, c: f64, n: u32) → f64` | $f_n = nc/(2L)$ | `physics::acoustics::resonance` |
| `resonant_frequency_pipe_closed` | `fn resonant_frequency_pipe_closed(l: f64, c: f64, n: u32) → f64` | $f_n = nc/(4L)$ (odd) | `physics::acoustics::resonance` |
| `quality_factor` | `fn quality_factor(f0: f64, bandwidth: f64) → f64` | $Q = f_0/\Delta f$ | `physics::acoustics::resonance` |
| `helmholtz_resonator` | `fn helmholtz_resonator(c: f64, a: f64, v: f64, l: f64) → f64` | $f = \frac{c}{2\pi}\sqrt{A/(VL)}$ | `physics::acoustics::resonance` |
| `beat_frequency` | `fn beat_frequency(f1: f64, f2: f64) → f64` | $f_b = |f_1 - f_2|$ | `physics::acoustics::resonance` |
| `standing_wave_nodes` | `fn standing_wave_nodes(l: f64, wavelength: f64) → u32` | Number of standing wave nodes | `physics::acoustics::resonance` |
| `reverberation_time_sabine` | `fn reverberation_time_sabine(v: f64, a: f64) → f64` | $T_{60} = 0.161V/A$ (Sabine) | `physics::acoustics::resonance` |
| `room_mode_frequency` | `fn room_mode_frequency(c: f64, lx: f64, ly: f64, lz: f64, nx: u32, ny: u32, nz: u32) → f64` | Room mode frequencies | `physics::acoustics::resonance` |
| `schroeder_frequency` | `fn schroeder_frequency(rt60: f64, v: f64) → f64` | $f_S = 2000\sqrt{T_{60}/V}$ | `physics::acoustics::resonance` |

### 🔟 materials/ — Condensed Matter & Material Science

#### crystallography.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `bragg_angle` | `fn bragg_angle(d: f64, wavelength: f64, n: i32) → f64` | $n\lambda = 2d\sin\theta$ | `physics::materials::crystallography` |
| `d_spacing_cubic` | `fn d_spacing_cubic(a: f64, h: i32, k: i32, l: i32) → f64` | $d = a/\sqrt{h^2+k^2+l^2}$ | `physics::materials::crystallography` |
| `miller_planar_density_cubic` | `fn miller_planar_density_cubic(a: f64, atoms_per_plane: f64, h: i32, k: i32, l: i32) → f64` | Planar density for $(hkl)$ | `physics::materials::crystallography` |
| `packing_fraction_bcc` | `fn packing_fraction_bcc() → f64` | BCC packing fraction 0.68 | `physics::materials::crystallography` |
| `packing_fraction_fcc` | `fn packing_fraction_fcc() → f64` | FCC packing fraction 0.74 | `physics::materials::crystallography` |
| `lattice_parameter_from_density` | `fn lattice_parameter_from_density(m: f64, z: f64, rho: f64) → f64` | $a = (zM/\rho N_A)^{1/3}$ | `physics::materials::crystallography` |
| `structure_factor_bcc` | `fn structure_factor_bcc(h: i32, k: i32, l: i32) → f64` | BCC structure factor | `physics::materials::crystallography` |
| `structure_factor_fcc` | `fn structure_factor_fcc(h: i32, k: i32, l: i32) → f64` | FCC structure factor | `physics::materials::crystallography` |
| `debye_temperature` | `fn debye_temperature(theta_d: f64, t: f64) → f64` | Debye temperature estimate | `physics::materials::crystallography` |
| `specific_heat_debye` | `fn specific_heat_debye(t: f64, theta_d: f64) → f64` | $C_V = 9Nk_B(T/\Theta_D)^3\int_0^{\Theta_D/T}...$ | `physics::materials::crystallography` |
| `vacancy_concentration` | `fn vacancy_concentration(ev: f64, t: f64) → f64` | $n_v/N = e^{-E_v/k_BT}$ | `physics::materials::crystallography` |

#### diffusion.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fick_first_law` | `fn fick_first_law(d: f64, dc_dx: f64) → f64` | $J = -D\frac{dc}{dx}$ | `physics::materials::diffusion` |
| `fick_second_law_solution` | `fn fick_second_law_solution(c0: f64, cs: f64, x: f64, d: f64, t: f64) → f64` | $c(x,t)$ solution to Fick's 2nd law | `physics::materials::diffusion` |
| `diffusion_coefficient` | `fn diffusion_coefficient(d0: f64, q: f64, t: f64) → f64` | $D = D_0 e^{-Q/RT}$ | `physics::materials::diffusion` |
| `diffusion_length` | `fn diffusion_length(d: f64, t: f64) → f64` | $L = \sqrt{Dt}$ | `physics::materials::diffusion` |
| `interdiffusion_coefficient` | `fn interdiffusion_coefficient(d_a: f64, d_b: f64, x_a: f64) → f64` | Darken equation $\tilde{D}$ | `physics::materials::diffusion` |
| `kirkendall_velocity` | `fn kirkendall_velocity(d_a: f64, d_b: f64, dc_dx: f64, c_total: f64) → f64` | Kirkendall marker shift | `physics::materials::diffusion` |
| `grain_boundary_diffusion` | `fn grain_boundary_diffusion(d_gb: f64, delta: f64, d_l: f64, grain_size: f64) → f64` | Effective GB diffusion | `physics::materials::diffusion` |
| `permeability` | `fn permeability(d: f64, s: f64) → f64` | $P = DS$ | `physics::materials::diffusion` |
| `carburization_depth` | `fn carburization_depth(d: f64, t: f64) → f64` | $x \approx \sqrt{Dt}$ | `physics::materials::diffusion` |
| `mean_free_path` | `fn mean_free_path(d: f64, n_density: f64) → f64` | Diffusion mean free path | `physics::materials::diffusion` |

#### phases.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `lever_rule` | `fn lever_rule(c0: f64, c_alpha: f64, c_beta: f64) → (f64, f64)` | Phase fractions from lever rule | `physics::materials::phases` |
| `gibbs_phase_rule` | `fn gibbs_phase_rule(c: u32, p: u32) → i32` | $F = C - P + 2$ | `physics::materials::phases` |
| `clausius_clapeyron_slope` | `fn clausius_clapeyron_slope(delta_h: f64, t: f64, delta_v: f64) → f64` | $dP/dT = \Delta H/(T\Delta V)$ | `physics::materials::phases` |
| `regular_solution_gibbs` | `fn regular_solution_gibbs(xa: f64, omega: f64, t: f64) → f64` | Regular solution model $G_{mix}$ | `physics::materials::phases` |
| `spinodal_temperature` | `fn spinodal_temperature(omega: f64) → f64` | $T_s = \Omega/(2R)$ | `physics::materials::phases` |
| `nucleation_barrier` | `fn nucleation_barrier(gamma: f64, delta_gv: f64) → f64` | $\Delta G^* = 16\pi\gamma^3/(3\Delta G_V^2)$ | `physics::materials::phases` |
| `critical_nucleus_radius` | `fn critical_nucleus_radius(gamma: f64, delta_gv: f64) → f64` | $r^* = 2\gamma/|\Delta G_V|$ | `physics::materials::phases` |
| `nucleation_rate` | `fn nucleation_rate(n0: f64, delta_g_star: f64, t: f64) → f64` | $J = N_0 e^{-\Delta G^*/k_BT}$ | `physics::materials::phases` |
| `coarsening_rate` | `fn coarsening_rate(k: f64, t: f64, t0: f64) → f64` | LSW coarsening $\bar{r}^3$ | `physics::materials::phases` |
| `jmak` | `fn jmak(k: f64, t: f64, n: f64) → f64` | $f = 1 - e^{-kt^n}$ (JMAK) | `physics::materials::phases` |
| `partition_coefficient` | `fn partition_coefficient(c_solid: f64, c_liquid: f64) → f64` | $k = C_s/C_l$ | `physics::materials::phases` |
| `scheil_equation` | `fn scheil_equation(c0: f64, k: f64, fs: f64) → f64` | $C_l = C_0(1-f_s)^{k-1}$ | `physics::materials::phases` |

#### semiconductors.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fermi_energy` | `fn fermi_energy(n: f64, m_eff: f64) → f64` | $E_F = \frac{\hbar^2}{2m}(3\pi^2 n)^{2/3}$ | `physics::materials::semiconductors` |
| `fermi_dirac` | `fn fermi_dirac(e: f64, ef: f64, t: f64) → f64` | $f(E) = \frac{1}{e^{(E-E_F)/k_BT}+1}$ | `physics::materials::semiconductors` |
| `intrinsic_carrier_concentration` | `fn intrinsic_carrier_concentration(eg: f64, t: f64, nc: f64, nv: f64) → f64` | $n_i = \sqrt{N_cN_v}e^{-E_g/2k_BT}$ | `physics::materials::semiconductors` |
| `conductivity_semiconductor` | `fn conductivity_semiconductor(n: f64, mu_e: f64, p: f64, mu_h: f64) → f64` | $\sigma = q(n\mu_e + p\mu_h)$ | `physics::materials::semiconductors` |
| `hall_coefficient` | `fn hall_coefficient(n: f64, p: f64, mu_e: f64, mu_h: f64) → f64` | Hall coefficient $R_H$ | `physics::materials::semiconductors` |
| `drift_velocity` | `fn drift_velocity(mu: f64, e_field: f64) → f64` | $v_d = \mu E$ | `physics::materials::semiconductors` |
| `depletion_width` | `fn depletion_width(epsilon: f64, v_bi: f64, na: f64, nd: f64) → f64` | $W = \sqrt{\frac{2\varepsilon V_{bi}}{q}\frac{N_A+N_D}{N_AN_D}}$ | `physics::materials::semiconductors` |
| `built_in_potential` | `fn built_in_potential(na: f64, nd: f64, ni: f64, t: f64) → f64` | $V_{bi} = \frac{kT}{q}\ln\frac{N_AN_D}{n_i^2}$ | `physics::materials::semiconductors` |
| `diode_current` | `fn diode_current(is: f64, v: f64, t: f64, n: f64) → f64` | $I = I_s(e^{qV/nkT}-1)$ | `physics::materials::semiconductors` |
| `band_gap_temperature` | `fn band_gap_temperature(eg0: f64, alpha: f64, beta: f64, t: f64) → f64` | $E_g(T) = E_g(0) - \frac{\alpha T^2}{T+\beta}$ (Varshni) | `physics::materials::semiconductors` |
| `doping_ionization` | `fn doping_ionization(nd: f64, ed: f64, t: f64) → f64` | Dopant ionization fraction | `physics::materials::semiconductors` |

### 1️⃣1️⃣ thermodynamics/ — Classical & Statistical Thermodynamics

#### equations.rs (29 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ideal_gas_pressure` | `fn ideal_gas_pressure(n_moles: f64, temperature: f64, volume: f64) → f64` | $PV = nRT$ | `physics::thermodynamics::equations` |
| `ideal_gas_volume` | `fn ideal_gas_volume(n_moles: f64, temperature: f64, pressure: f64) → f64` | $V = nRT/P$ | `physics::thermodynamics::equations` |
| `ideal_gas_temperature` | `fn ideal_gas_temperature(pressure: f64, volume: f64, n_moles: f64) → f64` | $T = PV/nR$ | `physics::thermodynamics::equations` |
| `van_der_waals_pressure` | `fn van_der_waals_pressure(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) → f64` | $(P+an^2/V^2)(V-nb) = nRT$ | `physics::thermodynamics::equations` |
| `redlich_kwong_pressure` | `fn redlich_kwong_pressure(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) → f64` | Redlich-Kwong EOS | `physics::thermodynamics::equations` |
| `virial_eos` | `fn virial_eos(pressure: f64, temperature: f64, b2: f64, b3: f64) → f64` | Virial equation of state | `physics::thermodynamics::equations` |
| `compressibility_factor` | `fn compressibility_factor(pressure: f64, volume: f64, n_moles: f64, temperature: f64) → f64` | $Z = PV/nRT$ | `physics::thermodynamics::equations` |
| `internal_energy_ideal` | `fn internal_energy_ideal(n_moles: f64, cv: f64, temperature: f64) → f64` | $U = nC_vT$ | `physics::thermodynamics::equations` |
| `enthalpy_ideal` | `fn enthalpy_ideal(n_moles: f64, cp: f64, temperature: f64) → f64` | $H = nC_pT$ | `physics::thermodynamics::equations` |
| `entropy_ideal_gas` | `fn entropy_ideal_gas(n_moles: f64, cv: f64, t1: f64, t2: f64, v1: f64, v2: f64) → f64` | $\Delta S = nC_v\ln(T_2/T_1) + nR\ln(V_2/V_1)$ | `physics::thermodynamics::equations` |
| `gibbs_free_energy` | `fn gibbs_free_energy(enthalpy: f64, temperature: f64, entropy: f64) → f64` | $G = H - TS$ | `physics::thermodynamics::equations` |
| `helmholtz_free_energy` | `fn helmholtz_free_energy(internal_energy: f64, temperature: f64, entropy: f64) → f64` | $A = U - TS$ | `physics::thermodynamics::equations` |
| `chemical_potential_ideal_gas` | `fn chemical_potential_ideal_gas(mu0: f64, temperature: f64, pressure: f64, p0: f64) → f64` | $\mu = \mu^0 + RT\ln(P/P^0)$ | `physics::thermodynamics::equations` |
| `clausius_clapeyron` | `fn clausius_clapeyron(p1: f64, t1: f64, t2: f64, delta_h_vap: f64) → f64` | $\ln(P_2/P_1) = \frac{\Delta H}{R}(\frac{1}{T_1}-\frac{1}{T_2})$ | `physics::thermodynamics::equations` |
| `heat_capacity_ratio` | `fn heat_capacity_ratio(cp: f64, cv: f64) → f64` | $\gamma = C_p/C_v$ | `physics::thermodynamics::equations` |
| `speed_of_sound_ideal` | `fn speed_of_sound_ideal(gamma: f64, temperature: f64, molar_mass: f64) → f64` | $c = \sqrt{\gamma RT/M}$ | `physics::thermodynamics::equations` |
| `maxwell_speed_distribution` | `fn maxwell_speed_distribution(v: f64, mass: f64, temperature: f64) → f64` | $f(v) = 4\pi n\left(\frac{m}{2\pi k_BT}\right)^{3/2}v^2 e^{-mv^2/2k_BT}$ | `physics::thermodynamics::equations` |
| `most_probable_speed` | `fn most_probable_speed(mass: f64, temperature: f64) → f64` | $v_p = \sqrt{2k_BT/m}$ | `physics::thermodynamics::equations` |
| `mean_speed` | `fn mean_speed(mass: f64, temperature: f64) → f64` | $\langle v\rangle = \sqrt{8k_BT/\pi m}$ | `physics::thermodynamics::equations` |
| `rms_speed` | `fn rms_speed(mass: f64, temperature: f64) → f64` | $v_{rms} = \sqrt{3k_BT/m}$ | `physics::thermodynamics::equations` |
| `mean_free_path` | `fn mean_free_path(number_density: f64, cross_section: f64) → f64` | $\lambda = 1/(n\sigma)$ | `physics::thermodynamics::equations` |
| `pressure_atm_to_pascal` | `fn pressure_atm_to_pascal(p_atm: f64) → f64` | atm → Pa | `physics::thermodynamics::equations` |
| `pressure_pascal_to_atm` | `fn pressure_pascal_to_atm(p_pa: f64) → f64` | Pa → atm | `physics::thermodynamics::equations` |
| `pressure_bar_to_pascal` | `fn pressure_bar_to_pascal(p_bar: f64) → f64` | bar → Pa | `physics::thermodynamics::equations` |
| `energy_calories_to_joules` | `fn energy_calories_to_joules(cal: f64) → f64` | cal → J | `physics::thermodynamics::equations` |
| `energy_joules_to_calories` | `fn energy_joules_to_calories(j: f64) → f64` | J → cal | `physics::thermodynamics::equations` |
| `energy_kwh_to_joules` | `fn energy_kwh_to_joules(kwh: f64) → f64` | kWh → J | `physics::thermodynamics::equations` |
| `plasma_temperature_kev_to_kelvin` | `fn plasma_temperature_kev_to_kelvin(t_kev: f64) → f64` | keV → K | `physics::thermodynamics::equations` |
| `ideal_gas_pressure_atm` | `fn ideal_gas_pressure_atm(n_moles: f64, temperature: f64, volume_liters: f64) → f64` | Ideal gas in atm/L | `physics::thermodynamics::equations` |

#### processes.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `carnot_efficiency` | `fn carnot_efficiency(t_hot: f64, t_cold: f64) → f64` | $\eta = 1 - T_C/T_H$ | `physics::thermodynamics::processes` |
| `carnot_cop_heating` | `fn carnot_cop_heating(t_hot: f64, t_cold: f64) → f64` | $COP_H = T_H/(T_H-T_C)$ | `physics::thermodynamics::processes` |
| `carnot_cop_cooling` | `fn carnot_cop_cooling(t_hot: f64, t_cold: f64) → f64` | $COP_C = T_C/(T_H-T_C)$ | `physics::thermodynamics::processes` |
| `isothermal_work` | `fn isothermal_work(n_moles: f64, temperature: f64, v1: f64, v2: f64) → f64` | $W = nRT\ln(V_2/V_1)$ | `physics::thermodynamics::processes` |
| `adiabatic_work` | `fn adiabatic_work(n_moles: f64, cv: f64, t1: f64, t2: f64) → f64` | $W = nC_v(T_1-T_2)$ | `physics::thermodynamics::processes` |
| `isobaric_work` | `fn isobaric_work(pressure: f64, v1: f64, v2: f64) → f64` | $W = P(V_2-V_1)$ | `physics::thermodynamics::processes` |
| `adiabatic_relation_tv` | `fn adiabatic_relation_tv(t1: f64, v1: f64, v2: f64, gamma: f64) → f64` | $TV^{\gamma-1} = \text{const}$ | `physics::thermodynamics::processes` |
| `adiabatic_relation_pv` | `fn adiabatic_relation_pv(p1: f64, v1: f64, v2: f64, gamma: f64) → f64` | $PV^\gamma = \text{const}$ | `physics::thermodynamics::processes` |
| `otto_efficiency` | `fn otto_efficiency(compression_ratio: f64, gamma: f64) → f64` | $\eta = 1 - r^{1-\gamma}$ | `physics::thermodynamics::processes` |
| `diesel_efficiency` | `fn diesel_efficiency(compression_ratio: f64, cutoff_ratio: f64, gamma: f64) → f64` | Diesel cycle efficiency | `physics::thermodynamics::processes` |
| `joule_thomson_coefficient` | `fn joule_thomson_coefficient(cp: f64, v_molar: f64, temperature: f64, dv_dt_p: f64) → f64` | $\mu_{JT} = \frac{1}{C_p}(T\frac{\partial V}{\partial T}_P - V)$ | `physics::thermodynamics::processes` |
| `throttling_temperature_change` | `fn throttling_temperature_change(mu_jt: f64, dp: f64) → f64` | $\Delta T = \mu_{JT}\Delta P$ | `physics::thermodynamics::processes` |
| `heat_conduction_rate` | `fn heat_conduction_rate(k: f64, area: f64, dt: f64, dx: f64) → f64` | $\dot{Q} = -kA\frac{dT}{dx}$ | `physics::thermodynamics::processes` |
| `thermal_diffusion_1d` | `fn thermal_diffusion_1d(t: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize)` | 1D heat equation FTCS | `physics::thermodynamics::processes` |
| `mixing_entropy` | `fn mixing_entropy(mole_fractions: &[f64]) → f64` | $\Delta S_{mix} = -R\sum x_i\ln x_i$ | `physics::thermodynamics::processes` |
| `reaction_gibbs` | `fn reaction_gibbs(delta_g0: f64, temperature: f64, q: f64) → f64` | $\Delta G = \Delta G^0 + RT\ln Q$ | `physics::thermodynamics::processes` |
| `equilibrium_constant` | `fn equilibrium_constant(delta_g0: f64, temperature: f64) → f64` | $K = e^{-\Delta G^0/RT}$ | `physics::thermodynamics::processes` |
| `vant_hoff` | `fn vant_hoff(k1: f64, delta_h: f64, t1: f64, t2: f64) → f64` | $\ln(K_2/K_1) = -\frac{\Delta H}{R}(\frac{1}{T_2}-\frac{1}{T_1})$ | `physics::thermodynamics::processes` |

#### statistical.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `boltzmann_distribution` | `fn boltzmann_distribution(energy: f64, temperature: f64) → f64` | $P \propto e^{-E/k_BT}$ | `physics::thermodynamics::statistical` |
| `partition_function_discrete` | `fn partition_function_discrete(energies: &[f64], temperature: f64) → f64` | $Z = \sum e^{-E_i/k_BT}$ | `physics::thermodynamics::statistical` |
| `partition_function_harmonic` | `fn partition_function_harmonic(omega: f64, temperature: f64) → f64` | QHO partition function | `physics::thermodynamics::statistical` |
| `mean_energy_canonical` | `fn mean_energy_canonical(energies: &[f64], temperature: f64) → f64` | $\langle E\rangle = -\partial\ln Z/\partial\beta$ | `physics::thermodynamics::statistical` |
| `entropy_canonical` | `fn entropy_canonical(energies: &[f64], temperature: f64) → f64` | Canonical ensemble entropy | `physics::thermodynamics::statistical` |
| `fermi_dirac` | `fn fermi_dirac(energy: f64, mu: f64, temperature: f64) → f64` | $f_{FD}(E)$ | `physics::thermodynamics::statistical` |
| `bose_einstein` | `fn bose_einstein(energy: f64, mu: f64, temperature: f64) → f64` | $f_{BE}(E)$ | `physics::thermodynamics::statistical` |
| `planck_radiation` | `fn planck_radiation(frequency: f64, temperature: f64) → f64` | $B_\nu(T)$ Planck spectral radiance | `physics::thermodynamics::statistical` |
| `planck_radiation_wavelength` | `fn planck_radiation_wavelength(wavelength: f64, temperature: f64) → f64` | $B_\lambda(T)$ | `physics::thermodynamics::statistical` |
| `wien_displacement` | `fn wien_displacement(temperature: f64) → f64` | $\lambda_{max} = b/T$ | `physics::thermodynamics::statistical` |
| `stefan_boltzmann_power` | `fn stefan_boltzmann_power(temperature: f64, area: f64) → f64` | $P = \sigma A T^4$ | `physics::thermodynamics::statistical` |
| `debye_heat_capacity` | `fn debye_heat_capacity(temperature: f64, debye_temp: f64, n_atoms: f64) → f64` | Debye model $C_V$ | `physics::thermodynamics::statistical` |
| `einstein_heat_capacity` | `fn einstein_heat_capacity(temperature: f64, einstein_temp: f64, n_atoms: f64) → f64` | Einstein model $C_V$ | `physics::thermodynamics::statistical` |
| `sackur_tetrode` | `fn sackur_tetrode(n_moles: f64, temperature: f64, volume: f64, mass: f64) → f64` | Sackur-Tetrode entropy | `physics::thermodynamics::statistical` |
| `fermi_energy` | `fn fermi_energy(mass: f64, number_density: f64) → f64` | $E_F = \frac{\hbar^2}{2m}(3\pi^2 n)^{2/3}$ | `physics::thermodynamics::statistical` |
