# Meteorology Module

The **Meteorology** module spans **8 submodules** covering atmospheric science: thermodynamics and composition, radiative transfer, atmospheric dynamics, and precipitation hydrology. It provides computational tools for weather analysis, climate modeling, and hydrological engineering.

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

### 1. Atmosphere

Vertical structure, thermodynamics, and moisture physics of the troposphere and stratosphere.

**Barometric Formula** — Pressure decreases exponentially with altitude: $P(h) = P_0\exp\!\left(-\frac{Mgh}{R^*T}\right)$ where $M$ is the mean molar mass of air, $g$ is gravity, and $R^* = 8.314\;\text{J mol}^{-1}\text{K}^{-1}$. The atmospheric scale height $H = R^*T/(Mg) \approx 8.5\;\text{km}$ is the e-folding length for pressure.

**Lapse Rates** — The dry adiabatic lapse rate $\Gamma_d = g/c_p \approx 9.8\;\text{K/km}$ applies to unsaturated air. The moist (saturated) adiabatic lapse rate is lower because latent heat release partially offsets cooling:

$$\Gamma_m = g\,\frac{1 + L_v r_s/(R_d T)}{1 + L_v^2 r_s/(c_p R_v T^2)}$$

where $L_v$ is the latent heat of vaporization, $r_s$ the saturation mixing ratio, $R_d$ and $R_v$ the gas constants for dry air and water vapor.

**Potential Temperature** — $\theta = T\left(\frac{P_0}{P}\right)^{R_d/c_p}$ is conserved for adiabatic displacements and serves as the natural vertical coordinate for stability analysis.

**Virtual Temperature** — $T_v = T(1 + 0.608\,r)$ accounts for the reduced density of moist air, where $r$ is the mixing ratio.

**Moisture Variables** — Mixing ratio $r = 0.622\,e/(P-e)$, saturation vapor pressure (Magnus formula) $e_s = 6.112\exp(17.67\,T_C/(T_C+243.5))$ hPa, relative humidity $\text{RH} = e/e_s \times 100\%$, and dew point from the inverse Magnus equation.

**Density Altitude** — Corrects pressure altitude for non-standard temperature: $h_\rho = h_P + 120(T - T_{\text{ISA}})$, critical for aviation performance.

**Brunt-Väisälä Frequency** — $N = \sqrt{(g/\theta)\,d\theta/dz}$ measures the static stability of the atmosphere. $N^2 > 0$ → stable (oscillatory); $N^2 < 0$ → unstable (convective).

---

### 2. Dynamics

Large-scale atmospheric motion governed by the balance between pressure gradient, Coriolis, and inertial forces.

**Coriolis Parameter** — $f = 2\Omega\sin\phi$ where $\Omega = 7.292 \times 10^{-5}\;\text{rad/s}$ and $\phi$ is the latitude. $f = 0$ at the equator and maximal at the poles.

**Geostrophic Wind** — In the free atmosphere where friction is negligible, the pressure gradient is balanced by the Coriolis force:

$$u_g = -\frac{1}{\rho f}\frac{\partial P}{\partial y}, \qquad v_g = \frac{1}{\rho f}\frac{\partial P}{\partial x}$$

The wind blows parallel to isobars with low pressure to the left (Northern Hemisphere).

**Rossby Number** — $Ro = U/(fL)$ compares inertial to Coriolis accelerations. $Ro \ll 1$ → quasi-geostrophic regime (synoptic scale); $Ro \sim 1$ → mesoscale (tornadoes, sea breezes).

**Rossby Waves** — Planetary-scale waves with phase speed $c = -\beta/k^2$ where $\beta = df/dy$ is the meridional gradient of Coriolis. These waves control teleconnection patterns and jet-stream meanders.

**Thermal Wind** — Vertical shear of the geostrophic wind due to horizontal temperature gradients: $\frac{\partial \vec{v}_g}{\partial \ln p} = \frac{R_d}{f}\hat{k} \times \nabla_p T$. This links upper-level jet streams to surface temperature contrasts.

**Potential Vorticity** — Ertel's PV $q = -g\,f\,\partial\theta/\partial p$ is conserved for adiabatic, frictionless flow — a powerful diagnostic for tropopause dynamics and stratospheric intrusions.

**Ekman Layer** — Friction-influenced boundary layer of depth $D_E = \pi\sqrt{2\nu/|f|}$, within which winds cross isobars toward low pressure (surface convergence in cyclones).

