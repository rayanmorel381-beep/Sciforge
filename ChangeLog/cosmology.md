# Cosmology Expansion — ChangeLog

---

## v0.0.3

### Expansion — `astronomy::cosmology` — 91 → 124 pub fn (+33)

Replaced hardcoded local constants (`C_KM_S`, `G_SI`, `MPC_IN_M`) with `use crate::constants::*`.

No new functions — see `testing.md` for test details.

### 1️⃣ New — Hubble Utilities — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hubble distance (Mpc) | `hubble_distance_mpc(h0: f64) → f64` | $d_H = c/H_0$ (Mpc) | `astronomy::cosmology` |
| Hubble time (Gyr) | `hubble_time_gyr(h0: f64) → f64` | $t_H = 1/H_0$ (Gyr) | `astronomy::cosmology` |
| Curvature parameter | `omega_k_from(omega_m: f64, omega_r: f64, omega_de: f64) → f64` | $\Omega_k = 1 - \Omega_m - \Omega_r - \Omega_\Lambda$ | `astronomy::cosmology` |
| Little h | `little_h(h0: f64) → f64` | $h = H_0 / 100$ | `astronomy::cosmology` |

### 2️⃣ New — Deceleration & ΛCDM — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Deceleration parameter | `deceleration_parameter(omega_m: f64, z: f64) → f64` | $q(z) = \Omega_m(z)/2 - \Omega_\Lambda(z)$ | `astronomy::cosmology` |
| Deceleration today | `deceleration_parameter_today(omega_m: f64) → f64` | $q_0 = \Omega_m/2 - (1 - \Omega_m)$ | `astronomy::cosmology` |
| $E(z)$ ΛCDM + radiation | `e_z_lcdm_rad(omega_m: f64, omega_r: f64, z: f64) → f64` | $E(z) = \sqrt{\Omega_r(1+z)^4 + \Omega_m(1+z)^3 + \Omega_\Lambda}$ | `astronomy::cosmology` |
| $H(z)$ ΛCDM | `hubble_at_z_lcdm(h0: f64, omega_m: f64, z: f64) → f64` | $H(z) = H_0 E(z)$ | `astronomy::cosmology` |

### 3️⃣ New — Distances from $z$ (ΛCDM) — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Comoving distance from $z$ | `comoving_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_C = \int_0^z dz'/E(z')$ | `astronomy::cosmology` |
| Luminosity distance from $z$ | `luminosity_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_L = d_C(1 + z)$ | `astronomy::cosmology` |
| Angular diameter distance from $z$ | `angular_diameter_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_A = d_C/(1 + z)$ | `astronomy::cosmology` |
| Distance modulus from $z$ | `distance_modulus_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $\mu = 5\log_{10}(d_L/10\,\text{pc})$ | `astronomy::cosmology` |

### 4️⃣ New — Distances (General) — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Comoving distance (general) | `comoving_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General cosmology integral | `astronomy::cosmology` |
| Transverse comoving distance | `transverse_comoving_distance(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | Curved geometry correction | `astronomy::cosmology` |
| Luminosity distance (general) | `luminosity_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $d_L$ | `astronomy::cosmology` |
| Angular diameter distance (general) | `angular_diameter_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $d_A$ | `astronomy::cosmology` |
| Distance modulus (general) | `distance_modulus_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $\mu$ | `astronomy::cosmology` |

