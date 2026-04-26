# Astronomy — ChangeLog

---

## v0.0.3

### 1️⃣ New — Galactic Dynamics — `astronomy::galactic` — 21 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Galactic rotation velocity | `galactic_rotation_velocity(r: f64) → f64` | $v = \sqrt{GM/R}$ — flat rotation curve | `astronomy::galactic` |
| Sun orbital period | `sun_orbital_period() → f64` | Sun's period around galactic center | `astronomy::galactic` |
| Sun orbital velocity | `sun_orbital_velocity() → f64` | ~220 km/s orbital speed | `astronomy::galactic` |
| Sun galactocentric distance | `sun_galactocentric_distance() → f64` | ~8 kpc distance to GC | `astronomy::galactic` |
| Galactic mass within radius | `galactic_mass_within_radius(r: f64) → f64` | $M(R) = v^2 R / G$ | `astronomy::galactic` |
| Sgr A* Schwarzschild radius | `sgr_a_schwarzschild_radius() → f64` | $R_s = 2GM/c^2$ for Sgr A* | `astronomy::galactic` |
| Sgr A* sphere of influence | `sgr_a_sphere_of_influence() → f64` | Gravitational influence radius | `astronomy::galactic` |
| Sgr A* distance | `sgr_a_distance() → f64` | Distance to Sgr A* | `astronomy::galactic` |
| M31 approach time | `m31_approach_time() → f64` | Andromeda–MW merger timescale | `astronomy::galactic` |
| M31 reduced mass | `m31_reduced_mass() → f64` | $\mu = M_1 M_2 / (M_1 + M_2)$ | `astronomy::galactic` |
| Hubble recession velocity | `hubble_recession_velocity(distance_mpc: f64) → f64` | $v = H_0 d$ | `astronomy::galactic` |
| Galactic disk density | `galactic_disk_density(r: f64, z: f64) → f64` | $\rho(R,z)=\rho_0 e^{-R/h_R} \text{sech}^2(z/h_z)$ | `astronomy::galactic` |
| Galactic escape velocity | `galactic_escape_velocity(r: f64) → f64` | $v_{esc}=\sqrt{2GM/R}$ | `astronomy::galactic` |
| Galactic circular velocity | `galactic_circular_velocity(r: f64) → f64` | $v_c=\sqrt{GM/R}$ | `astronomy::galactic` |
| Galactic dynamical time | `galactic_dynamical_time(r: f64) → f64` | $t_{dyn}=R/v$ | `astronomy::galactic` |
| Stellar number density | `stellar_number_density(r: f64, z: f64) → f64` | $n(R)=n_0 e^{-R/h_R}$ | `astronomy::galactic` |
| Oort A constant | `oort_a_constant(r: f64, dr: f64) → f64` | $A=\frac{1}{2}\left(\frac{v_c}{R}-\frac{dv_c}{dR}\right)$ | `astronomy::galactic` |
| Oort B constant | `oort_b_constant(r: f64, dr: f64) → f64` | $B=-\frac{1}{2}\left(\frac{v_c}{R}+\frac{dv_c}{dR}\right)$ | `astronomy::galactic` |
| Epicyclic frequency | `epicyclic_frequency(r: f64, dr: f64) → f64` | $\kappa=\sqrt{-4B(A-B)}$ | `astronomy::galactic` |
| Bulge mass within | `bulge_mass_within(r: f64) → f64` | Hernquist profile: $M(R)=M_b \frac{R^2}{(R+a)^2}$ | `astronomy::galactic` |
| Tidal radius | `tidal_radius(m_cluster: f64, r_galactic: f64) → f64` | $r_t=d\left(\frac{m}{3M}\right)^{1/3}$ | `astronomy::galactic` |