**Richardson Number** — $Ri = N^2/(du/dz)^2$ balances buoyancy suppression against shear production of turbulence. $Ri < 0.25$ → onset of Kelvin-Helmholtz instability and turbulent mixing.

**Rossby Deformation Radius** — $L_R = NH/|f|$ is the horizontal scale at which rotation becomes as important as buoyancy, controlling the width of fronts and the size of baroclinic eddies.

**Gradient Wind** — Curved-flow solution including centrifugal acceleration: $V = -\frac{fR}{2} + \sqrt{\frac{f^2R^2}{4} + \frac{R}{\rho}\frac{\partial P}{\partial r}}$, applicable to tightly curved cyclones and anticyclones.

---

### 3. Precipitation

Rainfall characterization, evapotranspiration, and surface hydrology.

**Marshall-Palmer Distribution** — Radar reflectivity $Z$ to rain rate $R$ conversion: $Z = 200\,R^{1.6}$ (mm⁶/m³) or equivalently $R = (Z/200)^{1/1.6}$ mm/h. An empirical relation foundational to weather radar operations.

**Raindrop Terminal Velocity** — $v_t = 9.65 - 10.3\,e^{-0.6D}$ m/s where $D$ is the drop diameter in mm, capturing the transition from Stokes drag (small drops) to turbulent drag (large drops).

**Thornthwaite PET** — Monthly potential evapotranspiration from temperature alone: $\text{PET} = 16\,(N/12)(10T/I)^a$ mm/month, where $I$ is the annual heat index and $a$ a polynomial function of $I$.

**Penman Equation** — Combines energy balance and aerodynamic mass transfer: $E = \frac{\Delta\,R_n + \gamma\,f(u)\,e_a}{\Delta + \gamma}$ where $\Delta$ is the slope of the saturation vapor pressure curve, $\gamma$ the psychrometric constant, $R_n$ the net radiation, and $f(u)$ a wind function.

**Intensity-Duration-Frequency (IDF)** — Engineering design curves: $i = \frac{a\ln T_r}{t_d + b}$ where $T_r$ is the return period (years) and $t_d$ the storm duration. Used for storm sewer, culvert, and dam design.

**SCS Curve Number** — USDA runoff model: $Q = (P - I_a)^2/(P - I_a + S)$ where $S = 25400/CN - 254$ mm is the potential maximum retention and $I_a = 0.2S$ is the initial abstraction. $CN$ ranges from 30 (permeable forest) to 98 (impervious surface).

**Rational Method** — Peak runoff $Q_p = CiA$ for small catchments ($< 80$ ha), where $C$ is the runoff coefficient, $i$ the rainfall intensity for the time of concentration, and $A$ the drainage area.

**Unit Hydrograph** — Peak discharge $Q_p = 2.08\,A/t_p$ (SCS triangular approximation) where $A$ is the basin area (km²) and $t_p$ the time to peak (hours).

**Antecedent Precipitation Index** — $\text{API}_n = k \cdot \text{API}_{n-1} + P_n$ with decay factor $k \approx 0.85$–$0.95$, tracking soil moisture from recent rainfall history.

---

### 4. Radiation

Solar and terrestrial radiation budgets governing Earth's energy balance and climate.

**Stefan-Boltzmann Law** — Total radiative flux from a surface: $F = \sigma T^4$ where $\sigma = 5.67 \times 10^{-8}\;\text{W m}^{-2}\text{K}^{-4}$. For a non-blackbody: $F = \varepsilon\sigma T^4$ (emissivity $0 < \varepsilon \le 1$).

**Solar Constant** — Total solar irradiance at the top of the atmosphere: $S_0 = 1361\;\text{W/m}^2$ (mean Earth-Sun distance).

**Planetary Energy Balance** — Absorbed solar radiation balances outgoing longwave: $\frac{S_0(1-\alpha)}{4} = \sigma T_e^4$, giving the effective temperature $T_e = \left(\frac{S_0(1-\alpha)}{4\sigma}\right)^{1/4} \approx 255\;\text{K}$ for Earth ($\alpha \approx 0.3$).

**Greenhouse Effect** — The difference $\Delta T = T_s - T_e$ between the actual surface temperature ($T_s \approx 288\;\text{K}$) and the effective radiating temperature quantifies the warming by greenhouse gases ($\Delta T \approx 33\;\text{K}$).

