# Astronomy Module

The **Astronomy** module covers **9 submodules** providing computational tools for astrophysics, from the mechanics of planetary orbits to the large-scale structure and evolution of the universe. Every function works in SI units unless stated otherwise, and all constants (G, c, σ) are embedded as compile-time literals.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This section defines the scope: which problem this module solves, which abstraction level it targets, and which outputs are expected.

### Modeling Assumptions
- The equations are numerical models, not analytical proofs.
- Inputs must respect the documented units.
- Validity domains (linear regime, local approximation, and so on) should be verified by the reader.

### Reading Strategy
1. Read the module structure.
2. Identify equations and their conditions.
3. Map these equations to Rust functions in `docs/code/`.


## Reading Guide

### Goal
This page explains the module from a scientific perspective: assumptions, models, equations, and usage limits.

### How To Read This Document
- Start with the domain section relevant to your case.
- Identify key equations and their usage conditions.
- Then verify the corresponding Rust signatures in `docs/code/`.

### Conventions
- Units: SI unless explicitly stated.
- Variables: standard mathematical notation for the domain.
- Equations are presented as reference models, not formal proofs.


## Chapter 2 - Submodules and Models

### 1. Orbits

Orbital mechanics — the mathematical description of how bodies move under gravity — is the backbone of mission planning, satellite operations, and planetary science.

**Kepler's Laws** — The orbital period of a body on an elliptical path is $T = 2\pi\sqrt{a^3/\mu}$, where $a$ is the semi-major axis and $\mu = GM$ is the gravitational parameter. The vis-viva equation $v^2 = \mu(2/r - 1/a)$ gives the instantaneous speed at any orbital radius, unifying circular ($a = r$), elliptical, parabolic ($a \to \infty$), and hyperbolic cases. The module computes circular velocity $v_c = \sqrt{\mu/r}$, escape velocity $v_e = \sqrt{2\mu/r}$, specific orbital energy $\varepsilon = -\mu/2a$, and specific angular momentum $h = \sqrt{\mu a(1-e^2)}$.

**Orbit Geometry** — Periapsis $r_p = a(1-e)$ and apoapsis $r_a = a(1+e)$ define the extremes. The conic section equation $r(\theta) = a(1-e^2)/(1+e\cos\theta)$ maps true anomaly to radius. These are the building blocks for trajectory design.

**Transfer Orbits** — The Hohmann transfer between two circular orbits requires two impulsive burns: $\Delta v = |v_{t1} - v_1| + |v_2 - v_{t2}|$, where $v_{t}$ comes from vis-viva evaluated on the transfer ellipse with $a_t = (r_1+r_2)/2$. This is the most fuel-efficient two-impulse transfer in a central field.

**Gravitational Domains** — The sphere of influence $r_{SOI} = a(m/M)^{2/5}$ defines the region where a planet's gravity dominates over the star's. The Hill sphere $r_H = a(1-e)(m/3M)^{1/3}$ bounds the region where stable satellite orbits can exist. The Roche limit $d = 2.456\,R_p(\rho_p/\rho_s)^{1/3}$ is the minimum orbital distance before tidal forces tear a fluid satellite apart.

---

### 2. Stellar Physics

Stellar astrophysics characterizes stars by their observable and intrinsic properties, connecting luminosity, temperature, mass, and evolutionary state.

**Luminosity and Temperature** — A star radiates as an approximate blackbody: $L = 4\pi R^2 \sigma T_{\text{eff}}^4$ (Stefan-Boltzmann law). Wien's displacement law $\lambda_{\max} = 2.898 \times 10^{-3}/T$ gives the peak emission wavelength. The module computes luminosity from radius and temperature, and the inverse.

**Magnitude System** — Absolute magnitude $M = m - 5\log_{10}(d/10)$ standardizes brightness at 10 pc. The distance modulus inverts this to give distance from apparent and absolute magnitudes. Stellar flux follows the inverse-square law $F = L/(4\pi d^2)$. Bolometric corrections convert visual magnitudes to total luminosity.

**Mass-Luminosity Relation** — On the main sequence, luminosity scales steeply with mass: $L \propto M^{2.3}$ for $M < 0.43\,M_\odot$, $L \propto M^4$ for solar-type stars, $L \propto M^{3.5}$ for intermediate masses, and $L \propto M$ for the most massive stars. Main-sequence lifetime follows as $\tau \approx 10^{10}\,M/L$ years — massive stars burn out in millions of years, while red dwarfs last trillions.

