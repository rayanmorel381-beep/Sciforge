# Meteorology ChangeLog

## v0.0.4

### 1️⃣ New Submodule — `winds.rs` — Atmospheric Wind Dynamics (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hadley_cell_extent` | `fn hadley_cell_extent(planet_radius: f64, rotation_rate: f64, delta_t: f64) → f64` | Hadley cell poleward extent | `meteorology::winds` |
| `thermal_wind_shear` | `fn thermal_wind_shear(g_local: f64, coriolis: f64, temperature: f64, dt_dy: f64) → f64` | $\partial u/\partial z = -g\,\partial T/\partial y\,/(f T)$ | `meteorology::winds` |
| `jet_stream_velocity` | `fn jet_stream_velocity(g_local: f64, delta_t: f64, meridional_distance: f64, coriolis: f64, scale_height: f64) → f64` | Thermal wind jet estimate | `meteorology::winds` |
| `surface_wind_speed` | `fn surface_wind_speed(pressure_gradient: f64, density: f64, drag_coefficient: f64) → f64` | $u = \sqrt{(\nabla P)/(\rho C_d)}$ | `meteorology::winds` |
| `wind_stress` | `fn wind_stress(air_density: f64, drag_coefficient: f64, wind_speed: f64) → f64` | $\tau = \rho C_d u^2$ | `meteorology::winds` |
| `planetary_vorticity` | `fn planetary_vorticity(rotation_rate: f64, latitude: f64) → f64` | $f = 2\Omega\sin\phi$ | `meteorology::winds` |
| `rossby_wave_phase_speed` | `fn rossby_wave_phase_speed(beta: f64, zonal_wavenumber: f64, deformation_radius: f64) → f64` | $c_R = -\beta/(k^2 + L_d^{-2})$ | `meteorology::winds` |
| `beta_parameter` | `fn beta_parameter(rotation_rate: f64, planet_radius: f64, latitude: f64) → f64` | $\beta = 2\Omega\cos\phi/a$ | `meteorology::winds` |
| `baroclinic_instability_wavelength` | `fn baroclinic_instability_wavelength(deformation_radius: f64) → f64` | $\lambda_{bc} = 2\pi\sqrt{2}\,L_d$ | `meteorology::winds` |
| `sea_breeze_speed` | `fn sea_breeze_speed(g_local: f64, delta_t: f64, boundary_layer_height: f64, mean_temperature: f64) → f64` | $u = \sqrt{g\,\Delta T\,h/T}$ | `meteorology::winds` |
| `katabatic_wind_speed` | `fn katabatic_wind_speed(g_local: f64, delta_t: f64, mean_temperature: f64, slope_angle: f64, slope_length: f64, drag_coefficient: f64) → f64` | Buoyancy-driven slope flow | `meteorology::winds` |
| `mountain_wave_vertical_velocity` | `fn mountain_wave_vertical_velocity(wind_speed: f64, mountain_height: f64, brunt_vaisala: f64, horizontal_wavelength: f64) → f64` | Orographic wave vertical velocity | `meteorology::winds` |
| `ekman_pumping_velocity` | `fn ekman_pumping_velocity(wind_stress_curl: f64, density: f64, coriolis: f64) → f64` | $w_E = \nabla\times\boldsymbol{\tau}/(\rho f)$ | `meteorology::winds` |
| `monin_obukhov_length` | `fn monin_obukhov_length(friction_velocity: f64, surface_temp: f64, sensible_heat_flux: f64, air_density: f64, specific_heat: f64) → f64` | $L = -\rho c_p T u_*^3/(k g H)$ | `meteorology::winds` |
| `log_wind_profile` | `fn log_wind_profile(friction_velocity: f64, z: f64, roughness_length: f64) → f64` | $u(z) = (u_*/k)\ln(z/z_0)$ | `meteorology::winds` |
| `cyclostrophic_wind` | `fn cyclostrophic_wind(pressure_gradient: f64, density: f64, radius: f64) → f64` | $u_c = \sqrt{(\nabla P)\,r/\rho}$ | `meteorology::winds` |
| `gradient_wind` | `fn gradient_wind(coriolis: f64, pressure_gradient: f64, density: f64, radius: f64) → f64` | Gradient wind balance solution | `meteorology::winds` |
| `superrotation_index` | `fn superrotation_index(zonal_wind: f64, planet_radius: f64, rotation_rate: f64, latitude: f64) → f64` | $S = u/(\Omega a\cos\phi) - 1$ | `meteorology::winds` |
| `foehn_warming` | `fn foehn_warming(lapse_dry: f64, lapse_moist: f64, mountain_height: f64, condensation_level: f64) → f64` | Net temperature gain from foehn descent | `meteorology::winds` |
| `beaufort_to_m_s` | `fn beaufort_to_m_s(b: f64) → f64` | $u = C_B\, b^{e_B}$ | `meteorology::winds` |
| `wind_chill` | `fn wind_chill(t: f64, v: f64) → f64` | Environment Canada wind chill index | `meteorology::winds` |