**Beer-Lambert Atmospheric Extinction** — $I = I_0\,e^{-\tau}$ where the optical depth $\tau = \kappa\,\ell$ is the product of absorption/scattering coefficient and path length. Governs visibility, solar dimming, and radiative transfer through clouds.

**Planck Function** — Spectral radiance of a blackbody:

$$B(\lambda, T) = \frac{C_1}{\lambda^5\left(\exp\!\left(\frac{C_2}{\lambda T}\right) - 1\right)}$$

with $C_1 = 3.742 \times 10^{-16}\;\text{W m}^2$ and $C_2 = 1.4388 \times 10^{-2}\;\text{m K}$. The peak wavelength follows Wien's law $\lambda_{\max} = 2898/T$ µm.

**Solar Zenith Angle** — $\cos\theta_z = \sin\phi\sin\delta + \cos\phi\cos\delta\cos h$ where $\phi$ is latitude, $\delta$ the solar declination, and $h$ the hour angle. Controls the diurnal and seasonal variation of insolation.

**CO₂ Radiative Forcing** — The logarithmic dependence $\Delta F = 5.35\ln(C/C_0)\;\text{W/m}^2$ gives the additional downward infrared forcing from a change in atmospheric CO₂ concentration. The climate sensitivity parameter $\lambda = \Delta T/\Delta F$ (K per W/m²) links forcing to equilibrium temperature response.

---

### 5. Clouds

Cloud microphysics, radiative properties, and precipitation initiation — the bridge between thermodynamics and rainfall.

**Saturation Vapor Pressure** — The Magnus formula $e_s = 611.2\exp(17.67\,T_C/(T_C + 243.5))$ Pa gives the saturation vapor pressure over a flat water surface. This is the fundamental threshold: when ambient vapor pressure exceeds $e_s$, condensation begins.

**Cloud Base Altitude** — The lifting condensation level $z_{\text{LCL}} = (T - T_d)/\Gamma$ where $T - T_d$ is the dew-point depression and $\Gamma$ the lapse rate. A quick estimate: cloud base rises $\sim 125\;\text{m}$ per degree of dew-point deficit.

**Liquid Water Content** — The adiabatic LWC increases linearly above cloud base: $\text{LWC} \approx \rho_a\,(dq_s/dT)\,\Gamma\,\Delta z / T$, where $dq_s/dT$ is the Clausius-Clapeyron slope. Typical values range from $0.1$ to $0.3\;\text{g/m}^3$ in stratocumulus.

**Cloud Optical Depth and Albedo** — $\tau = 3\,\text{LWP}/(2\rho_w r_e)$ where LWP is the liquid water path (g/m²) and $r_e$ the effective droplet radius. The two-stream albedo approximation: $\alpha = \tau^*/(\tau^* + 2)$ with $\tau^* = \tau(1-g)$ and asymmetry parameter $g \approx 0.85$. Doubling droplet concentration (keeping LWP constant) increases albedo — the Twomey indirect aerosol effect.

**Droplet Growth — Condensation** — The diffusional growth rate $dr/dt = S/(r(F_k + F_d))$ where $S$ is the supersaturation and $F_k$, $F_d$ are thermodynamic and diffusion resistance terms. Growth is fast at first but slows as $r$ increases (parabolic growth law $r \propto \sqrt{t}$).

**Droplet Growth — Collision-Coalescence** — Once drops exceed $\sim 20\;\mu\text{m}$, gravitational collection dominates: $dm/dt = \pi(R_1 + R_2)^2\,E\,\Delta v\,\text{LWC}/4$ where $E$ is the collection efficiency and $\Delta v$ the differential terminal velocity. This warm-rain process produces precipitation in $\sim 20$ minutes.

**Autoconversion** — The Kessler-type threshold: rain begins when LWC exceeds a critical value ($\sim 0.5\;\text{g/m}^3$). The rate depends on droplet concentration: fewer, larger drops convert faster (clean maritime clouds rain more efficiently than polluted continental clouds).

**Ice Microphysics** — Ice crystal growth rate depends on temperature-dependent habit factor (plates, dendrites, columns) and supersaturation with respect to ice. The Bergeron process exploits the difference between saturation vapor pressure over water and over ice: in mixed-phase clouds, ice crystals grow at the expense of supercooled droplets.