**Spectral Classification** — An empirical relation maps spectral index to effective temperature: $T_{\text{eff}} \approx 42000/(n+0.92)$, bridging the O-B-A-F-G-K-M sequence to physical temperature.

**Compact Objects** — The Schwarzschild radius $r_s = 2GM/c^2$ defines the event horizon of a non-rotating black hole. The Chandrasekhar limit ($1.4\,M_\odot$) is the maximum mass for a white dwarf supported by electron degeneracy pressure. The Eddington luminosity $L_E = 4\pi GMc/\kappa$ is the maximum luminosity before radiation pressure overcomes gravity, limiting accretion rates.

---

### 3. Cosmology

Cosmology studies the origin, structure, and evolution of the universe on the largest scales, grounded in general relativity and the Friedmann-Lemaître-Robertson-Walker metric.

**Hubble's Law and H(z)** — The expansion of the universe produces a recession velocity proportional to distance: $v = H_0 d$. Beyond local linear expansion, the module exposes redshift-dependent Hubble rates via $H(z) = H_0 E(z)$, including general and flat-LCDM helpers.

**Redshift** — Cosmological redshift $z = a_{\text{obs}}/a_{\text{emit}} - 1$ relates the scale factor at emission to the observed wavelength stretching. The scale factor is $a = 1/(1+z)$. For low redshift, $z \approx v/c$ (non-relativistic approximation).

**Friedmann and E(z) parameterizations** — The expansion rate $H(a) = H_0\sqrt{\Omega_r/a^4 + \Omega_m/a^3 + \Omega_\Lambda}$ governs cosmic dynamics. The module provides direct `E(z)` implementations for general models and common parameterizations: flat LCDM, LCDM with radiation, constant-$w$ dark energy (wCDM), and CPL ($w_0$-$w_a$).

**Distances** — Cosmological distances are not unique. The module computes line-of-sight and transverse comoving distance, luminosity distance $d_L = d_C(1+z)$, and angular diameter distance $d_A = d_C/(1+z)$ from redshift using numerical integration. Implementations are available for flat LCDM, general curved models, and flat wCDM/CPL variants.

**Time and LCDM shortcuts** — The module keeps approximate legacy formulas, but also includes LCDM-focused utilities such as analytical flat-LCDM age where applicable, plus numerical lookback/age functions for more general models.

**CMB Temperature** — The cosmic microwave background temperature scales with redshift: $T(z) = T_0(1+z)$, where $T_0 \approx 2.725$ K today.

**Hub wiring** — Cosmology APIs are directly exposed through the Hub astronomy dispatcher, so these calculations are available from `Experiment::new(DomainType::Astronomy, ...)` without bypassing the unified dispatch layer.

---

### 4. Celestial Mechanics

Celestial mechanics applies Newtonian gravity to multi-body systems, covering forces, tidal interactions, and observable geometry.

**Gravitational Force** — Newton's law $F = Gm_1m_2/r^2$ and the tidal force $F_t = 2Gm\,\Delta r/r^3$ (differential gravity across an extended body). Tidal forces explain ocean tides, tidal locking, and the Roche limit.

**Two-Body Geometry** — The barycenter of two masses lies at $d \cdot m_2/(m_1+m_2)$ from $m_1$. The synodic period $P_{\text{syn}} = 1/|1/P_1 - 1/P_2|$ gives the recurrence time of alignments (e.g., planetary conjunctions). Sidereal-to-solar day conversion accounts for orbital motion.

**Lagrange Points and Hill Sphere** — The L1 point $r_{L1} \approx d(m_2/3m_1)^{1/3}$ is the equilibrium between gravitational pulls, used for solar observatories (e.g., SOHO). The Hill sphere bounds stable satellite orbits.

**Surface Properties** — Surface gravity $g = GM/R^2$ determines atmospheric retention and landing dynamics. The apparent angular size $\theta = 2\arctan(d_{\text{phys}}/2D)$ gives how large an object appears in the sky. Parallax distance $d = 1/p''$ (in parsecs) is the fundamental distance ladder rung.

**Habitable Zone** — The circumstellar habitable zone bounds are estimated from stellar luminosity: inner edge $r_{\text{in}} = \sqrt{L/1.1}$ AU and outer edge $r_{\text{out}} = \sqrt{L/0.53}$ AU, where $L$ is in solar luminosities. These correspond to the range where liquid water can exist on a rocky surface.