### 5️⃣ New — Distances (wCDM / w₀wₐ) — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Comoving distance wCDM | `comoving_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | Constant $w$ dark energy | `astronomy::cosmology` |
| Luminosity distance wCDM | `luminosity_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | $d_L$ with constant $w$ | `astronomy::cosmology` |
| Comoving distance $w_0 w_a$ | `comoving_distance_w0wa(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | CPL parametrization | `astronomy::cosmology` |
| Luminosity distance $w_0 w_a$ | `luminosity_distance_w0wa(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | $d_L$ with CPL | `astronomy::cosmology` |

### 6️⃣ New — Distance Relations — 3 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| $d_L$ from $d_A$ | `luminosity_from_angular_diameter(d_a: f64, z: f64) → f64` | $d_L = d_A(1+z)^2$ | `astronomy::cosmology` |
| $d_A$ from $d_L$ | `angular_diameter_from_luminosity(d_l: f64, z: f64) → f64` | $d_A = d_L/(1+z)^2$ | `astronomy::cosmology` |
| Proper distance | `proper_distance(comoving_d: f64, z: f64) → f64` | $d_P = d_C / (1+z)$ | `astronomy::cosmology` |

### 7️⃣ New — Lookback & Densities — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Lookback time from $z$ | `lookback_time_from_z(h0: f64, omega_m: f64, z: f64) → f64` | ΛCDM lookback integration | `astronomy::cosmology` |
| Baryon density at $z$ | `matter_density_at_z(h0: f64, omega_m: f64, z: f64) → f64` | $\rho_m(z) = \rho_{c,0}\Omega_m(1+z)^3$ | `astronomy::cosmology` |
| Cosmological constant density | `dark_energy_density_today() → f64` | $\rho_\Lambda$ | `astronomy::cosmology` |
| Saha ionization fraction | `saha_ionization_fraction(temperature: f64, n_b: f64) → f64` | Saha equation for recombination | `astronomy::cosmology` |

### 8️⃣ New — Dark Matter Relics — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| WIMP annihilation rate | `wimp_annihilation_rate(rho: f64, mass_gev: f64) → f64` | $\Gamma = \langle\sigma v\rangle \rho^2 / m^2$ | `astronomy::cosmology` |
| WIMP direct detection rate | `wimp_direct_detection_rate(rho: f64, mass_gev: f64, sigma_cm2: f64, v_mean: f64) → f64` | Scattering event rate | `astronomy::cosmology` |
| DM annihilation luminosity | `dm_annihilation_luminosity(rho: f64, mass_gev: f64, volume: f64) → f64` | Volume-integrated luminosity | `astronomy::cosmology` |
| Axion Compton frequency | `axion_compton_frequency(mass_ev: f64) → f64` | $\nu = mc^2/h$ | `astronomy::cosmology` |
| Axion de Broglie wavelength | `axion_de_broglie_wavelength(mass_ev: f64, velocity: f64) → f64` | $\lambda = h/(mv)$ | `astronomy::cosmology` |

---

## v0.0.2

### Expansion — `astronomy::cosmology` — 12 → 91 pub fn (+79)

Added Simpson numerical integration (`quad`), local constants (`C_KM_S`, `G_SI`, `MPC_IN_M`).

### 1️⃣ New — Friedmann & Normalized Hubble — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Generic $E(z)$ | `e_z(omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | $E(z) = \sqrt{\sum \Omega_i(1+z)^{n_i}}$ | `astronomy::cosmology` |
| $E(z)$ ΛCDM | `e_z_lcdm(omega_m: f64, z: f64) → f64` | $E(z) = \sqrt{\Omega_m(1+z)^3 + \Omega_\Lambda}$ | `astronomy::cosmology` |
| $E(z)$ wCDM | `e_z_wcdm(omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | Constant $w$ dark energy | `astronomy::cosmology` |
| $E(z)$ $w_0 w_a$ | `e_z_w0wa(omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | CPL parametrization | `astronomy::cosmology` |
| $H(z)$ generic | `hubble_at_z(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | $H(z) = H_0 E(z)$ | `astronomy::cosmology` |

### 2️⃣ New — Distance Modulus — 1 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Distance modulus | `distance_modulus_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $\mu = 5\log_{10}(d_L) + 25$ | `astronomy::cosmology` |

### 3️⃣ New — Dark Energy Models — 3 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Friedmann $H^2$ | `friedmann_expansion(h0: f64, omega_m: f64, omega_r: f64, omega_lambda: f64, a: f64) → f64` | Full Friedmann equation | `astronomy::cosmology` |
| wCDM equation of state | `wcdm_equation_of_state(omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | DE density with constant $w$ | `astronomy::cosmology` |
| $w_0 w_a$ equation of state | `w0wa_equation_of_state(omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | Chevallier-Polarski-Linder | `astronomy::cosmology` |

### 4️⃣ New — BAO & Sound Horizon — 2 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Sound horizon | `sound_horizon_eh98(omega_m: f64, omega_b: f64, h0: f64) → f64` | Eisenstein-Hu 1998 fit | `astronomy::cosmology` |
| Drag epoch redshift | `acceleration_redshift(omega_m: f64) → f64` | $z_{\text{acc}}$ where $q(z) = 0$ | `astronomy::cosmology` |

### 5️⃣ New — Gravitational Lensing — 2 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Einstein radius | `einstein_radius(h0: f64, omega_m: f64, mass: f64, z_l: f64, z_s: f64) → f64` | $\theta_E = \sqrt{4GM D_{ls}/(c^2 D_l D_s)}$ | `astronomy::cosmology` |
| Critical surface density | `critical_surface_density(h0: f64, omega_m: f64, z_l: f64, z_s: f64) → f64` | $\Sigma_{\text{cr}} = c^2 D_s/(4\pi G D_l D_{ls})$ | `astronomy::cosmology` |

### 6️⃣ New — NFW Profile — 3 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| NFW density | `nfw_density_profile(r_kpc: f64) → f64` | $\rho(r) = \rho_s / [(r/r_s)(1+r/r_s)^2]$ | `astronomy::cosmology` |
| NFW enclosed mass | `nfw_enclosed_mass(r_kpc: f64) → f64` | $M(r) = 4\pi\rho_s r_s^3 [\ln(1+c) - c/(1+c)]$ | `astronomy::cosmology` |
| NFW circular velocity | `nfw_circular_velocity(r_kpc: f64) → f64` | $v_c = \sqrt{GM(r)/r}$ | `astronomy::cosmology` |

### 7️⃣ New — Time Evolution — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Lookback time (general) | `lookback_time_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General lookback integration | `astronomy::cosmology` |
| Age at $z$ | `age_at_z(h0: f64, omega_m: f64, z: f64) → f64` | $t(z)$ | `astronomy::cosmology` |
| Age ΛCDM | `age_lcdm(h0: f64, omega_m: f64) → f64` | Age of flat ΛCDM universe | `astronomy::cosmology` |
| Age general | `age_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) → f64` | General age | `astronomy::cosmology` |
| Matter-radiation equality $z$ | `matter_radiation_equality_z(omega_m: f64, omega_r: f64) → f64` | $z_{\text{eq}} = \Omega_m/\Omega_r - 1$ | `astronomy::cosmology` |

### 8️⃣ New — Horizons — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Particle horizon | `particle_horizon(h0: f64, omega_m: f64) → f64` | Causal horizon | `astronomy::cosmology` |
| Particle horizon (general) | `particle_horizon_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) → f64` | General particle horizon | `astronomy::cosmology` |
| Event horizon | `event_horizon(h0: f64, omega_m: f64) → f64` | Future event horizon | `astronomy::cosmology` |
| Conformal time | `conformal_time(h0: f64, omega_m: f64, z: f64) → f64` | $\eta = \int_0^t dt'/a(t')$ | `astronomy::cosmology` |

### 9️⃣ New — Volume & Density Evolution — 6 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Comoving volume element | `comoving_volume_element(h0: f64, omega_m: f64, z: f64) → f64` | $dV_C/dz$ | `astronomy::cosmology` |
| Comoving volume total | `comoving_volume_total(h0: f64, omega_m: f64, z: f64) → f64` | $V_C(z)$ | `astronomy::cosmology` |
| Comoving volume shell | `comoving_volume_shell(h0: f64, omega_m: f64, z1: f64, z2: f64) → f64` | $\Delta V_C$ between $z_1,z_2$ | `astronomy::cosmology` |
| Radiation density at $z$ | `radiation_density_at_z(h0: f64, omega_r: f64, z: f64) → f64` | $\rho_r(z) = \rho_{c,0}\Omega_r(1+z)^4$ | `astronomy::cosmology` |
| DE density at $z$ | `dark_energy_density_at_z(h0: f64, omega_de: f64, w: f64, z: f64) → f64` | $\rho_{DE}(z)$ | `astronomy::cosmology` |
| Conformal distance | `conformal_distance(h0: f64, omega_m: f64, z: f64) → f64` | $d_\eta$ | `astronomy::cosmology` |

### 🔟 New — Density Parameters & Growth — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| $\Omega_m(z)$ | `omega_m_at_z(omega_m: f64, z: f64) → f64` | $\Omega_m(z) = \Omega_m(1+z)^3/E^2(z)$ | `astronomy::cosmology` |
| $\Omega_{DE}(z)$ | `omega_de_at_z(omega_m: f64, z: f64) → f64` | $\Omega_\Lambda(z) = \Omega_\Lambda/E^2(z)$ | `astronomy::cosmology` |
| Growth factor | `growth_factor_approx(omega_m: f64, z: f64) → f64` | $D(z)$ approximate | `astronomy::cosmology` |
| Growth rate | `growth_rate(omega_m: f64, z: f64) → f64` | $f(z) = d\ln D/d\ln a$ | `astronomy::cosmology` |
| $\sigma_8(z)$ | `sigma8_at_z(sigma8: f64, omega_m: f64, z: f64) → f64` | $\sigma_8 D(z)/D(0)$ | `astronomy::cosmology` |

### 11. New — CMB & Radiation — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| CMB temperature at $z$ | `cmb_temperature_at_z(z: f64) → f64` | $T(z) = T_0(1+z)$ | `astronomy::cosmology` |
| Photon number density | `photon_number_density(z: f64) → f64` | $n_\gamma \propto (1+z)^3$ | `astronomy::cosmology` |
| CMB energy density | `cmb_energy_density(z: f64) → f64` | $u_\gamma = aT^4$ | `astronomy::cosmology` |
| $\Omega_r$ from $T_{\text{CMB}}$ | `omega_r_from_tcmb(t_cmb: f64, h0: f64) → f64` | Photon density parameter | `astronomy::cosmology` |
| Baryon-to-photon ratio | `baryon_to_photon_ratio(omega_b: f64, h0: f64) → f64` | $\eta = n_b/n_\gamma$ | `astronomy::cosmology` |

### 12. New — Type Ia Supernovae — 3 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| SN Ia apparent magnitude | `sn1a_apparent_magnitude(h0: f64, omega_m: f64, z: f64, abs_mag: f64) → f64` | $m = M + \mu(z)$ | `astronomy::cosmology` |
| SN Ia $\chi^2$ single | `sn1a_chi2_single(m_obs: f64, m_model: f64, sigma: f64) → f64` | $(m_{\text{obs}} - m_{\text{model}})^2/\sigma^2$ | `astronomy::cosmology` |
| SN Ia $\chi^2$ total | `sn1a_chi2_total(h0: f64, omega_m: f64, abs_mag: f64, data: &[(f64, f64, f64)]) → f64` | $\chi^2 = \sum$ | `astronomy::cosmology` |

### 13. New — BAO & Virial — 6 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| BAO $D_V$ | `dv_bao(h0: f64, omega_m: f64, z: f64) → f64` | $D_V = [D_M^2 cz/H(z)]^{1/3}$ | `astronomy::cosmology` |
| Hubble distance at $z$ | `hubble_distance_at_z(h0: f64, omega_m: f64, z: f64) → f64` | $D_H(z) = c/H(z)$ | `astronomy::cosmology` |
| $r_d / D_V$ | `rd_over_dv(r_d: f64, h0: f64, omega_m: f64, z: f64) → f64` | BAO observable ratio | `astronomy::cosmology` |
| Virial radius | `virial_radius(h0: f64, omega_m: f64, mass_solar: f64, z: f64) → f64` | $R_{200}$ | `astronomy::cosmology` |
| NFW concentration | `nfw_concentration_duffy08(mass_solar: f64, z: f64) → f64` | Duffy+ 2008 fit | `astronomy::cosmology` |
| Virial velocity | `virial_velocity(h0: f64, omega_m: f64, mass_solar: f64, z: f64) → f64` | $V_{200} = \sqrt{GM/R_{200}}$ | `astronomy::cosmology` |

### 14. New — Cosmological Parameters — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Photon density today | `photon_density_today() → f64` | $\rho_\gamma(z=0)$ | `astronomy::cosmology` |
| Neutrino temperature | `neutrino_temperature() → f64` | $T_\nu = (4/11)^{1/3} T_\gamma$ | `astronomy::cosmology` |
| $N_{\text{eff}}$ standard | `n_eff_standard() → f64` | Standard $N_{\text{eff}} = 3.044$ | `astronomy::cosmology` |
| $\Omega_r$ with neutrinos | `omega_r_with_neutrinos(omega_gamma: f64, n_eff: f64) → f64` | $\Omega_r = \Omega_\gamma(1 + 7N_{\text{eff}}/8 \cdot (4/11)^{4/3})$ | `astronomy::cosmology` |
| Jeans length | `jeans_length(sound_speed: f64, density: f64) → f64` | $\lambda_J = c_s\sqrt{\pi/(G\rho)}$ | `astronomy::cosmology` |
| Jeans mass | `jeans_mass(sound_speed: f64, density: f64) → f64` | $M_J = (4\pi/3)\rho(\lambda_J/2)^3$ | `astronomy::cosmology` |
| $z$ from temperature | `redshift_from_temperature(t: f64) → f64` | $z = T/T_0 - 1$ | `astronomy::cosmology` |
| NFW concentration | `nfw_concentration() → f64` | MW halo $c$ | `astronomy::cosmology` |
| NFW virial mass | `nfw_virial_mass() → f64` | MW $M_{\text{vir}}$ | `astronomy::cosmology` |
| NFW virial radius | `nfw_virial_radius() → f64` | MW $R_{\text{vir}}$ | `astronomy::cosmology` |
| Local DM density (GeV) | `local_dm_density_gev() → f64` | $\rho_{\text{DM}}$ (GeV/cm³) | `astronomy::cosmology` |
| Local DM density (SI) | `local_dm_density_si() → f64` | $\rho_{\text{DM}}$ (kg/m³) | `astronomy::cosmology` |
| WIMP mass range | `wimp_mass_range() → (f64, f64)` | (min, max) GeV | `astronomy::cosmology` |
| WIMP cross-section upper | `wimp_cross_section_upper() → f64` | $\sigma_{\text{max}}$ (cm²) | `astronomy::cosmology` |
| Thermal relic $\langle\sigma v\rangle$ | `thermal_relic_cross_section() → f64` | $3 \times 10^{-26}$ cm³/s | `astronomy::cosmology` |
| Axion mass range | `axion_mass_range() → (f64, f64)` | (min, max) eV | `astronomy::cosmology` |
| Axion decay constant | `axion_decay_constant() → f64` | $f_a$ | `astronomy::cosmology` |

### 15. New — Density & Epoch Constants — 8 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| DM relic density | `dm_relic_density() → f64` | $\Omega_{\text{DM}}$ | `astronomy::cosmology` |
| Baryon density | `baryon_density() → f64` | $\Omega_b$ | `astronomy::cosmology` |
| Matter density | `matter_density() → f64` | $\Omega_m$ | `astronomy::cosmology` |
| Matter-radiation eq temperature | `matter_radiation_equality_temperature() → f64` | $T_{\text{eq}}$ | `astronomy::cosmology` |
| Matter-radiation eq redshift | `matter_radiation_equality_redshift() → f64` | $z_{\text{eq}}$ | `astronomy::cosmology` |
| Recombination temperature | `recombination_temperature() → f64` | $T_{\text{rec}}$ | `astronomy::cosmology` |
| Recombination redshift | `recombination_redshift() → f64` | $z_{\text{rec}}$ | `astronomy::cosmology` |
| Age of universe (s) | `age_of_universe_seconds() → f64` | $t_0$ (s) | `astronomy::cosmology` |

### 16. New — Global Hubble — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hubble constant | `hubble_constant() → f64` | $H_0$ (km/s/Mpc) | `astronomy::cosmology` |
| Cosmological constant | `cosmological_constant() → f64` | $\Lambda$ (m⁻²) | `astronomy::cosmology` |
| Hubble time | `hubble_time() → f64` | $1/H_0$ (s) | `astronomy::cosmology` |
| Hubble radius | `hubble_radius() → f64` | $c/H_0$ (m) | `astronomy::cosmology` |

---

## v0.0.1

### 1️⃣ Cosmology — `astronomy::cosmology` — 15 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hubble velocity | `hubble_velocity(h0: f64, distance: f64) → f64` | $v = H_0 d$ | `astronomy::cosmology` |
| Hubble distance | `hubble_distance(h0: f64, velocity: f64) → f64` | $d = v/H_0$ | `astronomy::cosmology` |
| Redshift velocity | `redshift_velocity(v: f64, c: f64) → f64` | $z = v/c$ (non-relativistic) | `astronomy::cosmology` |
| Relativistic redshift | `relativistic_redshift(v: f64) → f64` | $z = \sqrt{(1+\beta)/(1-\beta)} - 1$ | `astronomy::cosmology` |
| Velocity from redshift | `velocity_from_redshift(z: f64) → f64` | Inverse of relativistic $z$ | `astronomy::cosmology` |
| Cosmological redshift | `cosmological_redshift(a_emit: f64, a_obs: f64) → f64` | $z = a_{\text{obs}}/a_{\text{emit}} - 1$ | `astronomy::cosmology` |
| Scale factor | `scale_factor(redshift: f64) → f64` | $a = 1/(1+z)$ | `astronomy::cosmology` |
| Critical density | `critical_density(h: f64) → f64` | $\rho_c = 3H^2/(8\pi G)$ | `astronomy::cosmology` |
| Critical density from $H_0$ | `critical_density_from_h0(h0: f64) → f64` | $\rho_c(H_0)$ | `astronomy::cosmology` |
| Friedmann expansion | `friedmann_expansion(h0: f64, omega_m: f64, omega_r: f64, omega_lambda: f64, a: f64) → f64` | $H(a)$ from Friedmann | `astronomy::cosmology` |
| Lookback time | `lookback_time(z: f64, h0: f64) → f64` | $t_L(z)$ | `astronomy::cosmology` |
| Luminosity distance | `luminosity_distance(comoving_distance: f64, z: f64) → f64` | $d_L = d_C(1+z)$ | `astronomy::cosmology` |
| Angular diameter distance | `angular_diameter_distance(comoving_distance: f64, z: f64) → f64` | $d_A = d_C/(1+z)$ | `astronomy::cosmology` |
| CMB temperature | `cmb_temperature(t0: f64, z: f64) → f64` | $T(z) = T_0(1+z)$ | `astronomy::cosmology` |
| Age of universe | `age_of_universe(h0: f64) → f64` | $t_0 \approx 1/H_0$ | `astronomy::cosmology` |