### 2️⃣ New — Planetary Physics — `astronomy::planetary` — 48 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hydrostatic pressure | `hydrostatic_pressure(density: f64, g_surface: f64, depth: f64) → f64` | $P = \rho g d$ | `astronomy::planetary` |
| Central pressure | `central_pressure(density: f64, g_surface: f64, radius: f64) → f64` | $P_c = \rho g R$ | `astronomy::planetary` |
| Adiabatic temperature gradient | `adiabatic_temperature_gradient(g_local: f64, specific_heat: f64) → f64` | $\nabla T = -g/c_p$ | `astronomy::planetary` |
| Planetary MoI factor | `planetary_moment_of_inertia_factor(core_density: f64, mantle_density: f64, core_radius: f64, total_radius: f64) → f64` | Two-layer MoI dimensionless factor | `astronomy::planetary` |
| Love number k₂ | `love_number_k2(rigidity: f64, density: f64, radius: f64) → f64` | Tidal deformability parameter | `astronomy::planetary` |
| Tidal heating | `tidal_heating(radius: f64, eccentricity: f64, mean_motion: f64, k2: f64, tidal_q: f64, perturber_mass: f64, semi_major_axis: f64) → f64` | $\dot{E}=\frac{21}{2}\frac{k_2 n^5 R^5 e^2}{Q}$ | `astronomy::planetary` |
| Tidal locking timescale | `tidal_locking_timescale(mass: f64, radius: f64, semi_major_axis: f64, perturber_mass: f64, tidal_q: f64, rigidity: f64, initial_spin: f64) → f64` | Spin-down time estimate | `astronomy::planetary` |
| Roche limit (fluid) | `roche_limit_fluid(primary_radius: f64, primary_density: f64, secondary_density: f64) → f64` | $d=2.46 R_p (\rho_p/\rho_s)^{1/3}$ | `astronomy::planetary` |
| Roche limit (rigid) | `roche_limit_rigid(primary_radius: f64, primary_density: f64, secondary_density: f64) → f64` | $d=1.26 R_p (\rho_p/\rho_s)^{1/3}$ | `astronomy::planetary` |
| Equilibrium temperature | `equilibrium_temperature(stellar_luminosity: f64, semi_major_axis: f64, albedo: f64) → f64` | $T_{eq}=\left(\frac{L(1-A)}{16\pi\sigma a^2}\right)^{1/4}$ | `astronomy::planetary` |
| Greenhouse surface temp | `greenhouse_surface_temperature(equilibrium_temp: f64, optical_depth_ir: f64) → f64` | $T_s = T_{eq}(1+\tau_{IR})^{1/4}$ | `astronomy::planetary` |
| Scale height | `scale_height(temperature: f64, mean_molecular_mass: f64, g_surface: f64) → f64` | $H = kT/(\mu g)$ | `astronomy::planetary` |
| Jeans escape | `atmospheric_escape_jeans(temperature: f64, molecular_mass: f64, escape_velocity: f64) → f64` | Jeans thermal escape rate | `astronomy::planetary` |
| Magnetopause standoff | `magnetopause_standoff(dipole_moment: f64, solar_wind_pressure: f64) → f64` | Dipole vs solar wind pressure balance | `astronomy::planetary` |
| Ring Roche inner | `ring_roche_inner(planet_mass: f64, planet_radius: f64, particle_density: f64) → f64` | Inner edge of ring stability | `astronomy::planetary` |
| Ring optical depth | `ring_optical_depth(surface_density: f64, particle_radius: f64, particle_density: f64) → f64` | $\tau = 3\Sigma/(4\rho r)$ | `astronomy::planetary` |
| Synchronous orbit radius | `synchronous_orbit_radius(mass: f64, rotation_period: f64) → f64` | $(GMP^2/4\pi^2)^{1/3}$ | `astronomy::planetary` |
| Oblateness | `oblateness(rotation_rate: f64, equatorial_radius: f64, mass: f64) → f64` | Rotational flattening | `astronomy::planetary` |
| Precession rate | `precession_rate(oblateness_j2: f64, planet_radius: f64, semi_major_axis: f64, mean_motion: f64) → f64` | $\dot{\Omega}=\frac{3}{2}J_2(R_p/a)^2 n$ | `astronomy::planetary` |
| Bond albedo from geometric | `bond_albedo_from_geometric(geometric_albedo: f64, phase_integral: f64) → f64` | $A_B = A_g \cdot q$ | `astronomy::planetary` |
| Thermal inertia | `thermal_inertia(thermal_conductivity: f64, density: f64, specific_heat: f64) → f64` | $\Gamma=\sqrt{k\rho c_p}$ | `astronomy::planetary` |
| Diurnal skin depth | `diurnal_skin_depth(thermal_diffusivity: f64, rotation_period: f64) → f64` | $l_s=\sqrt{\kappa P/\pi}$ | `astronomy::planetary` |
| Subsolar temperature | `subsolar_temperature(stellar_flux: f64, albedo: f64, emissivity: f64) → f64` | $T_{ss}=\left(\frac{F(1-A)}{\varepsilon\sigma}\right)^{1/4}$ | `astronomy::planetary` |
| Nightside temperature | `nightside_temperature(thermal_inertia_val: f64, subsolar_temp: f64, rotation_period: f64) → f64` | Thermal inertia–controlled | `astronomy::planetary` |
| Sputtering loss rate | `sputtering_loss_rate(stellar_wind_flux: f64, sputtering_yield: f64, cross_section: f64) → f64` | Solar wind sputtering | `astronomy::planetary` |
| Hill sphere (atmospheric) | `hill_sphere_atmospheric(planet_mass: f64, stellar_mass: f64, semi_major_axis: f64) → f64` | $r_H=a(M_p/3M_s)^{1/3}$ | `astronomy::planetary` |
| Planet data lookup | `planet_data(name: &str) → Option<PlanetData>` | Database lookup by name | `astronomy::planetary` |
| Planet mass | `planet_mass(name: &str) → Option<f64>` | Mass (kg) | `astronomy::planetary` |
| Planet radius | `planet_radius(name: &str) → Option<f64>` | Mean radius (m) | `astronomy::planetary` |
| Planet flattening | `planet_flattening(name: &str) → Option<f64>` | Oblateness | `astronomy::planetary` |
| Planet orbital period | `planet_orbital_period(name: &str) → Option<f64>` | Sidereal period (s) | `astronomy::planetary` |
| Planet semi-major axis | `planet_semi_major_axis(name: &str) → Option<f64>` | Orbital distance (m) | `astronomy::planetary` |
| Planet eccentricity | `planet_eccentricity(name: &str) → Option<f64>` | Orbital eccentricity | `astronomy::planetary` |
| Planet inclination | `planet_inclination(name: &str) → Option<f64>` | Orbital inclination (rad) | `astronomy::planetary` |
| Planet axial tilt | `planet_axial_tilt(name: &str) → Option<f64>` | Obliquity (rad) | `astronomy::planetary` |
| Planet sidereal day | `planet_sidereal_day(name: &str) → Option<f64>` | Rotation period (s) | `astronomy::planetary` |
| Planet surface gravity | `planet_surface_gravity(name: &str) → Option<f64>` | Surface g (m/s²) | `astronomy::planetary` |
| Planet escape velocity | `planet_escape_velocity(name: &str) → Option<f64>` | $v_{esc}$ (m/s) | `astronomy::planetary` |
| Planet mean density | `planet_mean_density(name: &str) → Option<f64>` | Bulk density (kg/m³) | `astronomy::planetary` |
| Planet Bond albedo | `planet_bond_albedo(name: &str) → Option<f64>` | Bond albedo | `astronomy::planetary` |
| Planet orbital velocity | `planet_orbital_velocity(name: &str) → Option<f64>` | Mean orbital speed (m/s) | `astronomy::planetary` |
| Planet volume | `planet_volume(name: &str) → Option<f64>` | Volume (m³) | `astronomy::planetary` |
| Planet circumference | `planet_circumference(name: &str) → Option<f64>` | Equatorial circumference (m) | `astronomy::planetary` |
| Planet surface area | `planet_surface_area(name: &str) → Option<f64>` | Total surface area (m²) | `astronomy::planetary` |
| Planet gravitational param | `planet_gravitational_parameter(name: &str) → Option<f64>` | $\mu = GM$ (m³/s²) | `astronomy::planetary` |
| Planet Hill sphere | `planet_hill_sphere(name: &str, stellar_mass: f64) → Option<f64>` | Hill radius (m) | `astronomy::planetary` |
| Planet Roche limit | `planet_roche_limit(name: &str, secondary_density: f64) → Option<f64>` | Roche limit (m) | `astronomy::planetary` |
| Planet synchronous orbit | `planet_synchronous_orbit(name: &str) → Option<f64>` | Geostationary radius (m) | `astronomy::planetary` |