### 2️⃣ New Submodule — `storms.rs` — Severe Weather & Cyclone Dynamics (5 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `potential_intensity` | `fn potential_intensity(ck: f64, cd: f64, eta: f64, delta_k: f64) → f64` | $V_{max} = \sqrt{(C_k/C_d)\,\eta\,\Delta k}$ | `meteorology::storms` |
| `accumulated_cyclone_energy` | `fn accumulated_cyclone_energy(v_kt_series: &[f64]) → f64` | $ACE = \sum v_{kt}^2 / 10^4$ | `meteorology::storms` |
| `cape` | `fn cape(t_parcel: f64, t_env: f64, g: f64, dz: f64) → f64` | $CAPE = g(T_p - T_e)/T_e \cdot\Delta z$ | `meteorology::storms` |
| `rossby_deformation_radius` | `fn rossby_deformation_radius(n: f64, h: f64, f: f64) → f64` | $L_d = NH/f$ | `meteorology::storms` |
| `fujita_scale` | `fn fujita_scale(v: f64) → u8` | F-scale category from wind speed (m/s) | `meteorology::storms` |

### 3️⃣ New Function — `atmosphere.rs`

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `rayleigh_phase` | `fn rayleigh_phase(cos_theta: f64) → f64` | $P(\cos\theta) = \frac{3}{4}(1 + \cos^2\theta)$ | `meteorology::atmosphere` |

---

## v0.0.3

### 1️⃣ New Submodules

#### ☁️ clouds.rs — Cloud Microphysics & Radiative Properties (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `saturation_vapor_pressure` | `fn saturation_vapor_pressure(temperature_k: f64) → f64` | $e_s = 6.112 \cdot e^{\frac{17.67(T-273.15)}{T-29.65}}$ | `meteorology::clouds` |
| `cloud_base_altitude` | `fn cloud_base_altitude(surface_temperature: f64, dew_point: f64, lapse_rate: f64) → f64` | $z_{base} = \frac{T_{sfc} - T_d}{\Gamma}$ | `meteorology::clouds` |
| `adiabatic_liquid_water_content` | `fn adiabatic_liquid_water_content(altitude_above_base: f64, lapse_rate: f64, temperature_at_base: f64) → f64` | Adiabatic LWC from altitude above cloud base | `meteorology::clouds` |
| `cloud_optical_depth` | `fn cloud_optical_depth(liquid_water_path: f64, effective_radius: f64) → f64` | $\tau = \frac{3 \cdot LWP}{2 \cdot \rho_w \cdot r_e}$ | `meteorology::clouds` |
| `cloud_albedo` | `fn cloud_albedo(optical_depth: f64) → f64` | $A = \frac{\tau}{\tau + 7.7}$ (two-stream approximation) | `meteorology::clouds` |
| `droplet_growth_condensation` | `fn droplet_growth_condensation(supersaturation: f64, radius: f64, temperature: f64, pressure: f64) → f64` | Condensational growth rate $\frac{dr}{dt}$ | `meteorology::clouds` |
| `droplet_growth_collision` | `fn droplet_growth_collision(collector_radius: f64, collected_radius: f64, liquid_water_content: f64, collection_efficiency: f64, density_water: f64, terminal_velocity_diff: f64) → f64` | Collision-coalescence growth rate | `meteorology::clouds` |
| `autoconversion_rate` | `fn autoconversion_rate(lwc: f64, droplet_concentration: f64, threshold_lwc: f64) → f64` | Cloud-to-rain autoconversion (Kessler-type) | `meteorology::clouds` |
| `ice_crystal_growth_rate` | `fn ice_crystal_growth_rate(temperature_k: f64, supersaturation_ice: f64) → f64` | Ice crystal depositional growth | `meteorology::clouds` |
| `bergeron_process_rate` | `fn bergeron_process_rate(ice_crystal_density: f64, lwc: f64, temperature_k: f64) → f64` | Bergeron–Findeisen ice-water interaction | `meteorology::clouds` |
| `cloud_layer_temperature` | `fn cloud_layer_temperature(base_temperature: f64, lapse_rate: f64, altitude_above_base: f64) → f64` | $T = T_{base} - \Gamma \cdot \Delta z$ | `meteorology::clouds` |
| `mie_size_parameter` | `fn mie_size_parameter(particle_radius: f64, wavelength: f64) → f64` | $x = \frac{2\pi r}{\lambda}$ | `meteorology::clouds` |
| `extinction_efficiency` | `fn extinction_efficiency(size_parameter: f64) → f64` | Mie extinction efficiency $Q_{ext}$ | `meteorology::clouds` |
| `cloud_emissivity` | `fn cloud_emissivity(optical_depth: f64) → f64` | $\varepsilon = 1 - e^{-\tau}$ | `meteorology::clouds` |
| `convective_available_potential_energy` | `fn convective_available_potential_energy(parcel_temperatures: &[f64], env_temperatures: &[f64], altitudes: &[f64], g_local: f64) → f64` | $CAPE = \int_{LFC}^{EL} g \frac{T_p - T_e}{T_e} dz$ | `meteorology::clouds` |
| `convective_inhibition` | `fn convective_inhibition(parcel_temperatures: &[f64], env_temperatures: &[f64], altitudes: &[f64], g_local: f64) → f64` | $CIN = -\int_{sfc}^{LFC} g \frac{T_p - T_e}{T_e} dz$ | `meteorology::clouds` |
| `henyey_greenstein_phase` | `fn henyey_greenstein_phase(cos_theta: f64, asymmetry: f64) → f64` | $P(\cos\theta) = \frac{1 - g^2}{(1 + g^2 - 2g\cos\theta)^{3/2}}$ | `meteorology::clouds` |
| `cloud_radiative_forcing` | `fn cloud_radiative_forcing(cloud_albedo: f64, surface_albedo: f64, solar_flux: f64, cloud_fraction: f64, longwave_trapping: f64) → f64` | Net CRF (shortwave cooling + longwave warming) | `meteorology::clouds` |
| `nucleation_rate_ccn` | `fn nucleation_rate_ccn(supersaturation: f64, ccn_concentration: f64, k_exponent: f64) → f64` | $N_{act} = C \cdot S^k$ (CCN activation spectrum) | `meteorology::clouds` |
| `rain_evaporation_rate` | `fn rain_evaporation_rate(rain_rate: f64, relative_humidity: f64, temperature: f64) → f64` | Sub-cloud rain evaporation rate | `meteorology::clouds` |
| `terminal_velocity_droplet` | `fn terminal_velocity_droplet(radius_m: f64) → f64` | Terminal fall speed of cloud/rain droplet | `meteorology::clouds` |