**Radiative Properties** — Cloud emissivity $\varepsilon = 1 - e^{-\tau}$ approaches unity for optically thick clouds. The Henyey-Greenstein phase function $p(\cos\theta) = (1-g^2)/(1+g^2-2g\cos\theta)^{3/2}$ describes the angular scattering pattern. Cloud radiative forcing is the net difference between shortwave cooling (albedo) and longwave warming (greenhouse).

**CAPE and CIN** — Convective Available Potential Energy $\text{CAPE} = \int g\,(T_p - T_e)/T_e\,dz$ measures the energy available for thunderstorm updrafts. Convective Inhibition (CIN) is the negative contribution that must be overcome for convection to initiate.

---

### 6. Ocean

Physical oceanography — wave mechanics, circulation, thermohaline dynamics, and ocean-climate coupling.

**Wave Regimes** — Deep-water phase speed $c = \sqrt{g\lambda/2\pi}$ (gravity waves, depth $>\lambda/2$) and shallow-water phase speed $c = \sqrt{gh}$ (tsunamis, tides). The full dispersion relation $\omega^2 = gk\tanh(kh)$ bridges both regimes.

**Wave Spectra** — The Phillips equilibrium spectrum $S(k) \propto k^{-4}\exp(-g^2/(k^2 U^4))$ describes the high-frequency tail of a fully developed sea. The JONSWAP spectrum adds a peak enhancement factor $\gamma \approx 3.3$ for fetch-limited growth: $S(\omega) = \alpha g^2/\omega^5 \cdot e^{-1.25(\omega_p/\omega)^4} \cdot \gamma^r$, where the peak frequency $\omega_p$ depends on wind speed and fetch.

**Significant Wave Height** — $H_s$ scales with wind speed and fetch: $H_s = A\,(U^2/g)\,(gF/U^2)^B$ where $A,B$ are empirical coefficients. The wave period follows a similar fetch-dependent scaling.

**Stokes Drift** — Net mass transport by surface waves: $u_S = a^2\omega k\,\cosh[2k(z+h)]/(2\sinh^2 kh)$ — a second-order effect that drives Langmuir circulation and oil-spill trajectories.

**Ekman Dynamics** — Ekman transport $M = \tau/(\rho f)$ is the wind-driven mass flux perpendicular to the wind direction. The Ekman spiral describes the velocity profile: flow rotates $45°$ from the wind at the surface and decays exponentially with depth over the Ekman depth $D_E = \pi\sqrt{2K_z/|f|}$. Ekman pumping/suction at the base of the Ekman layer drives large-scale upwelling and downwelling.

**Thermohaline Circulation** — Seawater density depends on temperature and salinity via the UNESCO/TEOS-10 polynomial equation of state: $\rho(T,S) = \rho_0 + \alpha_T(T) + \beta_S(S) + \gamma_{TS}(T,S)$. The thermal expansion coefficient $\alpha \approx 2 \times 10^{-4}\;\text{K}^{-1}$ and haline contraction coefficient $\beta \approx 7.5 \times 10^{-4}\;\text{psu}^{-1}$ control the density-driven overturning.

**Mixed Layer Depth** — Determined by wind stirring and buoyancy flux, the mixed layer is the well-homogenized upper ocean. Internal wave speed $c_i = \sqrt{g'h}$ (where $g'$ is the reduced gravity across the pycnocline) governs baroclinic wave propagation.

**Geostrophic Currents** — $v_g = -(g/f)\,d\eta/dx$ from sea surface height gradients, measurable by satellite altimetry. Upwelling velocity $w = (1/\rho f)\,\partial\tau/\partial y$ links wind patterns to nutrient supply and biological productivity.

**Ocean Heat Content** — $Q = \rho c_p \int_0^D \Delta T\,dz$ integrates temperature anomalies over the water column — Earth's dominant thermal reservoir absorbing $>90\%$ of anthropogenic heating.

**Sea Ice and Sound** — Ice growth rate from the Stefan growth law. Sound speed in seawater $c(T,S,D)$ depends on temperature, salinity, and depth via the Medwin empirical formula — critical for sonar and acoustic thermometry. Sea-level rise from thermal expansion: $\Delta h = \alpha\,\Delta T\,D$. Henry's law $K_H(T)$ governs CO₂ solubility.

---

### 7. Storms

Tropical cyclones, severe convection, and storm intensity metrics.