### 3️⃣ New — Impact Physics — `astronomy::impacts` — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Crater diameter | `crater_diameter(rho_i: f64, d_p: f64, v: f64, g: f64, rho_t: f64) → f64` | Holsapple pi-scaling: $D \propto \rho_i^{1/3} d_p^{0.78} v^{0.44} g^{-0.22}$ | `astronomy::impacts` |
| Fireball radius | `fireball_radius(e_kt: f64) → f64` | $R = 55 \cdot E_{kt}^{0.4}$ (m) | `astronomy::impacts` |
| Ejecta volume | `ejecta_volume(d: f64, depth: f64) → f64` | Truncated paraboloid ejecta | `astronomy::impacts` |
| Impact velocity | `impact_velocity(v_inf: f64, v_esc: f64) → f64` | $v=\sqrt{v_\infty^2 + v_{esc}^2}$ | `astronomy::impacts` |
| Ejecta escape fraction | `ejecta_escape_fraction(v_esc: f64, v_ejecta: f64) → f64` | Velocity-dependent fraction | `astronomy::impacts` |

### 4️⃣ New — Rotation — `astronomy::rotation` — 7 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Surface velocity at latitude | `surface_velocity_at_latitude(omega: f64, r: f64, phi: f64) → f64` | $v = \omega R \cos\varphi$ | `astronomy::rotation` |
| Centripetal acceleration | `centripetal_acceleration(omega: f64, r: f64, phi: f64) → f64` | $a = \omega^2 R \cos\varphi$ | `astronomy::rotation` |
| Coriolis parameter | `coriolis_parameter(omega: f64, phi: f64) → f64` | $f = 2\omega\sin\varphi$ | `astronomy::rotation` |
| Moment of inertia | `moment_of_inertia(c_factor: f64, m: f64, r: f64) → f64` | $I = C \cdot MR^2$ | `astronomy::rotation` |
| Rotational kinetic energy | `rotational_kinetic_energy(inertia: f64, omega: f64) → f64` | $E_k=\frac{1}{2}I\omega^2$ | `astronomy::rotation` |
| Nutation obliquity | `nutation_obliquity_rad(omega_node: f64) → f64` | $\Delta\varepsilon = 9.2''\cos\Omega$ — IAU 1980 | `astronomy::rotation` |
| Day length variation | `day_length_variation(doy: f64, latitude: f64, tilt: f64) → f64` | Sunrise equation: $\cos h_0 = -\tan\varphi\tan\delta$ | `astronomy::rotation` |

### 5️⃣ New — Tidal Physics — `astronomy::tides` — 6 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Tidal potential | `tidal_potential(m: f64, r: f64, d: f64, k2: f64, theta: f64) → f64` | $U=-\frac{GM r^2}{d^3}(1+k_2)\left(\frac{3}{2}\cos^2\theta-\frac{1}{2}\right)$ | `astronomy::tides` |
| Tidal bulge height | `tidal_bulge_height(a_tidal: f64, r: f64, g: f64, h2: f64) → f64` | $h_2 \cdot a_{\text{tidal}} R / g$ — equilibrium tide | `astronomy::tides` |
| Spring tide amplitude | `spring_tide_amplitude(h_moon: f64, h_sun: f64) → f64` | Moon + Sun combined bulge | `astronomy::tides` |
| Neap tide amplitude | `neap_tide_amplitude(h_moon: f64, h_sun: f64) → f64` | Moon − Sun differential | `astronomy::tides` |
| Tidal dissipation rate | `tidal_dissipation_rate(k2: f64, n: f64, m: f64, r: f64, q: f64, d: f64) → f64` | $\dot{E}=\frac{21}{2}\frac{k_2 n G M^2 R^5}{Q d^6}$ | `astronomy::tides` |
| Tidal locking timescale | `tidal_locking_timescale(omega: f64, a: f64, mu: f64, q: f64, m: f64, r: f64) → f64` | $t_{\text{lock}}=\frac{\omega a^6 \mu Q}{6 G M^2 R^5}$ | `astronomy::tides` |