---

### 5. Galactic Astrophysics

Structure, dynamics, and evolution of the Milky Way and its interaction with the Local Group.

**Galactic Rotation Curve** — The circular velocity at galactocentric radius $r$ follows from an enclosed-mass profile $M(r) = M_{\text{MW}}\,r/(r + R_d)$, giving $v_c(r) = \sqrt{GM(r)/r}$ where $R_d$ is the disk scale length. The flat rotation curve at large radii ($v_c \approx 220\;\text{km/s}$) is a key piece of evidence for dark matter, since visible matter alone predicts a Keplerian decline $v \propto r^{-1/2}$.

**Solar Galactic Parameters** — The Sun orbits at $R_\odot \approx 8.2\;\text{kpc}$ with a velocity $v_\odot \approx 220\;\text{km/s}$ and an orbital period $T_\odot \approx 225\;\text{Myr}$ (one galactic year). These are stored as CODATA/IAU-verified constants.

**Sagittarius A\*** — The central supermassive black hole ($M \approx 4 \times 10^6\,M_\odot$) has a Schwarzschild radius $r_s = 2GM/c^2$ and a gravitational sphere of influence $r_{\text{infl}} = GM/\sigma^2$ where $\sigma$ is the local stellar velocity dispersion (approximated here by $v_\odot$).

**Milky Way – Andromeda Interaction** — The approach time $t = d/v_r$ gives a first-order collision estimate from the current separation $d \approx 2.54\;\text{Mly}$ and radial approach velocity $v_r \approx 110\;\text{km/s}$. The two-body reduced mass $\mu = M_1 M_2/(M_1 + M_2)$ is used for energy-budget estimates of the merger.

**Disk Density Model** — The stellar mass density follows an exponential-disk profile:

$$\rho(r,z) = \frac{\Sigma_0}{2h_z}\exp\!\left(-\frac{r}{R_d}\right)\exp\!\left(-\frac{|z|}{h_z}\right)$$

where $\Sigma_0$ is the central surface density, $R_d$ the radial scale length, and $h_z$ the vertical scale height. The stellar number density uses the same functional form normalized to the total star count.

**Oort Constants and Epicyclic Frequency** — The Oort constants $A = \frac{1}{2}\!\left(\frac{v}{r} - \frac{dv}{dr}\right)$ and $B = -\frac{1}{2}\!\left(\frac{v}{r} + \frac{dv}{dr}\right)$ describe local differential rotation and are measured from proper motions. The epicyclic frequency $\kappa = \sqrt{-4B(A-B)}$ governs the radial oscillation of stellar orbits around their guiding center.

**Galactic Escape Velocity** — $v_{\text{esc}} = \sqrt{2GM(r)/r}$ determines the minimum speed for unbound orbits. The dynamical time $t_{\text{dyn}} = 2\pi r / v_c$ is the characteristic orbital crossing time.

**Bulge and Tidal Radius** — The bulge mass within radius $r$ uses a Hernquist-like profile: $M_b(r) = 0.15\,M_* r^3/(r^3 + r_b^3)$, where $r_b$ is the bulge scale radius. The tidal (Jacobi) radius of a star cluster at galactocentric distance $R$ is $r_t = R\,(m_c/3M_g)^{1/3}$, the boundary at which galactic tides strip cluster members.

---

### 6. Impact Cratering

Hypervelocity impact mechanics — crater formation, fireball evolution, and ejecta dynamics.

**Crater Scaling** — The transient crater diameter follows an energy–gravity scaling law:

$$D = K\left(\frac{\rho_i}{\rho_t}\right)^{\!\alpha} d_p^{\,\beta}\, v^{\gamma}\, g^{\delta}$$

where $K$, $\alpha$, $\beta$, $\gamma$, $\delta$ are empirical exponents from laboratory and numerical hydrocode experiments (Pi-group scaling sensu Holsapple). The density ratio accounts for the impedance mismatch between impactor and target.

**Fireball Radius** — The thermal emission sphere of a large impact follows a power-law in energy: $R_{\text{fb}} = K_{\text{fb}}\,E^{\alpha_{\text{fb}}}$, with $E$ expressed in kilotons TNT equivalent. This scaling derives from nuclear test analogs (Sedov-Taylor blast wave).