#### 🌊 ocean.rs — Physical Oceanography (26 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `deep_water_phase_speed` | `fn deep_water_phase_speed(g_local: f64, wavelength: f64) → f64` | $c = \sqrt{\frac{g\lambda}{2\pi}}$ | `meteorology::ocean` |
| `shallow_water_phase_speed` | `fn shallow_water_phase_speed(g_local: f64, depth: f64) → f64` | $c = \sqrt{gd}$ | `meteorology::ocean` |
| `wave_dispersion` | `fn wave_dispersion(g_local: f64, k: f64, depth: f64) → f64` | $\omega^2 = gk\tanh(kd)$ | `meteorology::ocean` |
| `phillips_spectrum` | `fn phillips_spectrum(k: f64, wind_speed: f64, g_local: f64, wind_direction_x: f64, wind_direction_y: f64, kx: f64, ky: f64) → f64` | Phillips wave energy spectrum | `meteorology::ocean` |
| `jonswap_spectrum` | `fn jonswap_spectrum(omega: f64, wind_speed: f64, fetch: f64, g_local: f64) → f64` | JONSWAP wave energy spectrum | `meteorology::ocean` |
| `significant_wave_height` | `fn significant_wave_height(wind_speed: f64, fetch: f64, g_local: f64) → f64` | $H_s$ from wind speed and fetch | `meteorology::ocean` |
| `wave_period` | `fn wave_period(wind_speed: f64, fetch: f64, g_local: f64) → f64` | Peak wave period from wind and fetch | `meteorology::ocean` |
| `stokes_drift` | `fn stokes_drift(amplitude: f64, omega: f64, k: f64, depth: f64, z: f64) → f64` | $u_S = \omega k a^2 \frac{\cosh 2k(z+d)}{2\sinh^2(kd)}$ | `meteorology::ocean` |
| `ekman_transport` | `fn ekman_transport(wind_stress: f64, coriolis_parameter: f64, density: f64) → f64` | $M_E = \frac{\tau}{\rho f}$ | `meteorology::ocean` |
| `ekman_spiral_velocity` | `fn ekman_spiral_velocity(surface_current: f64, ekman_depth: f64, z: f64) → (f64, f64)` | Ekman spiral $(u, v)$ velocity profile | `meteorology::ocean` |
| `thermohaline_density` | `fn thermohaline_density(rho_0: f64, alpha_t: f64, beta_s: f64, temperature: f64, t_ref: f64, salinity: f64, s_ref: f64) → f64` | $\rho = \rho_0[1 - \alpha_T(T - T_0) + \beta_S(S - S_0)]$ | `meteorology::ocean` |
| `overturning_stream_function` | `fn overturning_stream_function(thermal_forcing: f64, salinity_forcing: f64, alpha_t: f64, beta_s: f64, kappa: f64, depth: f64) → f64` | Thermohaline overturning circulation | `meteorology::ocean` |
| `mixed_layer_depth` | `fn mixed_layer_depth(wind_stress: f64, buoyancy_flux: f64, coriolis: f64) → f64` | Ocean mixed layer depth estimate | `meteorology::ocean` |
| `tidal_amplitude` | `fn tidal_amplitude(mass_perturber: f64, distance: f64, body_radius: f64, ocean_depth: f64, g_surface: f64) → f64` | Tidal amplitude from gravitational forcing | `meteorology::ocean` |
| `geostrophic_current` | `fn geostrophic_current(g_local: f64, sea_surface_slope: f64, coriolis: f64) → f64` | $u_g = -\frac{g}{f}\frac{\partial \eta}{\partial y}$ | `meteorology::ocean` |
| `upwelling_velocity` | `fn upwelling_velocity(wind_stress_curl: f64, coriolis: f64, density: f64) → f64` | $w = \frac{\nabla \times \tau}{\rho f}$ (Ekman pumping) | `meteorology::ocean` |
| `ocean_heat_content` | `fn ocean_heat_content(density: f64, specific_heat: f64, delta_t: f64, depth: f64, area: f64) → f64` | $Q = \rho c_p \Delta T \cdot d \cdot A$ | `meteorology::ocean` |
| `sea_ice_growth_rate` | `fn sea_ice_growth_rate(thermal_conductivity_ice: f64, ice_thickness: f64, surface_temp: f64, freezing_point: f64, latent_heat: f64, ice_density: f64) → f64` | Stefan ice growth law | `meteorology::ocean` |
| `internal_wave_speed` | `fn internal_wave_speed(g_local: f64, density_upper: f64, density_lower: f64, depth_upper: f64, depth_lower: f64) → f64` | $c = \sqrt{g' \frac{h_1 h_2}{h_1 + h_2}}$ (two-layer) | `meteorology::ocean` |
| `langmuir_circulation_depth` | `fn langmuir_circulation_depth(wind_speed: f64, surface_current: f64) → f64` | Langmuir cell penetration depth | `meteorology::ocean` |
| `seawater_density` | `fn seawater_density(t: f64, s: f64) → f64` | UNESCO equation of state (simplified) | `meteorology::ocean` |
| `sound_speed` | `fn sound_speed(t: f64, s: f64, d: f64) → f64` | Mackenzie equation for sound in seawater | `meteorology::ocean` |
| `sea_level_rise_thermal` | `fn sea_level_rise_thermal(alpha: f64, delta_t: f64, d: f64) → f64` | $\Delta h = \alpha \cdot \Delta T \cdot d$ | `meteorology::ocean` |
| `henry_solubility` | `fn henry_solubility(kh: f64, t: f64, delta_h: f64) → f64` | $K_H(T) = K_H^0 \exp\left[\frac{-\Delta H}{R}\left(\frac{1}{T}-\frac{1}{T_0}\right)\right]$ | `meteorology::ocean` |
| `haline_contraction_coefficient` | `fn haline_contraction_coefficient(salinity_psu: f64) → f64` | $\beta_S$ haline contraction coefficient | `meteorology::ocean` |
| `thermal_expansion_coefficient_sw` | `fn thermal_expansion_coefficient_sw(temperature_c: f64) → f64` | $\alpha_T$ thermal expansion for seawater | `meteorology::ocean` |

#### ⛈️ storms.rs — Severe Weather (5 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `potential_intensity` | `fn potential_intensity(ck: f64, cd: f64, eta: f64, delta_k: f64) → f64` | $V_{max} = \sqrt{\frac{C_k}{C_d} \eta \Delta k}$ (Emanuel PI) | `meteorology::storms` |
| `accumulated_cyclone_energy` | `fn accumulated_cyclone_energy(v_kt_series: &[f64]) → f64` | $ACE = \sum v_{max}^2 \times 10^{-4}$ | `meteorology::storms` |
| `cape` | `fn cape(t_parcel: f64, t_env: f64, g: f64, dz: f64) → f64` | $CAPE = g \frac{T_p - T_e}{T_e} \Delta z$ (single layer) | `meteorology::storms` |
| `rossby_deformation_radius` | `fn rossby_deformation_radius(n: f64, h: f64, f: f64) → f64` | $L_R = \frac{NH}{f}$ | `meteorology::storms` |
| `fujita_scale` | `fn fujita_scale(v: f64) → u8` | Enhanced Fujita scale classification (EF0–EF5) | `meteorology::storms` |

#### 💨 winds.rs — Global & Mesoscale Wind Dynamics (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hadley_cell_extent` | `fn hadley_cell_extent(planet_radius: f64, rotation_rate: f64, delta_t: f64) → f64` | $\phi_H \approx \left(\frac{5 \Delta\theta g H}{3 \Omega^2 a^2 \theta_0}\right)^{1/2}$ (Held–Hou) | `meteorology::winds` |
| `thermal_wind_shear` | `fn thermal_wind_shear(g_local: f64, coriolis: f64, temperature: f64, dt_dy: f64) → f64` | $\frac{\partial u}{\partial z} = -\frac{g}{fT}\frac{\partial T}{\partial y}$ | `meteorology::winds` |
| `jet_stream_velocity` | `fn jet_stream_velocity(g_local: f64, delta_t: f64, meridional_distance: f64, coriolis: f64, scale_height: f64) → f64` | Jet stream thermal wind velocity | `meteorology::winds` |
| `surface_wind_speed` | `fn surface_wind_speed(pressure_gradient: f64, density: f64, drag_coefficient: f64) → f64` | Surface wind from pressure gradient force | `meteorology::winds` |
| `wind_stress` | `fn wind_stress(air_density: f64, drag_coefficient: f64, wind_speed: f64) → f64` | $\tau = \rho C_D U^2$ | `meteorology::winds` |
| `planetary_vorticity` | `fn planetary_vorticity(rotation_rate: f64, latitude: f64) → f64` | $f = 2\Omega\sin\phi$ | `meteorology::winds` |
| `rossby_wave_phase_speed` | `fn rossby_wave_phase_speed(beta: f64, zonal_wavenumber: f64, deformation_radius: f64) → f64` | $c = -\frac{\beta}{k^2 + L_R^{-2}}$ | `meteorology::winds` |
| `beta_parameter` | `fn beta_parameter(rotation_rate: f64, planet_radius: f64, latitude: f64) → f64` | $\beta = \frac{2\Omega\cos\phi}{a}$ | `meteorology::winds` |
| `baroclinic_instability_wavelength` | `fn baroclinic_instability_wavelength(deformation_radius: f64) → f64` | $\lambda = 2\pi \cdot 3.9 \cdot L_R$ (Eady) | `meteorology::winds` |
| `sea_breeze_speed` | `fn sea_breeze_speed(g_local: f64, delta_t: f64, boundary_layer_height: f64, mean_temperature: f64) → f64` | $u = \sqrt{g \frac{\Delta T}{T_m} h}$ | `meteorology::winds` |
| `katabatic_wind_speed` | `fn katabatic_wind_speed(g_local: f64, delta_t: f64, mean_temperature: f64, slope_angle: f64, slope_length: f64, drag_coefficient: f64) → f64` | Katabatic (downslope) gravity wind | `meteorology::winds` |
| `mountain_wave_vertical_velocity` | `fn mountain_wave_vertical_velocity(wind_speed: f64, mountain_height: f64, brunt_vaisala: f64, horizontal_wavelength: f64) → f64` | Lee wave vertical velocity | `meteorology::winds` |
| `ekman_pumping_velocity` | `fn ekman_pumping_velocity(wind_stress_curl: f64, density: f64, coriolis: f64) → f64` | $w_E = \frac{\nabla \times \tau}{\rho f}$ | `meteorology::winds` |
| `monin_obukhov_length` | `fn monin_obukhov_length(friction_velocity: f64, surface_temp: f64, sensible_heat_flux: f64, air_density: f64, specific_heat: f64) → f64` | $L = -\frac{u_*^3 \theta \rho c_p}{\kappa g H}$ | `meteorology::winds` |
| `log_wind_profile` | `fn log_wind_profile(friction_velocity: f64, z: f64, roughness_length: f64) → f64` | $u(z) = \frac{u_*}{\kappa}\ln\frac{z}{z_0}$ | `meteorology::winds` |
| `cyclostrophic_wind` | `fn cyclostrophic_wind(pressure_gradient: f64, density: f64, radius: f64) → f64` | $V = \sqrt{\frac{r}{\rho}\frac{\partial p}{\partial r}}$ | `meteorology::winds` |
| `gradient_wind` | `fn gradient_wind(coriolis: f64, pressure_gradient: f64, density: f64, radius: f64) → f64` | $\frac{V^2}{r} + fV = \frac{1}{\rho}\frac{\partial p}{\partial r}$ | `meteorology::winds` |
| `superrotation_index` | `fn superrotation_index(zonal_wind: f64, planet_radius: f64, rotation_rate: f64, latitude: f64) → f64` | $S = \frac{u}{\Omega a \cos\phi}$ | `meteorology::winds` |
| `foehn_warming` | `fn foehn_warming(lapse_dry: f64, lapse_moist: f64, mountain_height: f64, condensation_level: f64) → f64` | Foehn temperature gain (dry vs moist lapse) | `meteorology::winds` |
| `beaufort_to_m_s` | `fn beaufort_to_m_s(b: f64) → f64` | $v = 0.836 \cdot B^{3/2}$ (Beaufort scale) | `meteorology::winds` |
| `wind_chill` | `fn wind_chill(t: f64, v: f64) → f64` | NWS wind chill temperature index | `meteorology::winds` |

### 2️⃣ Changes to Existing Submodules

#### 🌡️ atmosphere.rs — Added Functions (+5 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `mie_phase` | `fn mie_phase(cos_theta: f64, g: f64) → f64` | Henyey-Greenstein phase function for Mie scattering | `meteorology::atmosphere` |
| `lifted_condensation_level` | `fn lifted_condensation_level(t_surface: f64, dew_point: f64) → f64` | $LCL \approx \frac{T_{sfc} - T_d}{8} \times 1000$ | `meteorology::atmosphere` |
| `dry_adiabatic_temperature` | `fn dry_adiabatic_temperature(t_surface: f64, altitude: f64) → f64` | $T(z) = T_{sfc} - \Gamma_d \cdot z$ | `meteorology::atmosphere` |
| `convective_available_potential_energy` | `fn convective_available_potential_energy(t_parcel: &[f64], t_env: &[f64], dz: f64) → f64` | $CAPE = \int_{LFC}^{EL} g \frac{T_p - T_e}{T_e} dz$ | `meteorology::atmosphere` |
| `convective_inhibition` | `fn convective_inhibition(t_parcel: &[f64], t_env: &[f64], dz: f64) → f64` | $CIN = -\int_{sfc}^{LFC} g \frac{T_p - T_e}{T_e} dz$ | `meteorology::atmosphere` |

#### 🌀 dynamics.rs — Added Functions (+2 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `rossby_deformation_radius` | `fn rossby_deformation_radius(n: f64, h: f64, f: f64) → f64` | $L_R = \frac{NH}{f}$ | `meteorology::dynamics` |
| `cyclone_gradient_wind` | `fn cyclone_gradient_wind(r: f64, f: f64, dp_dr: f64, rho: f64) → f64` | Gradient wind balance in cyclone | `meteorology::dynamics` |

#### 🌧️ precipitation.rs — Added Functions (+7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `thornthwaite_pet` | `fn thornthwaite_pet(t_mean: f64, heat_index: f64, day_length_hours: f64) → f64` | Thornthwaite potential evapotranspiration | `meteorology::precipitation` |
| `penman_evaporation` | `fn penman_evaporation(delta: f64, rn: f64, gamma: f64, ea: f64, u: f64) → f64` | $E = \frac{\Delta R_n + \gamma E_a}{\Delta + \gamma}$ (Penman) | `meteorology::precipitation` |
| `intensity_duration_frequency` | `fn intensity_duration_frequency(a: f64, b: f64, duration: f64, return_period: f64) → f64` | IDF curve intensity estimate | `meteorology::precipitation` |
| `scs_curve_number_runoff` | `fn scs_curve_number_runoff(p: f64, cn: f64) → f64` | $Q = \frac{(P - 0.2S)^2}{P + 0.8S}$ (SCS-CN) | `meteorology::precipitation` |
| `rational_method_runoff` | `fn rational_method_runoff(c: f64, i: f64, a: f64) → f64` | $Q = CiA$ (rational method) | `meteorology::precipitation` |
| `unit_hydrograph_peak` | `fn unit_hydrograph_peak(a: f64, tp: f64) → f64` | Unit hydrograph peak discharge | `meteorology::precipitation` |
| `antecedent_precipitation_index` | `fn antecedent_precipitation_index(prev_api: f64, rainfall: f64, k: f64) → f64` | $API_t = k \cdot API_{t-1} + P_t$ | `meteorology::precipitation` |

### 3️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 23 → 46 (+23) |
| Test files | 4 → 8 |
| Total suite | 814 tests, 0 warnings |

Test files: `meteorology/main.rs`, `meteorology/atmosphere.rs`, `meteorology/dynamics.rs`, `meteorology/radiation.rs`, `meteorology/ocean.rs`, `meteorology/precipitation.rs`, `meteorology/storms.rs`, `meteorology/winds.rs`

---

## v0.0.2

### 1️⃣ Changes

Structure identical to v0.0.1 (4 submodules). All 4 source files modified (`atmosphere.rs`, `dynamics.rs`, `precipitation.rs`, `radiation.rs`).

### 2️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 0 → 23 |
| Test files | 4 |

| File | Tests |
|---|---|
| `meteorology/dynamics.rs` | 8 |
| `meteorology/atmosphere.rs` | 7 |
| `meteorology/radiation.rs` | 7 |
| `meteorology/main.rs` | 1 |

---

## v0.0.1

Module: `src/meteorology/` — 4 submodules, 35 pub fn

### 1️⃣ atmosphere.rs — Atmospheric Thermodynamics (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `barometric_formula` | `fn barometric_formula(p0: f64, m: f64, g: f64, h: f64, t: f64) → f64` | $P = P_0 \cdot e^{-\frac{mgh}{kT}}$ | `meteorology::atmosphere` |
| `scale_height` | `fn scale_height(t: f64, m: f64, g: f64) → f64` | $H = \frac{kT}{mg}$ | `meteorology::atmosphere` |
| `lapse_rate_dry` | `fn lapse_rate_dry() → f64` | $\Gamma_d = \frac{g}{c_p} \approx 9.8$ K/km | `meteorology::atmosphere` |
| `lapse_rate_moist` | `fn lapse_rate_moist(t: f64, l_v: f64, r_s: f64) → f64` | $\Gamma_m = \Gamma_d \frac{1 + \frac{L_v r_s}{R_d T}}{1 + \frac{L_v^2 r_s}{c_p R_v T^2}}$ | `meteorology::atmosphere` |
| `potential_temperature` | `fn potential_temperature(t: f64, p: f64, p0: f64) → f64` | $\theta = T \left(\frac{P_0}{P}\right)^{R/c_p}$ | `meteorology::atmosphere` |
| `virtual_temperature` | `fn virtual_temperature(t: f64, r: f64) → f64` | $T_v = T(1 + 0.61r)$ | `meteorology::atmosphere` |
| `mixing_ratio` | `fn mixing_ratio(e: f64, p: f64) → f64` | $r = \frac{\epsilon \cdot e}{P - e}$ | `meteorology::atmosphere` |
| `saturation_vapor_pressure` | `fn saturation_vapor_pressure(t_celsius: f64) → f64` | $e_s = 6.112 \cdot e^{\frac{17.67T}{T + 243.5}}$ (Magnus) | `meteorology::atmosphere` |
| `relative_humidity` | `fn relative_humidity(e: f64, es: f64) → f64` | $RH = \frac{e}{e_s} \times 100$ | `meteorology::atmosphere` |
| `dew_point` | `fn dew_point(t: f64, rh: f64) → f64` | Inverse Magnus formula | `meteorology::atmosphere` |
| `density_altitude` | `fn density_altitude(pressure_altitude: f64, temperature_c: f64) → f64` | $DA = PA + 120(T - T_{ISA})$ | `meteorology::atmosphere` |
| `brunt_vaisala_frequency` | `fn brunt_vaisala_frequency(g: f64, theta: f64, dtheta_dz: f64) → f64` | $N = \sqrt{\frac{g}{\theta}\frac{d\theta}{dz}}$ | `meteorology::atmosphere` |

### 2️⃣ dynamics.rs — Atmospheric Dynamics (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `coriolis_parameter` | `fn coriolis_parameter(latitude: f64, omega: f64) → f64` | $f = 2\Omega\sin\phi$ | `meteorology::dynamics` |
| `geostrophic_wind` | `fn geostrophic_wind(dp_dx: f64, dp_dy: f64, rho: f64, f: f64) → (f64, f64)` | $u_g = -\frac{1}{\rho f}\frac{\partial p}{\partial y}$, $v_g = \frac{1}{\rho f}\frac{\partial p}{\partial x}$ | `meteorology::dynamics` |
| `rossby_number` | `fn rossby_number(u: f64, l: f64, f: f64) → f64` | $Ro = \frac{U}{fL}$ | `meteorology::dynamics` |
| `rossby_wave_speed` | `fn rossby_wave_speed(beta: f64, k: f64) → f64` | $c = -\frac{\beta}{k^2}$ | `meteorology::dynamics` |
| `thermal_wind` | `fn thermal_wind(f: f64, delta_t: f64, delta_x: f64, t_mean: f64) → f64` | $\frac{\partial u_g}{\partial \ln p} = -\frac{R}{f}\frac{\partial T}{\partial y}$ | `meteorology::dynamics` |
| `potential_vorticity` | `fn potential_vorticity(f: f64, dtheta_dp: f64) → f64` | $PV = -g(f + \zeta)\frac{\partial\theta}{\partial p}$ | `meteorology::dynamics` |
| `ekman_depth` | `fn ekman_depth(nu: f64, f: f64) → f64` | $D_E = \sqrt{\frac{2\nu}{f}}$ | `meteorology::dynamics` |
| `richardson_number` | `fn richardson_number(n2: f64, du_dz: f64) → f64` | $Ri = \frac{N^2}{(du/dz)^2}$ | `meteorology::dynamics` |

### 3️⃣ precipitation.rs — Precipitation & Hydrology (3 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `rain_rate_marshall_palmer` | `fn rain_rate_marshall_palmer(z: f64) → f64` | $Z = 200 R^{1.6}$ (Marshall-Palmer) | `meteorology::precipitation` |
| `radar_reflectivity` | `fn radar_reflectivity(rain_rate: f64) → f64` | $Z = 200 R^{1.6}$ (inverse) | `meteorology::precipitation` |
| `terminal_velocity_raindrop` | `fn terminal_velocity_raindrop(diameter_mm: f64) → f64` | Terminal fall velocity of raindrops | `meteorology::precipitation` |

### 4️⃣ radiation.rs — Radiative Transfer (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `stefan_boltzmann_flux` | `fn stefan_boltzmann_flux(t: f64) → f64` | $F = \sigma T^4$ | `meteorology::radiation` |
| `solar_constant` | `fn solar_constant() → f64` | $S_0 \approx 1361$ W/m² | `meteorology::radiation` |
| `albedo_reflected` | `fn albedo_reflected(solar_flux: f64, albedo: f64) → f64` | $F_{ref} = \alpha \cdot S$ | `meteorology::radiation` |
| `effective_temperature` | `fn effective_temperature(solar_flux: f64, albedo: f64) → f64` | $T_e = \left[\frac{S(1-\alpha)}{4\sigma}\right]^{1/4}$ | `meteorology::radiation` |
| `greenhouse_effect` | `fn greenhouse_effect(t_surface: f64, t_effective: f64) → f64` | $\Delta T = T_s - T_e$ | `meteorology::radiation` |
| `optical_depth` | `fn optical_depth(absorption_coeff: f64, path_length: f64) → f64` | $\tau = \kappa \cdot l$ | `meteorology::radiation` |
| `beer_lambert` | `fn beer_lambert(i0: f64, tau: f64) → f64` | $I = I_0 e^{-\tau}$ | `meteorology::radiation` |
| `planck_function` | `fn planck_function(wavelength: f64, t: f64) → f64` | $B_\lambda = \frac{2hc^2}{\lambda^5}\frac{1}{e^{hc/\lambda k_BT}-1}$ | `meteorology::radiation` |
| `solar_zenith_angle` | `fn solar_zenith_angle(lat: f64, declination: f64, hour_angle: f64) → f64` | $\cos\theta_z = \sin\phi\sin\delta + \cos\phi\cos\delta\cos h$ | `meteorology::radiation` |
| `radiative_forcing_co2` | `fn radiative_forcing_co2(c: f64, c0: f64) → f64` | $\Delta F = 5.35 \ln\frac{C}{C_0}$ | `meteorology::radiation` |
| `climate_sensitivity` | `fn climate_sensitivity(delta_t: f64, delta_f: f64) → f64` | $\lambda = \frac{\Delta T}{\Delta F}$ | `meteorology::radiation` |
| `outgoing_longwave_radiation` | `fn outgoing_longwave_radiation(t: f64, emissivity: f64) → f64` | $OLR = \varepsilon\sigma T^4$ | `meteorology::radiation` |