### 6️⃣ Changes — Cosmology — `astronomy::cosmology` — 77 pub fn (+25 new)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hubble distance (Mpc) | `hubble_distance_mpc(h0: f64) → f64` | $d_H = c/H_0$ in Mpc | `astronomy::cosmology` |
| Hubble time (Gyr) | `hubble_time_gyr(h0: f64) → f64` | $t_H = 1/H_0$ in Gyr | `astronomy::cosmology` |
| Omega_k | `omega_k_from(omega_m: f64, omega_r: f64, omega_de: f64) → f64` | $\Omega_k = 1 - \Omega_m - \Omega_r - \Omega_{DE}$ | `astronomy::cosmology` |
| Little h | `little_h(h0: f64) → f64` | $h = H_0 / 100$ | `astronomy::cosmology` |
| Deceleration parameter | `deceleration_parameter(omega_m: f64, z: f64) → f64` | $q = \Omega_m/2 - \Omega_\Lambda$ | `astronomy::cosmology` |
| Deceleration param. today | `deceleration_parameter_today(omega_m: f64) → f64` | Current epoch $q_0$ | `astronomy::cosmology` |
| Comoving distance (z) | `comoving_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_C = \int_0^z \frac{c\,dz'}{H(z')}$ | `astronomy::cosmology` |
| Luminosity distance (z) | `luminosity_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_L = (1+z)\,d_C$ | `astronomy::cosmology` |
| Angular diameter dist (z) | `angular_diameter_distance_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_A = d_C/(1+z)$ | `astronomy::cosmology` |
| Distance modulus (z) | `distance_modulus_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $\mu = 5\log_{10}(d_L/10\text{pc})$ | `astronomy::cosmology` |
| Comoving dist. (general) | `comoving_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General ΛCDM integration | `astronomy::cosmology` |
| Transverse comoving dist. | `transverse_comoving_distance(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | Curvature-corrected $d_M$ | `astronomy::cosmology` |
| Lum. distance (general) | `luminosity_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $d_L$ | `astronomy::cosmology` |
| Angular diam. dist. (gen.) | `angular_diameter_distance_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $d_A$ | `astronomy::cosmology` |
| Distance modulus (general) | `distance_modulus_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General $\mu$ | `astronomy::cosmology` |
| Comoving dist. wCDM | `comoving_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | $w$CDM dark energy model | `astronomy::cosmology` |
| Lum. distance wCDM | `luminosity_distance_wcdm(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | $w$CDM luminosity distance | `astronomy::cosmology` |
| Comoving dist. w0wa | `comoving_distance_w0wa(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | CPL $w(a)=w_0+w_a(1-a)$ | `astronomy::cosmology` |
| Lum. distance w0wa | `luminosity_distance_w0wa(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | CPL $d_L$ | `astronomy::cosmology` |
| Lum. from angular diam. | `luminosity_from_angular_diameter(d_a: f64, z: f64) → f64` | $d_L = d_A(1+z)^2$ | `astronomy::cosmology` |
| Angular diam. from lum. | `angular_diameter_from_luminosity(d_l: f64, z: f64) → f64` | $d_A = d_L/(1+z)^2$ | `astronomy::cosmology` |
| Proper distance | `proper_distance(comoving_d: f64, z: f64) → f64` | $d_p = d_C \cdot a$ | `astronomy::cosmology` |
| Lookback time (z) | `lookback_time_from_z(h0: f64, omega_m: f64, z: f64) → f64` | $t_L = \int_0^z \frac{dz'}{(1+z')H(z')}$ | `astronomy::cosmology` |
| E(z) ΛCDM + radiation | `e_z_lcdm_rad(omega_m: f64, omega_r: f64, z: f64) → f64` | $E(z)=\sqrt{\Omega_r(1+z)^4+\Omega_m(1+z)^3+\Omega_\Lambda}$ | `astronomy::cosmology` |
| H(z) ΛCDM | `hubble_at_z_lcdm(h0: f64, omega_m: f64, z: f64) → f64` | $H(z)=H_0 E(z)$ | `astronomy::cosmology` |

### 7️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 42 → 60 (+18) |
| Test files | 3 → 10 (atomized per submodule) |
| File list | `main.rs`, `celestial.rs`, `cosmology.rs`, `galactic.rs`, `impacts.rs`, `orbits.rs`, `planetary.rs`, `rotation.rs`, `stellar.rs`, `tides.rs` |
| Total suite | 814 tests, 0 warnings |

---

## v0.0.2

### 1️⃣ Cosmology Expansion — `astronomy::cosmology` — 15 → 77 pub fn (+62)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| E(z) | `e_z(omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | $E(z)=\sqrt{\sum_i \Omega_i(1+z)^{n_i}}$ | `astronomy::cosmology` |
| E(z) ΛCDM | `e_z_lcdm(omega_m: f64, z: f64) → f64` | ΛCDM Friedmann function | `astronomy::cosmology` |
| E(z) wCDM | `e_z_wcdm(omega_m: f64, omega_de: f64, w: f64, z: f64) → f64` | $w$CDM equation of state | `astronomy::cosmology` |
| E(z) w₀wₐ | `e_z_w0wa(omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) → f64` | CPL parametrization | `astronomy::cosmology` |
| H(z) | `hubble_at_z(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | $H(z)=H_0 E(z)$ | `astronomy::cosmology` |
| Lookback time (general) | `lookback_time_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) → f64` | General lookback integration | `astronomy::cosmology` |
| Age at z | `age_at_z(h0: f64, omega_m: f64, z: f64) → f64` | Universe age at redshift z | `astronomy::cosmology` |
| Age ΛCDM | `age_lcdm(h0: f64, omega_m: f64) → f64` | Universe age (ΛCDM) | `astronomy::cosmology` |
| Age general | `age_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) → f64` | Universe age (general) | `astronomy::cosmology` |
| Acceleration redshift | `acceleration_redshift(omega_m: f64) → f64` | $z_{acc}$ transition | `astronomy::cosmology` |
| Matter-radiation equality | `matter_radiation_equality_z(omega_m: f64, omega_r: f64) → f64` | $z_{eq} = \Omega_m/\Omega_r - 1$ | `astronomy::cosmology` |
| Particle horizon | `particle_horizon(h0: f64, omega_m: f64) → f64` | Causal horizon distance | `astronomy::cosmology` |
| Particle horizon (general) | `particle_horizon_general(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) → f64` | General particle horizon | `astronomy::cosmology` |
| Sound horizon (EH98) | `sound_horizon_eh98(omega_m: f64, omega_b: f64, h0: f64) → f64` | BAO sound horizon $r_s$ (Eisenstein & Hu 1998) | `astronomy::cosmology` |
| Event horizon | `event_horizon(h0: f64, omega_m: f64) → f64` | Future event horizon | `astronomy::cosmology` |
| Comoving volume element | `comoving_volume_element(h0: f64, omega_m: f64, z: f64) → f64` | $dV_C/d\Omega\,dz$ | `astronomy::cosmology` |
| Comoving volume total | `comoving_volume_total(h0: f64, omega_m: f64, z: f64) → f64` | $V_C=\frac{4}{3}\pi d_C^3$ | `astronomy::cosmology` |
| Comoving volume shell | `comoving_volume_shell(h0: f64, omega_m: f64, z1: f64, z2: f64) → f64` | $\Delta V_C$ between two z | `astronomy::cosmology` |
| Matter density at z | `matter_density_at_z(h0: f64, omega_m: f64, z: f64) → f64` | $\rho_m(z) = \rho_{c,0}\Omega_m(1+z)^3$ | `astronomy::cosmology` |
| Radiation density at z | `radiation_density_at_z(h0: f64, omega_r: f64, z: f64) → f64` | $\rho_r(z) = \rho_{c,0}\Omega_r(1+z)^4$ | `astronomy::cosmology` |
| Dark energy density at z | `dark_energy_density_at_z(h0: f64, omega_de: f64, w: f64, z: f64) → f64` | $\rho_{DE}(z)=\rho_{c,0}\Omega_{DE}(1+z)^{3(1+w)}$ | `astronomy::cosmology` |
| Ω_m at z | `omega_m_at_z(omega_m: f64, z: f64) → f64` | $\Omega_m(z)$ | `astronomy::cosmology` |
| Ω_DE at z | `omega_de_at_z(omega_m: f64, z: f64) → f64` | $\Omega_{DE}(z)$ | `astronomy::cosmology` |
| Growth factor (approx) | `growth_factor_approx(omega_m: f64, z: f64) → f64` | Carroll et al. approximation | `astronomy::cosmology` |
| Growth rate | `growth_rate(omega_m: f64, z: f64) → f64` | $f = d\ln D/d\ln a$ | `astronomy::cosmology` |
| σ₈ at z | `sigma8_at_z(sigma8: f64, omega_m: f64, z: f64) → f64` | $\sigma_8(z) = \sigma_8 D(z)/D(0)$ | `astronomy::cosmology` |
| CMB temperature at z | `cmb_temperature_at_z(z: f64) → f64` | $T(z)=T_0(1+z)$ | `astronomy::cosmology` |
| Photon number density | `photon_number_density(z: f64) → f64` | $n_\gamma(z)$ | `astronomy::cosmology` |
| CMB energy density | `cmb_energy_density(z: f64) → f64` | $u_\gamma(z) = aT^4(z)$ | `astronomy::cosmology` |
| Ω_r from T_CMB | `omega_r_from_tcmb(t_cmb: f64, h0: f64) → f64` | Radiation density from CMB temperature | `astronomy::cosmology` |
| Einstein radius | `einstein_radius(h0: f64, omega_m: f64, mass: f64, z_l: f64, z_s: f64) → f64` | $\theta_E=\sqrt{\frac{4GM}{c^2}\frac{d_{LS}}{d_L d_S}}$ | `astronomy::cosmology` |
| Critical surface density | `critical_surface_density(h0: f64, omega_m: f64, z_l: f64, z_s: f64) → f64` | $\Sigma_{cr}=\frac{c^2}{4\pi G}\frac{d_S}{d_L d_{LS}}$ | `astronomy::cosmology` |
| SN Ia apparent magnitude | `sn1a_apparent_magnitude(h0: f64, omega_m: f64, z: f64, abs_mag: f64) → f64` | $m = M + \mu(z)$ | `astronomy::cosmology` |
| SN Ia χ² (single) | `sn1a_chi2_single(m_obs: f64, m_model: f64, sigma: f64) → f64` | $(m_{obs}-m_{model})^2/\sigma^2$ | `astronomy::cosmology` |
| SN Ia χ² (total) | `sn1a_chi2_total(h0: f64, omega_m: f64, abs_mag: f64, data: &[(f64, f64, f64)]) → f64` | Sum over SN Ia data | `astronomy::cosmology` |
| D_V BAO | `dv_bao(h0: f64, omega_m: f64, z: f64) → f64` | BAO volume-averaged distance | `astronomy::cosmology` |
| Hubble distance at z | `hubble_distance_at_z(h0: f64, omega_m: f64, z: f64) → f64` | $d_H(z) = c/H(z)$ | `astronomy::cosmology` |

### 2️⃣ Other Module Changes

| Module | Changes |
|---|---|
| `astronomy::celestial` | Minor modifications |
| `astronomy::orbits` | Minor modifications |
| `astronomy::stellar` | Minor modifications |

### 3️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 0 → 42 |
| Test files | `cosmology.rs` (20), `stellar.rs` (11), `orbits.rs` (10), `main.rs` (1) |

---

## v0.0.1

### 1️⃣ Celestial Mechanics — `astronomy::celestial` — 43 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Gravitational force | `gravitational_force(m1: f64, m2: f64, r: f64) → f64` | $F = Gm_1 m_2/r^2$ | `astronomy::celestial` |
| Tidal force | `tidal_force(m: f64, r: f64, delta_r: f64) → f64` | Differential gravitational acceleration | `astronomy::celestial` |
| Synodic period | `synodic_period(p1: f64, p2: f64) → f64` | $1/P_{syn}=\|1/P_1-1/P_2\|$ | `astronomy::celestial` |
| Apparent angular size | `apparent_angular_size(diameter: f64, distance: f64) → f64` | $\theta = d/D$ (rad) | `astronomy::celestial` |
| Parallax distance | `parallax_distance(parallax_arcsec: f64) → f64` | $d = 1/p''$ (pc) | `astronomy::celestial` |
| Barycenter | `barycenter(m1: f64, m2: f64, d: f64) → f64` | $r_1 = d \cdot m_2/(m_1+m_2)$ | `astronomy::celestial` |
| Lagrange L₁ | `lagrange_l1(d: f64, m1: f64, m2: f64) → f64` | First Lagrange point distance | `astronomy::celestial` |
| Hill sphere | `hill_sphere(a: f64, m: f64, m_star: f64, e: f64) → f64` | $r_H = a(1-e)(m/3M)^{1/3}$ | `astronomy::celestial` |
| Surface gravity | `surface_gravity(m: f64, r: f64) → f64` | $g = GM/R^2$ | `astronomy::celestial` |
| Sidereal to solar | `sidereal_to_solar(sidereal_period: f64, orbital_period: f64) → f64` | Sidereal-to-solar day conversion | `astronomy::celestial` |
| Habitable zone inner | `habitable_zone_inner(luminosity_solar: f64) → f64` | $r_{in}=\sqrt{L/S_{in}}$ | `astronomy::celestial` |
| Habitable zone outer | `habitable_zone_outer(luminosity_solar: f64) → f64` | $r_{out}=\sqrt{L/S_{out}}$ | `astronomy::celestial` |
| JD to J2000 century | `julian_date_to_j2000_century(jd: f64) → f64` | $T=(JD-2451545)/36525$ | `astronomy::celestial` |
| J2000 century to JD | `j2000_century_to_julian_date(t: f64) → f64` | Inverse conversion | `astronomy::celestial` |
| Mean obliquity ecliptic | `mean_obliquity_ecliptic(t_century: f64) → f64` | IAU polynomial series | `astronomy::celestial` |
| Nutation longitude | `nutation_longitude(omega: f64) → f64` | $\Delta\psi=-17.2''\sin\Omega$ | `astronomy::celestial` |
| Precession rate | `precession_rate_arcsec_per_year(t_century: f64) → f64` | Lunisolar precession rate | `astronomy::celestial` |
| Equation of time | `equation_of_time(day_of_year: f64) → f64` | Equation of time (minutes) | `astronomy::celestial` |
| Sidereal year | `sidereal_year_seconds() → f64` | Duration in seconds | `astronomy::celestial` |
| Tropical year | `tropical_year_seconds() → f64` | Duration in seconds | `astronomy::celestial` |
| Precession period | `precession_period() → f64` | ~26 000 yr cycle | `astronomy::celestial` |
| Day length | `day_length_seconds() → f64` | 86 400 s | `astronomy::celestial` |
| Solar day to sidereal | `solar_day_to_sidereal_day(solar_day_s: f64, orbital_period_s: f64) → f64` | Conversion | `astronomy::celestial` |
| Degrees to radians | `degrees_to_radians(deg: f64) → f64` | $\text{rad}=\deg\cdot\pi/180$ | `astronomy::celestial` |
| Radians to degrees | `radians_to_degrees(rad: f64) → f64` | $\deg=\text{rad}\cdot180/\pi$ | `astronomy::celestial` |
| Tidal dissipation factor | `tidal_dissipation_factor() → f64` | Nominal Q factor | `astronomy::celestial` |
| Tidal quality factor | `tidal_quality_factor(specific_dissipation: f64) → f64` | Specific dissipation to Q | `astronomy::celestial` |
| Tidal heating | `tidal_heating(mass_primary: f64, radius: f64, eccentricity: f64, a: f64, n: f64) → f64` | Orbital energy dissipation | `astronomy::celestial` |
| AU to meters | `au_to_meters(au: f64) → f64` | Unit conversion | `astronomy::celestial` |
| Meters to AU | `meters_to_au(m: f64) → f64` | Unit conversion | `astronomy::celestial` |
| Light-years to meters | `light_years_to_meters(ly: f64) → f64` | Unit conversion | `astronomy::celestial` |
| Meters to light-years | `meters_to_light_years(m: f64) → f64` | Unit conversion | `astronomy::celestial` |
| AU to light-years | `au_to_light_years(au: f64) → f64` | Unit conversion | `astronomy::celestial` |
| Light-years to AU | `light_years_to_au(ly: f64) → f64` | Unit conversion | `astronomy::celestial` |
| Earth-Moon distance | `earth_moon_distance() → f64` | Mean distance (m) | `astronomy::celestial` |
| Lunar orbital period | `lunar_orbital_period() → f64` | Sidereal period (s) | `astronomy::celestial` |
| Moon data lookup | `moon_data(name: &str) → Option<MoonData>` | Database lookup | `astronomy::celestial` |
| Moon mass | `moon_mass(name: &str) → Option<f64>` | Mass (kg) | `astronomy::celestial` |
| Moon radius | `moon_radius(name: &str) → Option<f64>` | Mean radius (m) | `astronomy::celestial` |
| Moon surface gravity | `moon_surface_gravity(name: &str) → Option<f64>` | Surface g (m/s²) | `astronomy::celestial` |
| Moon escape velocity | `moon_escape_velocity(name: &str) → Option<f64>` | $v_{esc}$ (m/s) | `astronomy::celestial` |
| Moon volume | `moon_volume(name: &str) → Option<f64>` | Volume (m³) | `astronomy::celestial` |
| Moon mean density | `moon_mean_density(name: &str) → Option<f64>` | Bulk density (kg/m³) | `astronomy::celestial` |

### 2️⃣ Cosmology — `astronomy::cosmology` — 15 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hubble velocity | `hubble_velocity(h0: f64, distance: f64) → f64` | $v = H_0 d$ | `astronomy::cosmology` |
| Hubble distance | `hubble_distance(h0: f64, velocity: f64) → f64` | $d = v/H_0$ | `astronomy::cosmology` |
| Redshift velocity | `redshift_velocity(v: f64, c: f64) → f64` | $z = v/c$ (non-relativistic) | `astronomy::cosmology` |
| Relativistic redshift | `relativistic_redshift(v: f64) → f64` | $z=\sqrt{\frac{1+\beta}{1-\beta}}-1$ | `astronomy::cosmology` |
| Velocity from redshift | `velocity_from_redshift(z: f64) → f64` | Inverse relativistic | `astronomy::cosmology` |
| Cosmological redshift | `cosmological_redshift(a_emit: f64, a_obs: f64) → f64` | $z = a_{obs}/a_{emit} - 1$ | `astronomy::cosmology` |
| Scale factor | `scale_factor(redshift: f64) → f64` | $a = 1/(1+z)$ | `astronomy::cosmology` |
| Critical density | `critical_density(h: f64) → f64` | $\rho_c = 3H^2/(8\pi G)$ | `astronomy::cosmology` |
| Critical density (H₀) | `critical_density_from_h0(h0: f64) → f64` | Using $H_0$ directly | `astronomy::cosmology` |
| Friedmann expansion | `friedmann_expansion(h0: f64, omega_m: f64, omega_r: f64, omega_lambda: f64, a: f64) → f64` | Friedmann equation RHS | `astronomy::cosmology` |
| Lookback time | `lookback_time(z: f64, h0: f64) → f64` | $t_L = \int_0^z \frac{dz'}{(1+z')H(z')}$ | `astronomy::cosmology` |
| Luminosity distance | `luminosity_distance(comoving_distance: f64, z: f64) → f64` | $d_L = d_C(1+z)$ | `astronomy::cosmology` |
| Angular diameter distance | `angular_diameter_distance(comoving_distance: f64, z: f64) → f64` | $d_A = d_C/(1+z)$ | `astronomy::cosmology` |
| CMB temperature | `cmb_temperature(t0: f64, z: f64) → f64` | $T(z)=T_0(1+z)$ | `astronomy::cosmology` |
| Age of universe | `age_of_universe(h0: f64) → f64` | $t_0 \approx 1/H_0$ | `astronomy::cosmology` |

### 3️⃣ Orbital Mechanics — `astronomy::orbits` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Kepler period | `kepler_period(a: f64, mu: f64) → f64` | $T = 2\pi\sqrt{a^3/\mu}$ | `astronomy::orbits` |
| Kepler velocity | `kepler_velocity(mu: f64, r: f64, a: f64) → f64` | Mean orbital velocity | `astronomy::orbits` |
| Circular velocity | `circular_velocity(mu: f64, r: f64) → f64` | $v_c = \sqrt{\mu/r}$ | `astronomy::orbits` |
| Escape velocity | `escape_velocity(mu: f64, r: f64) → f64` | $v_{esc}=\sqrt{2\mu/r}$ | `astronomy::orbits` |
| Vis-viva | `vis_viva(mu: f64, r: f64, a: f64) → f64` | $v=\sqrt{\mu(2/r-1/a)}$ | `astronomy::orbits` |
| Orbital energy | `orbital_energy(mu: f64, a: f64) → f64` | $E=-\mu/(2a)$ | `astronomy::orbits` |
| Angular momentum | `angular_momentum(mu: f64, a: f64, e: f64) → f64` | $L=\sqrt{\mu a(1-e^2)}$ | `astronomy::orbits` |
| Periapsis | `periapsis(a: f64, e: f64) → f64` | $r_p = a(1-e)$ | `astronomy::orbits` |
| Apoapsis | `apoapsis(a: f64, e: f64) → f64` | $r_a = a(1+e)$ | `astronomy::orbits` |
| True anomaly to radius | `true_anomaly_to_radius(a: f64, e: f64, theta: f64) → f64` | $r=a(1-e^2)/(1+e\cos\nu)$ | `astronomy::orbits` |
| Hohmann Δv | `hohmann_delta_v(mu: f64, r1: f64, r2: f64) → f64` | Total impulse for Hohmann transfer | `astronomy::orbits` |
| Sphere of influence | `sphere_of_influence(a: f64, m_planet: f64, m_star: f64) → f64` | $r_{SOI}=a(m/M)^{2/5}$ | `astronomy::orbits` |
| Roche limit | `roche_limit(r_primary: f64, rho_primary: f64, rho_secondary: f64) → f64` | $d=2.46 R_p(\rho_p/\rho_s)^{1/3}$ | `astronomy::orbits` |
| Kepler equation solver | `solve_kepler(m: f64, e: f64, tol: f64) → f64` | Newton-Raphson iterative | `astronomy::orbits` |
| GR perihelion precession | `gr_perihelion_precession(mass: f64, a: f64, e: f64) → f64` | $\Delta\phi=\frac{6\pi GM}{a(1-e^2)c^2}$ | `astronomy::orbits` |
| J₂ value | `j2_value(body: &str) → Option<f64>` | Oblateness $J_2$ by body name | `astronomy::orbits` |
| J₂ nodal regression | `j2_nodal_regression(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) → f64` | $\dot{\Omega}$ from $J_2$ | `astronomy::orbits` |
| J₂ apsidal precession | `j2_apsidal_precession(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) → f64` | $\dot{\omega}$ from $J_2$ | `astronomy::orbits` |
| Yoshida4 step | `yoshida4_step(q: &mut [f64], p: &mut [f64], dt: f64, force: &dyn Fn(&[f64]) → Vec<f64>)` | 4th-order symplectic integrator | `astronomy::orbits` |
| Yoshida4 weights | `yoshida4_weights() → (f64, f64)` | Symplectic coefficients | `astronomy::orbits` |

### 4️⃣ Stellar Physics — `astronomy::stellar` — 49 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Luminosity from R, T | `luminosity_from_radius_temp(r: f64, t: f64) → f64` | $L = 4\pi R^2 \sigma T^4$ | `astronomy::stellar` |
| Absolute magnitude | `absolute_magnitude(apparent_mag: f64, distance_pc: f64) → f64` | $M = m - 5\log_{10}(d/10)$ | `astronomy::stellar` |
| Distance modulus | `distance_modulus(apparent_mag: f64, absolute_mag: f64) → f64` | $\mu = m - M$ | `astronomy::stellar` |
| Stellar flux | `stellar_flux(luminosity: f64, distance: f64) → f64` | $F = L/(4\pi d^2)$ | `astronomy::stellar` |
| Wien peak wavelength | `wien_peak_wavelength(temperature: f64) → f64` | $\lambda_{max}=b/T$ | `astronomy::stellar` |
| Main-sequence lifetime | `main_sequence_lifetime(mass_solar: f64, luminosity_solar: f64) → f64` | $t_{MS}\propto M/L$ | `astronomy::stellar` |
| Mass-luminosity | `mass_luminosity_relation(mass_solar: f64) → f64` | Piecewise MS power law | `astronomy::stellar` |
| Schwarzschild radius | `schwarzschild_radius(mass: f64) → f64` | $R_s = 2GM/c^2$ | `astronomy::stellar` |
| Chandrasekhar limit | `chandrasekhar_limit() → f64` | ~1.4 M☉ | `astronomy::stellar` |
| Chandrasekhar (kg) | `chandrasekhar_limit_kg() → f64` | In SI (kg) | `astronomy::stellar` |
| Above Chandrasekhar? | `is_above_chandrasekhar(mass_kg: f64) → bool` | Mass threshold check | `astronomy::stellar` |
| Eddington luminosity | `eddington_luminosity(mass: f64) → f64` | $L_{Edd}=4\pi G M c/\kappa$ | `astronomy::stellar` |
| Spectral type temperature | `spectral_type_temperature(spectral_index: f64) → f64` | Spectral classification T | `astronomy::stellar` |
| Bolometric correction | `bolometric_correction(t_eff: f64) → f64` | BC(T) polynomial | `astronomy::stellar` |
| TOV limit | `tov_limit() → f64` | Neutron star mass limit | `astronomy::stellar` |
| NS surface gravity | `neutron_star_surface_gravity() → f64` | Typical NS surface g | `astronomy::stellar` |
| NS mean density | `neutron_star_mean_density() → f64` | Typical NS density | `astronomy::stellar` |
| NS escape velocity | `neutron_star_escape_velocity() → f64` | Typical NS $v_{esc}$ | `astronomy::stellar` |
| Pulsar spindown luminosity | `pulsar_spindown_luminosity(period: f64, period_dot: f64) → f64` | $\dot{E}=4\pi^2 I \dot{P}/P^3$ | `astronomy::stellar` |
| Pulsar magnetic field | `pulsar_magnetic_field(period: f64, period_dot: f64) → f64` | $B=3.2\times10^{19}\sqrt{P\dot{P}}$ G | `astronomy::stellar` |
| Pulsar characteristic age | `pulsar_characteristic_age(period: f64, period_dot: f64) → f64` | $\tau = P/(2\dot{P})$ | `astronomy::stellar` |
| Pulsar death line | `pulsar_death_line_period(b_field: f64) → f64` | Critical period for emission | `astronomy::stellar` |
| Magnetar energy | `magnetar_energy_reservoir(b_field: f64) → f64` | $E_B = B^2 R^3/(6\mu_0)$ | `astronomy::stellar` |
| Magnetar typical energy | `magnetar_typical_energy() → f64` | Nominal value | `astronomy::stellar` |
| Radiation pressure | `radiation_pressure(temperature: f64) → f64` | $P_{rad}=aT^4/3$ | `astronomy::stellar` |
| Gas pressure | `gas_pressure(rho: f64, temperature: f64, mu: f64) → f64` | $P=\rho k_B T/(\mu m_H)$ | `astronomy::stellar` |
| Adiabatic sound speed | `adiabatic_sound_speed(temperature: f64, mu: f64) → f64` | $c_s=\sqrt{\gamma k_B T/\mu}$ | `astronomy::stellar` |
| pp-chain luminosity | `pp_chain_luminosity(mass_kg: f64, x_hydrogen: f64) → f64` | Proton-proton chain rate | `astronomy::stellar` |
| CNO cycle luminosity | `cno_cycle_luminosity(mass_kg: f64, x_hydrogen: f64, z_metals: f64) → f64` | CNO cycle energy output | `astronomy::stellar` |
| Kelvin-Helmholtz timescale | `kelvin_helmholtz_timescale(mass: f64, radius: f64, luminosity: f64) → f64` | $t_{KH}=GM^2/(2RL)$ | `astronomy::stellar` |
| Nuclear timescale | `nuclear_timescale(mass: f64, luminosity: f64) → f64` | $t_{nuc}=\epsilon M c^2/L$ | `astronomy::stellar` |
| WD radius from mass | `white_dwarf_radius_from_mass(mass_kg: f64) → f64` | Mass-radius relation | `astronomy::stellar` |
| Eddington lum. (numerical) | `eddington_luminosity_numerical(mass_solar: f64) → f64` | $L_{Edd}$ in solar masses | `astronomy::stellar` |
| Helium mass | `helium_mass() → f64` | $m_{He}=4$ amu | `astronomy::stellar` |
| Solar composition | `solar_composition() → (f64, f64, f64)` | (H, He, metals) fractions | `astronomy::stellar` |
| Solar effective temperature | `solar_effective_temperature() → f64` | 5778 K | `astronomy::stellar` |
| Solar absolute magnitude | `solar_absolute_magnitude() → f64` | $M_V = 4.83$ | `astronomy::stellar` |
| Solar metallicity | `solar_metallicity() → f64` | $Z_\odot = 0.0134$ | `astronomy::stellar` |
| Solar MS lifetime | `solar_main_sequence_lifetime() → f64` | ~10 Gyr | `astronomy::stellar` |
| H burning energy/kg | `hydrogen_burning_energy_per_kg() → f64` | $\varepsilon = 0.007 c^2$ | `astronomy::stellar` |
| Eddington ratio | `eddington_ratio(luminosity: f64, mass: f64) → f64` | $\Gamma = L/L_{Edd}$ | `astronomy::stellar` |
| Sun core temperature | `sun_core_temperature() → f64` | ~1.57×10⁷ K | `astronomy::stellar` |
| Sun surface temperature | `sun_surface_temperature() → f64` | 5778 K | `astronomy::stellar` |
| Sun core density | `sun_core_density() → f64` | ~1.5×10⁵ kg/m³ | `astronomy::stellar` |
| Sun age | `sun_age() → f64` | ~4.6 Gyr | `astronomy::stellar` |
| Sun rotation period | `sun_rotation_period() → f64` | Sidereal period (s) | `astronomy::stellar` |
| Solar density | `solar_density() → f64` | Mean density (kg/m³) | `astronomy::stellar` |
| Solar surface gravity | `solar_surface_gravity() → f64` | Surface g (m/s²) | `astronomy::stellar` |
| Sun L/M ratio | `sun_luminosity_to_mass_ratio() → f64` | $L_\odot/M_\odot$ | `astronomy::stellar` |