**Ejecta Volume** — Approximated as a paraboloid: $V_{\text{ej}} = \frac{\pi}{6}\,D^2\,h$ where $D$ is the crater diameter and $h$ the excavation depth.

**Impact Velocity** — Combining the approach velocity $v_\infty$ (hyperbolic excess) with the gravitational acceleration: $v_{\text{imp}} = \sqrt{v_\infty^2 + v_{\text{esc}}^2}$, yielding the actual surface encounter speed.

**Ejecta Escape Fraction** — The fraction of ejecta exceeding escape velocity: $f = \max\!\left(0,\;1 - (v_{\text{esc}}/v_{\text{ej}})^2\right)$. On low-gravity bodies (asteroids, comets), most ejecta escapes; on Earth, essentially none does.

---

### 7. Planetary Science

Interior structure, atmospheric physics, and surface thermodynamics of planets and moons.

**Interior Structure** — Hydrostatic pressure at depth: $P = \rho g d$. Central pressure for a uniform sphere: $P_c = \frac{3}{2}\rho g R$. The adiabatic temperature gradient $dT/dz = g/c_p$ governs the convective interior of gas giants. The moment-of-inertia factor $C/MR^2$ constrains density stratification — a uniform sphere has $C/MR^2 = 0.4$; differentiated bodies (iron core + silicate mantle) have lower values.

**Tidal Physics** — The Love number $k_2 = 1.5/(1 + 19\mu/(2G\rho^2 R^2/3))$ measures the body's tidal deformability, where $\mu$ is the shear rigidity. Tidal heating $\dot{E} = \frac{21}{2}\frac{k_2}{Q}\frac{G M_p^2 R^5 n}{a^6}e^2$ powers Io's volcanism and Europa's subsurface ocean ($Q$ is the tidal quality factor). The tidal locking timescale estimates when spin-orbit synchronization is reached.

**Roche Limits** — The fluid Roche limit $d = 2.456\,R_p(\rho_p/\rho_s)^{1/3}$ and the rigid Roche limit $d = 1.26\,R_p(\rho_p/\rho_s)^{1/3}$ bound the region inside which a satellite is torn apart. The factor difference (2.456 vs 1.26) reflects the role of material strength.

**Atmospheric Physics** — The equilibrium temperature $T_{\text{eq}} = \left(\frac{L(1-A)}{16\pi\sigma a^2}\right)^{1/4}$ is the blackbody temperature balanced by stellar insolation. The greenhouse surface temperature adds infrared optical depth: $T_s = T_{\text{eq}}(1 + \frac{3}{4}\tau_{\text{IR}})^{1/4}$. Atmospheric scale height $H = k_B T/(\bar{m}g)$. Jeans escape parameter $\lambda = (v_{\text{esc}}/v_{\text{th}})^2$ determines thermal atmospheric loss.

**Magnetosphere** — The magnetopause standoff distance $r_{\text{mp}} = \left(\frac{\mu_0\,\mathcal{M}^2}{8\pi^2\,P_{\text{sw}}}\right)^{1/6}$ balances magnetic pressure against solar wind dynamic pressure, where $\mathcal{M}$ is the planetary dipole moment.

**Ring Systems** — The inner Roche boundary sets the minimum ring radius. Optical depth $\tau = 3\Sigma/(4\rho_p r_p)$ relates the surface mass density $\Sigma$ to particle size. The synchronous orbit $r_{\text{sync}} = (GM/\omega^2)^{1/3}$ marks the geostationary distance.

**Surface Properties** — Oblateness $f = q/2$ (where $q = \omega^2 R^3/GM$), orbital precession from $J_2$, Bond–geometric albedo relation $A_B = p\,q_\phi$, thermal inertia $\Gamma = \sqrt{k\rho c_p}$, diurnal skin depth $\delta = \sqrt{\kappa P/\pi}$, and subsolar/nightside temperatures from radiative equilibrium.

**Planetary Database** — A lookup table for all 8 Solar System planets (Mercury–Neptune) with 14 physical and orbital parameters each (mass, radius, flattening, orbital period, semi-major axis, eccentricity, inclination, axial tilt, sidereal day, surface gravity, escape velocity, mean density, Bond albedo, orbital velocity), sourced from NASA/JPL ephemeris data.

---

### 8. Rotation

