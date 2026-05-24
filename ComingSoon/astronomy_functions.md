## 5️⃣ Tidal Physics

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Tidal potential | `tidal_potential(M, r, d, k2, θ) → f64` | $-\frac{GM r^2}{d^3}(1+k_2)\left(\frac{3}{2}\cos^2\theta - \frac{1}{2}\right)$ — P₂ Legendre with Love number k₂ | astronomy::tides |
| Tidal bulge height | `tidal_bulge_height(a_tidal, R, g, h2) → f64` | $h_2 \cdot \frac{a_{\text{tidal}} \cdot R}{g}$ — equilibrium tide height with Love number h₂ | astronomy::tides |
| Spring tide amplitude | `spring_tide_amplitude(h_moon, h_sun) → f64` | Moon + Sun combined bulge heights | astronomy::tides |
| Neap tide amplitude | `neap_tide_amplitude(h_moon, h_sun) → f64` | |Moon − Sun| differential bulge heights | astronomy::tides |
| Tidal dissipation rate | `tidal_dissipation_rate(k2, n, M, R, Q, d) → f64` | $\frac{21}{2}\frac{k_2 n G M^2 R^5}{Q d^6}$ — Kaula tidal dissipation (W) | astronomy::tides |
| Tidal locking timescale | `tidal_locking_timescale(ω, a, μ, Q, M, R) → f64` | $\frac{\omega a^6 \mu Q}{6 G M^2 R^5}$ — spin-down locking time | astronomy::tides |

## 6️⃣ Rotational / Planetary Dynamics

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Surface velocity at latitude | `surface_velocity_at_latitude(ω, R, φ) → f64` | $v = \omega R \cos\varphi$ | astronomy::rotation |
| Centripetal acceleration | `centripetal_acceleration(ω, R, φ) → f64` | $a = \omega^2 R \cos\varphi$ | astronomy::rotation |
| Coriolis parameter | `coriolis_parameter(ω, φ) → f64` | $f = 2\omega\sin\varphi$ | astronomy::rotation |
| Moment of inertia | `moment_of_inertia(C, M, R) → f64` | $I = C \cdot MR^2$ — dimensionless MoI factor | astronomy::rotation |
| Rotational kinetic energy | `rotational_kinetic_energy(I, ω) → f64` | $\frac{1}{2}I\omega^2$ | astronomy::rotation |
| Nutation obliquity | `nutation_obliquity_rad(Ω) → f64` | $\Delta\varepsilon = 9.2'' \cos\Omega$ — IAU 1980 dominant 18.6-yr term | astronomy::rotation |
| Day length variation | `day_length_variation(doy, φ, tilt) → f64` | Sunrise equation: $\cos h_0 = -\tan\varphi\tan\delta$ | astronomy::rotation |

## 7️⃣ General Relativity Corrections

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| GR perihelion precession | `gr_perihelion_precession(M, a, e) → f64` | $\Delta\phi = \frac{6\pi GM}{a(1-e^2)c^2}$ rad/orbit | astronomy::relativity |
| Kepler equation solver | `solve_kepler(M, e, tol) → f64` | $E - e\sin E = M$ — Newton-Raphson iterative solver | astronomy::orbits |

## 8️⃣ Impact / Cratering Physics

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Crater diameter | `crater_diameter(ρ_i, d_p, v, g, ρ_t) → f64` | Pi-scaling (Holsapple): $D \propto \rho_i^{1/3} d_p^{0.78} v^{0.44} g^{-0.22}$ | astronomy::impacts |
| Fireball radius | `fireball_radius(E_kt) → f64` | $R = 55 \cdot E_{kt}^{0.4}$ | astronomy::impacts |
| Ejecta volume | `ejecta_volume(D, d) → f64` | Truncated paraboloid ejecta volume | astronomy::impacts |
| Impact velocity | `impact_velocity(v_inf, v_esc) → f64` | $v = \sqrt{v_\infty^2 + v_{esc}^2}$ — hyperbolic approach | astronomy::impacts |
| Ejecta escape fraction | `ejecta_escape_fraction(v_esc, v_ejecta) → f64` | Velocity-dependent fraction | astronomy::impacts |