**Potential Intensity** — The theoretical maximum wind speed of a tropical cyclone: $V_{\text{max}} = \sqrt{C_k/C_d \cdot \eta \cdot \Delta k}$, where $C_k/C_d$ is the ratio of enthalpy to drag exchange coefficients, $\eta$ is the thermodynamic efficiency (Carnot engine between SST and outflow temperature), and $\Delta k$ is the air-sea enthalpy disequilibrium. This Emanuel potential intensity framework sets an upper bound on hurricane strength.

**Accumulated Cyclone Energy** — $\text{ACE} = \sum v_i^2 \times 10^{-4}$ where $v_i$ are 6-hourly maximum sustained winds in knots. ACE integrates intensity over a storm's lifetime and is the standard seasonal activity metric.

**CAPE** — Convective Available Potential Energy $\text{CAPE} = g\,(T_p - T_e)/T_e \cdot \Delta z$ for a discrete layer; summed vertically, it gives the total energy available to thunderstorm updrafts. CAPE $> 2000\;\text{J/kg}$ indicates severe weather potential.

**Rossby Deformation Radius** — $L_R = NH/(|f|)$ where $N$ is the Brunt-Väisälä frequency and $H$ the tropopause height. $L_R$ controls the size of mid-latitude cyclones and the width of tropical cyclone eyewalls.

**Fujita Scale** — Tornado intensity classification from estimated wind speed: F0 ($<33\;\text{m/s}$) to F5 ($>89\;\text{m/s}$). An empirical discrete mapping from continuous wind fields.

---

### 8. Winds

Global and mesoscale wind systems — from planetary Hadley cells to local mountain breezes.

**Hadley Cell Extent** — The poleward limit of the thermally driven tropical cell: $\phi_H \approx \left(\frac{5g\,\Delta T\,R}{3\Omega^2 R^2}\right)^{1/4}$, where $\Delta T$ is the equator-to-pole temperature contrast and $\Omega$ the planetary rotation rate. Fast rotation compresses the Hadley cell toward the equator.

**Thermal Wind** — Vertical shear of the geostrophic wind: $\partial u/\partial z = -(g/fT)\,\partial T/\partial y$. Horizontal temperature gradients drive upper-level jet streams: the subtropical jet lies above the maximum meridional temperature gradient.

**Jet Stream** — The estimated jet velocity $U_J \approx gH\Delta T/(fT_m L)$ scales with the scale height $H$, meridional temperature contrast $\Delta T$, and the distance $L$ over which the gradient acts.

**Surface Wind** — $v = \sqrt{(\partial P/\partial x)/(\rho C_D)}$ where $C_D$ is the drag coefficient. Wind stress $\tau = \rho_a C_D U^2$ drives ocean circulation via Ekman transport.

**Planetary Vorticity** — $f = 2\Omega\sin\phi$ and its meridional gradient $\beta = 2\Omega\cos\phi/R$ control Rossby wave propagation. The Rossby wave phase speed $c = -\beta/(k^2 + 1/L_R^2)$ is always westward relative to the mean flow.

**Baroclinic Instability** — The most unstable wavelength $\lambda = 2\pi\sqrt{2}\,L_R$ sets the typical size of mid-latitude weather systems ($\sim 4000\;\text{km}$).

**Mesoscale Winds** — Sea breeze speed $v \approx \sqrt{g\,\Delta T\,h/T_m}$ where $h$ is the boundary-layer depth. Katabatic (downslope) wind: $v \approx \sqrt{g\,\Delta T\,\sin\alpha\,L/(T_m C_D)}$ driven by radiative cooling over sloped terrain. Mountain wave vertical velocity $w = U H_m N k$ controls aviation turbulence and orographic precipitation.

**Boundary Layer** — The Monin-Obukhov length $L_{MO} = -u_*^3 T/(\kappa g \overline{w'\theta'})$ characterizes the stability of the surface layer ($L_{MO} > 0$: stable; $L_{MO} < 0$: unstable). The logarithmic wind profile $u(z) = (u_*/\kappa)\ln(z/z_0)$ applies in the neutral surface layer.

**Curvature Corrections** — Cyclostrophic wind (centrifugal balance without Coriolis, e.g., tornadoes) and gradient wind (full nonlinear balance including both Coriolis and centrifugal forces) extend the geostrophic approximation to curved flow.

**Empirical Scales** — Beaufort-to-m/s conversion: $v = 0.836\,B^{3/2}$ where $B$ is the Beaufort number. Wind chill temperature for human-felt cold. Föhn warming from dry adiabatic descent. Superrotation index for planetary atmosphere comparison.

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