Rotational dynamics — surface velocity, Coriolis effects, precession, and photoperiod.

**Surface Velocity** — At latitude $\phi$, $v = \omega R \cos\phi$ gives the linear speed due to axial rotation. The centripetal acceleration is $a = \omega^2 R \cos\phi$, which reduces the effective gravity at the equator by $\sim 0.3\%$.

**Coriolis Parameter** — $f = 2\omega\sin\phi$ controls large-scale atmospheric and oceanic circulation. $f = 0$ at the equator and maximal at the poles. The Coriolis acceleration deflects moving objects to the right in the Northern Hemisphere and to the left in the Southern Hemisphere.

**Moment of Inertia** — $I = C\,MR^2$ where the dimensionless factor $C$ encodes mass distribution. $C = 0.4$ for a uniform sphere; Earth's $C \approx 0.331$ reveals a dense core. The rotational kinetic energy is $E_{\text{rot}} = \frac{1}{2}I\omega^2$.

**Nutation** — Small periodic oscillation of the rotation axis: $\Delta\varepsilon = A_{\text{nut}}\cos\Omega_n$ (arcseconds), where $\Omega_n$ is the longitude of the ascending node of the lunar orbit (18.6-year period). The amplitude $A_{\text{nut}} \approx 9.2''$ for the Earth.

**Day Length Variation** — Hours of daylight as a function of day of year, latitude, and axial tilt. The solar declination $\delta = \varepsilon\sin(2\pi(d - d_{\text{eq}})/365)$ determines the hour angle at sunrise/sunset: $h_0 = \arccos(-\tan\phi\,\tan\delta)$, giving daylight duration $D = 24\,h_0/\pi$ hours. Polar day ($D = 24$) and polar night ($D = 0$) emerge naturally.

---

### 9. Tides

Gravitational tidal interactions — potentials, bulge heights, spring/neap cycles, and dissipation.

**Tidal Potential** — The degree-2 tidal potential at the surface of a body:

$$\Phi = -\frac{GM r^2}{d^3}(1 + k_2)\,P_2(\cos\theta)$$

where $P_2(x) = \frac{3}{2}x^2 - \frac{1}{2}$ is the second Legendre polynomial, $d$ the distance to the perturber, $r$ the body radius, and $k_2$ the Love number. The factor $(1 + k_2)$ accounts for the body's elastic response.

**Tidal Bulge Height** — The equilibrium tidal displacement: $\eta = h_2\,a_{\text{tidal}}\,r/g$ where $h_2$ is the displacement Love number and $a_{\text{tidal}}$ the tidal acceleration. For the Earth-Moon system, the equilibrium oceanic tide amplitude is $\sim 0.5\;\text{m}$.

**Spring and Neap Tides** — When the Sun and Moon are aligned (new or full Moon), their tidal bulges add: $h_{\text{spring}} = h_{\text{Moon}} + h_{\text{Sun}}$. At quadrature (first/third quarter), they partially cancel: $h_{\text{neap}} = |h_{\text{Moon}} - h_{\text{Sun}}|$. The ratio $h_{\text{Moon}}/h_{\text{Sun}} \approx 2.17$ reflects the Moon's proximity compensating for its smaller mass.

**Tidal Dissipation** — Energy dissipated by tidal friction:

$$\dot{E} = \frac{21}{2}\frac{k_2\,n\,G\,M_p^2\,R^5}{Q\,d^6}$$

where $n$ is the mean motion and $Q$ the tidal quality factor. Low $Q$ (high dissipation) drives orbital circularization, spin-orbit synchronization, and satellite migration. The Moon recedes from Earth at $\sim 3.8\;\text{cm/yr}$ due to tidal dissipation in Earth's oceans.

**Tidal Locking Timescale** — $\tau_{\text{lock}} = \omega a^6 \mu Q / (6GM_p^2 R^5)$ estimates the time to achieve synchronous rotation, where $\mu$ is the rigidity and $\omega$ the initial spin rate. Mercury, the Moon, and most large moons in the Solar System are tidally locked.

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` rounding errors can accumulate in long simulations.
- Extreme regimes (very large or very small scales) require explicit numerical stability checks.

### Recommended Verification
- Compare against a simple analytical case when available.
- Check the order of magnitude of results.
- Run sensitivity analysis on dominant parameters.

### Transition to Implementation
For concrete function calls, Rust signatures and module paths are documented in `docs/code/`.
