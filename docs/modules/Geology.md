# Geology Module

The **Geology** module spans **10 submodules** focused on Earth sciences: seismology, radiometric dating, petrology, and plate tectonics. It provides tools for seismic wave analysis, rock classification, age determination, and geodynamic modeling.

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

### 1. Dating

Radiometric and geochronological methods for determining the age of rocks, minerals, and organic remains. These techniques exploit the predictable decay of radioactive isotopes to anchor Earth's geological timescale.

**Radioactive Decay** — The fundamental law $N(t) = N_0 e^{-\lambda t}$ describes the exponential decrease of a parent isotope, where $\lambda = \ln 2 / t_{1/2}$ is the decay constant. The general radiometric age from a daughter-to-parent ratio is $t = \frac{1}{\lambda}\ln\!\left(1 + \frac{D}{P}\right)$.

**Carbon-14 Dating** — For organic materials up to ~50 ka, the age follows $t = -8267\,\ln(R)$ where $R$ is the measured ¹⁴C/¹²C ratio relative to the modern standard. The mean life $\tau = 8267$ years corresponds to the Libby half-life of 5730 years.

**Potassium-Argon (K-Ar)** — Used for volcanic rocks, with $t = \frac{1}{\lambda}\ln\!\left(1 + \frac{{}^{40}\text{Ar}}{f \cdot {}^{40}\text{K}}\right)$ where $\lambda = 5.543 \times 10^{-10}\,\text{yr}^{-1}$ and $f = 0.1048$ is the electron capture branching ratio.

**Uranium-Lead (U-Pb)** — The most precise chronometer for ancient rocks. Age from $t = \frac{1}{\lambda_{238}}\ln\!\left(1 + \frac{{}^{206}\text{Pb}}{{}^{238}\text{U}}\right)$ with $\lambda_{238} = 1.55125 \times 10^{-10}\,\text{yr}^{-1}$. Concordia diagrams combine both U decay chains for discordance analysis.

**Isochron Method** — Linear regression on a plot of daughter/stable vs. parent/stable isotope ratios: the slope $m$ gives $t = \frac{1}{\lambda}\ln(1+m)$, and the y-intercept gives the initial ratio, eliminating the need for assumptions about initial daughter abundance.

**Fission Track Dating** — Counts spontaneous fission tracks of ²³⁸U in minerals: $t = \frac{1}{\lambda}\ln\!\left(1 + \frac{\lambda\,\rho_s\,\rho_d}{\rho_i}\right)$ where $\rho_s$, $\rho_i$, $\rho_d$ are spontaneous, induced, and dosimeter track densities.

**Luminescence Dating** — Equivalent dose from accumulated radiation: $\text{Age} = \text{Natural Signal} / \text{Dose Rate}$. Applicable to quartz and feldspar grains in sediments (OSL/TL).

**Cosmogenic Nuclide Exposure** — Surface exposure age from in-situ production of cosmogenic isotopes (¹⁰Be, ²⁶Al): $t = -\frac{1}{\lambda}\ln\!\left(1 - \frac{C\lambda}{P}\right)$ where $C$ is concentration and $P$ is the local production rate.

---

### 2. Petrology

Rock classification, igneous geochemistry, and magmatic process modeling.

**CIPW Normative Mineralogy** — The Cross-Iddings-Pirsson-Washington norm recasts whole-rock oxide analyses (SiO₂, Al₂O₃, FeO, MgO, CaO, Na₂O, K₂O…) into hypothetical mineral assemblages. The quartz norm $Q = \text{SiO}_2 - \text{feldspars} - \text{mafics}$ determines silica saturation (positive Q → quartz-normative; negative → nepheline-normative).

**Mg Number** — An index of magmatic differentiation: $\text{Mg\#} = \frac{\text{MgO}/40.3}{\text{MgO}/40.3 + \text{FeO}/71.85} \times 100$. Primitive basalts have Mg# > 70; evolved magmas < 50.

**Total Alkali-Silica (TAS)** — Classification of volcanic rocks by plotting $(\text{Na}_2\text{O} + \text{K}_2\text{O})$ vs. SiO₂, delineating fields for basalt, andesite, dacite, rhyolite, etc.

**Alumina Saturation Index** — $\text{ASI} = \frac{\text{Al}_2\text{O}_3/102}{\text{CaO}/56 + \text{Na}_2\text{O}/62 + \text{K}_2\text{O}/94}$ in molar proportions. ASI < 1 → metaluminous; ASI = 1 → peraluminous boundary; ASI > 1 → peraluminous (cordierite/garnet-bearing granites).

**Differentiation Index** — $DI = Q + Or + Ab + Ne$ from the CIPW norm, quantifying degree of magmatic evolution (high DI → granitic; low DI → gabbroic).

**Liquidus & Solidus** — Liquidus temperature interpolation from a binary phase diagram. Solidus depression by water: $T_{\text{solidus}} = T_0 - k \cdot w^{0.6}$ where $w$ is water content (wt.%).

**Crystal Settling** — Stokes' velocity for crystals sinking through magma: $v = \frac{2\Delta\rho\,g\,r^2}{9\mu}$. Magma viscosity follows an Arrhenius relation $\mu = A\exp(E_a/RT)$, strongly dependent on SiO₂ content and temperature.

---

### 3. Seismology

Earthquake characterization and seismic wave propagation through Earth's interior.

**Body Waves** — P-wave velocity $V_P = \sqrt{(K + \frac{4}{3}G)/\rho}$ and S-wave velocity $V_S = \sqrt{G/\rho}$, where $K$ is the bulk modulus, $G$ the shear modulus, and $\rho$ the density. S-waves cannot propagate in liquids ($G = 0$), which constrains the outer core to be fluid.

**Magnitude Scales** — The local (Richter) magnitude $M_L = \log_{10} A + 3\log_{10} d - 2.92$ relates to the amplitude $A$ (µm) and epicentral distance $d$ (km). The moment magnitude $M_w = (\log_{10} M_0 - 9.1)/1.5$ is based on the seismic moment $M_0 = \mu A D$ (rigidity × fault area × slip). Seismic energy $E = 10^{1.5M+4.8}$ (Gutenberg-Richter energy-magnitude relation).

**Epicenter Location** — Triangulation from $S - P$ arrival time: $\Delta = \frac{(t_S - t_P) \cdot V_P \cdot V_S}{V_P - V_S}$ requires three or more station readings.

**Snell's Law (Seismic)** — Refraction at layer boundaries: $\frac{\sin\theta_1}{V_1} = \frac{\sin\theta_2}{V_2}$, critical for ray-tracing through Earth's velocity structure and detecting discontinuities (Mohorovičić, Gutenberg).

**Gutenberg-Richter Law** — Earthquake frequency-magnitude distribution: $\log_{10} N = a - bM$ where $N$ is the annual number of earthquakes ≥ magnitude $M$, $a$ measures total seismicity and $b \approx 1$ globally.

**Omori's Law** — Aftershock decay rate: $n(t) = K/(t+c)^p$ where $t$ is time after the mainshock, $c$ avoids the singularity at $t = 0$, and $p \approx 1$ empirically.

**Peak Ground Acceleration** — Ground motion prediction equation $\text{PGA} = a \cdot e^{bM}/d$ relating magnitude and hypocentral distance to hazard, used for seismic building codes.

---

### 4. Tectonics

Plate motion, lithospheric mechanics, and thermal evolution of sedimentary basins.

**Plate Velocity** — Average velocity $v = d/t$ from hotspot tracks or magnetic anomaly offsets. Velocity at any point from an Euler pole: $v = \omega R \sin\theta$ where $\omega$ is the angular velocity, $R$ the Earth's radius, and $\theta$ the angular distance (colatitude) from the rotation pole.

**Isostasy** — Airy model: a topographic elevation $h$ is compensated by a crustal root of depth $r = h\rho_c/(\rho_m - \rho_c)$. Pratt model: density varies laterally so that $\rho(D + h) = \rho_{\text{ref}} D_{\text{ref}}$ for constant compensation depth. General isostatic equilibrium: free-board $= t_c(1 - \rho_c/\rho_m)$.

**McKenzie Stretching Model** — Initial subsidence of a rift basin under uniform lithospheric thinning (stretching factor $\beta$):

$$S_i = t_c \frac{\rho_m \alpha T_l}{2(\rho_m - \rho_c)}\left(1 - \frac{1}{\beta}\right)$$

where $\alpha$ is the thermal expansion coefficient and $T_l$ the basal lithosphere temperature. Subsequent thermal subsidence follows $e(t) = e_0(1 - e^{-t/\tau})$ with a thermal time constant $\tau \approx 60$ Myr.

**Heat Flow** — Fourier's law $q = -k\,dT/dz$ links surface heat flow to thermal conductivity and the geothermal gradient. Temperature at depth: $T(z) = T_s + \nabla T \cdot z$.

**Flexural Rigidity** — The bending resistance of the lithosphere: $D = \frac{E\,T_e^3}{12(1-\nu^2)}$ where $E$ is Young's modulus, $T_e$ the effective elastic thickness, and $\nu$ Poisson's ratio. Inverse calculation of $T_e$ from observed flexure constrains lithospheric strength.

---

### 5. Erosion

Surface weathering and denudation processes — the destructive counterpart to tectonic uplift.

**Fluvial Erosion** — Stream-power erosion rate: $\dot{e} = K\,P^{3/2}\tan\alpha\,(1 - v_c)$ where $K$ is an erodibility coefficient, $P$ the precipitation rate, $\alpha$ the slope angle, and $v_c$ the vegetation cover fraction (0 = bare, 1 = full cover). This captures the interaction between climate, topography, and land cover.

**Chemical Weathering** — Arrhenius-modulated dissolution: $R = A\exp(-E_a/R^*T)\,P^{0.65}$ where $A$ is a pre-exponential factor, $E_a$ the activation energy, $R^*$ the gas constant, $T$ the temperature, and $P$ the precipitation. The $P^{0.65}$ exponent reflects the kinetic role of water throughput in mineral dissolution.

**Frost Weathering** — Mechanical fragmentation by freeze-thaw cycles: $R = 0.001\,n_{\text{ft}}\,\phi$ where $n_{\text{ft}}$ is the annual number of freeze-thaw cycles and $\phi$ the porosity. Dominant in periglacial environments.

**Wind Erosion** — The Bagnold threshold friction velocity: $u_* = 0.1\sqrt{(\rho_p - \rho_a)gd / \rho_a}$ where $d$ is the grain diameter. Below this threshold, wind cannot entrain particles. Fine silt and clay require higher velocities than sand due to cohesion.

**Volcanic Explosivity** — Qualitative VEI classification from ejecta volume, discretized on a logarithmic scale: VEI 0 ($<10^{-5}\;\text{km}^3$) through VEI 8 ($>100\;\text{km}^3$).

---

### 6. Geomorphology

Large-scale landform evolution — from planetary hypsometry to tectonic and volcanic topography.

**Hypsometric Curve** — Statistical elevation distribution: $z(f) = \bar{z} + \sigma\,(2/\pi)\arcsin(2f - 1)$ where $f$ is the cumulative area fraction, $\bar{z}$ the mean elevation, and $\sigma$ the standard deviation. The bimodal Earth hypsometry (continents at $\sim 0.84\;\text{km}$, ocean floor at $\sim -3.7\;\text{km}$) reflects isostatic equilibrium of two crust types.

**Ocean Basin Depth** — Lithospheric cooling model: $d(t) = d_r + 2\alpha T_l(\rho_m - \rho_w)/\rho_m \sqrt{\kappa t/\pi}$ where $d_r$ is ridge crest depth, $t$ is plate age (Myr), and $\kappa$ the thermal diffusivity. The $\sqrt{t}$ dependence (Parsons-Sclater model) fits oceanic bathymetry well to $\sim 80\;\text{Ma}$.

**Orogenic Elevation** — Net surface uplift from the competition between convergence and erosion: $h(t) = (v_{\text{conv}} - v_{\text{eros}})(1 - \rho_c/\rho_m)\,t$, incorporating isostatic compensation. Mid-ocean ridge elevation scales with spreading rate $\propto \sqrt{v_s}$.

**Stream Power Erosion** — The Hack/detachment-limited model: $\dot{e} = K A^m S^n$ where $A$ is drainage area (proxy for discharge), $S$ is channel slope, and $m$, $n$ are empirical exponents ($m \approx 0.5$, $n \approx 1$). This governs channel incision and knickpoint retreat.

**Sediment Transport** — Shields parameter $\tau^* = \tau / [(\rho_s - \rho_w)gd]$ determines whether flow entrains bed material. Above the critical Shields number ($\tau^*_c \approx 0.047$), the Meyer-Peter and Müller formula gives bedload transport: $q_b \propto (\tau^* - \tau^*_c)^{3/2}$.

**Impact Craters** — Energy-gravity scaling for crater diameter, with a simple-to-complex transition: simple craters ($D < D_t$) have $\text{depth} = D/5$; complex craters ($D > D_t$) follow $\text{depth} \propto D_t^{0.3} D^{0.7}$ due to central rebound.

**Volcanic Edifice Profiles** — Shield volcano (parabolic): $h(r) = H(1 - r^2/R^2)$. Stratovolcano (concave): $h(r) = H(1 - r/R)^2$. Caldera (piecewise): subsidence bowl + raised rim with Gaussian decay.

**Rift Valleys and Faults** — Rift half-width from flexural rigidity: $w = \pi(4D/\rho_m g)^{1/4}$. Tectonic stress $\sigma = \eta v / h$ (viscous plate model). Fault slip rate from Coulomb failure criterion.

**Weathering and Soil** — Thermally activated chemical weathering (same Arrhenius form as erosion module). Soil production function: $P = P_0 \exp(-d/d^*)$ with exponential decline under thickening soil. Landscape diffusion $\partial h/\partial t = D \nabla^2 h$ (creep-dominated hillslopes). Flexural wavelength from elastic plate bending.

---

### 7. Glaciology

Ice dynamics — rheology, flow, and erosion by glaciers and ice sheets.

**Glen's Flow Law** — The constitutive relation for polycrystalline ice: $\dot{\varepsilon} = A\,\tau^n$ where $\dot{\varepsilon}$ is the strain rate, $\tau$ the deviatoric stress, $A$ the creep parameter (strongly temperature-dependent via Arrhenius), and $n \approx 3$ the Glen exponent. This nonlinear viscous law is the foundation of all ice-sheet models.

**Shallow Ice Approximation** — The vertically integrated surface velocity for a grounded ice sheet on a sloped bed:

$$u_s = \frac{2A}{n+1}(\rho g \sin\alpha)^n H^{n+1}$$

where $\alpha$ is the surface slope and $H$ the ice thickness. This assumes that longitudinal stress gradients are negligible compared to basal shear — valid for large ice sheets far from margins.

**Ice Viscosity** — Effective viscosity $\eta = 1/(2A\tau^{n-1})$, diverging at zero stress (rigid behavior at rest) and decreasing under load (shear-thinning). Temperature dependence causes basal ice to flow orders of magnitude faster than cold surface ice.

**Glacial Erosion** — Quarrying and abrasion rate: $\dot{e} = K_g v_b^l$ where $v_b$ is the basal sliding velocity and $l \approx 1$ (linear erosion law) to $l \approx 2$ (quadratic). This produces U-shaped valleys, fjords, and cirques.

---

### 8. Hydrology

Open-channel flow, dimensionless regime indicators, and stream power for fluvial geomorphology.

**Manning Equation** — Mean flow velocity in an open channel: $v = (1/n)\,R_h^{2/3}\,S^{1/2}$ where $n$ is Manning's roughness coefficient, $R_h$ the hydraulic radius, and $S$ the energy slope. Typical $n$ ranges from 0.012 (smooth concrete) to 0.06 (mountain streams with boulders).

**Chézy Equation** — An alternative open-channel formula: $v = C\sqrt{R_h S}$ where $C$ is the Chézy coefficient (related to Manning's $n$ by $C = R_h^{1/6}/n$).

**Froude Number** — $Fr = v/\sqrt{gd}$ characterizes the flow regime: $Fr < 1$ is subcritical (tranquil, deep), $Fr = 1$ is critical, and $Fr > 1$ is supercritical (shooting, shallow). Hydraulic jumps occur at the transition from supercritical to subcritical.

**Reynolds Number** — $Re = vd/\nu$ separates laminar ($Re < 2000$) from turbulent ($Re > 4000$) flow in channels. Virtually all natural river flows are fully turbulent.

**Stream Power** — Total stream power per unit length: $\Omega = \rho g Q S$ where $Q$ is the discharge. This is the rate of potential energy expenditure available for sediment transport and channel incision. Specific stream power $\omega = \Omega / w$ (per unit channel width) is the key parameter for predicting channel morphology.

**Hjulström Curve** — Empirical erosion threshold velocity as a function of grain size: sand ($d \sim 0.5\;\text{mm}$) erodes at the lowest velocity ($\sim 0.2\;\text{m/s}$); both finer (clay, due to cohesion) and coarser (gravel, due to weight) particles require higher velocities. The piecewise function captures this characteristic minimum.

---

### 9. Mantle Dynamics

Thermal convection, core evolution, and geodynamo in planetary interiors.

**Rayleigh Number** — The master dimensionless number for convection: $Ra = \alpha g \Delta T D^3/(\kappa\nu)$ where $\alpha$ is the thermal expansion coefficient, $\Delta T$ the temperature contrast, $D$ the mantle depth, $\kappa$ the thermal diffusivity, and $\nu$ the kinematic viscosity. Earth's mantle $Ra \sim 10^7$ — vigorous convection far above the critical value ($Ra_c \approx 10^3$).

**Nusselt Number** — Heat transfer enhancement by convection: $Nu = 0.195\,Ra^\beta$ where $\beta \approx 1/3$ for high-$Ra$ convection. $Nu = 1$ means purely conductive; Earth's mantle $Nu \sim 20$.

**Convective Velocity** — $v \sim (\kappa/D)\,Ra^{2/3}$, typically a few cm/yr for the mantle — matching observed plate velocities. The mantle overturn time $\tau = D/v \sim 100\;\text{Myr}$ is the characteristic renewal timescale.

**Core Thermal Structure** — Adiabatic temperature profile: $T(P) = T_0(1 + \gamma P/K)$ where $\gamma$ is the Grüneisen parameter and $K$ the bulk modulus. Inner-core radius determined by the intersection of the adiabat with the iron solidus. Core heat flux $q = k\Delta T/R_c$ and CMB heat flux $q_{\text{CMB}} = Nu\,k\Delta T/D$.

**Viscosity** — Arrhenius temperature dependence: $\eta = \eta_0 \exp[(E_a/k_B)(1/T - 1/T_0)]$. Orders-of-magnitude viscosity variation ($10^{19}$–$10^{22}\;\text{Pa\cdot s}$) across the mantle drives stagnant-lid vs. mobile-lid convection regimes.

**Thermal Boundary Layer** — Thickness $\delta \sim D\,Ra^{-1/3}$ at both the surface (lithosphere) and CMB (D" layer). Mantle plumes arise from thermal instabilities at the CMB boundary layer.

**Plumes** — Buoyancy flux $B = \alpha\rho g \Delta T\,Q_v$ and conduit radius $r \propto (B\nu/\kappa)^{1/4}$ characterize hot spots. The buoyancy flux of the Hawaiian plume is $\sim 10^4\;\text{kg/s}$.

**Core Cooling and Geodynamo** — Core cooling rate $dT/dt = 3q/(R_c\rho_c c_p)$. Magnetic Reynolds number $Rm = vL/\eta_m$ must exceed $\sim 40$ for dynamo action. The dipole moment scales with convective power as $\mathcal{M} \propto (\rho\Omega P_{\text{conv}} / \mu_0\sigma)^{1/2}$.

**Energy Budget** — Radiogenic heating from $^{238}$U, $^{232}$Th, $^{40}$K with time-dependent exponential decay. Secular cooling flux from core crystallization. Gravitational differentiation power from inner-core growth. Parameterized thermal evolution: $T_{n+1} = T_n + (H_{\text{rad}} - Q_{\text{surf}})\Delta t / (c_p M)$.

**Stagnant Lid** — Lid thickness $\delta_{\text{lid}} \approx D/\Theta$ where $\Theta = E_a\Delta T/(k_B T_i^2)$ is the Frank-Kamenetskii parameter. Stagnant-lid convection characterizes Venus and Mars; mobile-lid (plate tectonics) requires additional weakening mechanisms. Surface heat flux and core liquidus complete the planetary interior toolkit.

---

### 10. Volcanism

Magma transport, eruption dynamics, and volcanic hazard quantification.

**Magma Viscosity** — Arrhenius relation: $\mu = A\exp(E_a/R^*T)$ with viscosity spanning $10^1\;\text{Pa\cdot s}$ (basalt at $1200\;\text{°C}$) to $10^{12}\;\text{Pa\cdot s}$ (rhyolite at $800\;\text{°C}$). SiO₂ content is the dominant compositional control.

**Magma Buoyancy** — Net upward force on a chamber: $F = (\rho_{\text{crust}} - \rho_{\text{magma}})\,g\,V$ drives ascent when the magma is lighter than the surrounding rock.

**Eruption Triggering** — Overpressure threshold: eruption occurs when chamber pressure exceeds $P_{\text{lith}} + \sigma_T$ (lithostatic pressure plus tensile strength). Chamber overpressure $\Delta P = K \Delta V / V$ links magma injection volume to pressure buildup via the bulk modulus.

**Conduit Flow** — Poiseuille flow for effusion rate: $Q = \pi r^4 \Delta P / (8\mu L)$ — controlled by conduit radius to the fourth power, explaining why small dike widening dramatically increases lava output.

**Lava Flow** — Gravity-driven viscous flow on a slope: $v = \rho g \sin\alpha\,h^2/(3\mu)$, identical to the Jeffreys equation for thin-film flow. Shield volcanoes produce fast, thin basaltic flows; stratovolcanoes produce slow, thick andesitic flows.

**Eruption Column** — Plinian column height $H \propto \dot{Q}_T^{0.25}$ from the thermal flux (Morton-Taylor-Turner buoyant plume theory). Tephra fallout distance $x = H\,U_w/v_t$ where $U_w$ is the wind speed and $v_t$ the particle terminal velocity.

**VEI** — Volcanic Explosivity Index from ejecta volume: $\text{VEI} = \log_{10}(V_{\text{km}^3}) + 4$, clamped to $[0, 8]$. Eruptive energy $E = \frac{1}{2}m v^2 + E_{\text{thermal}}$ combines kinetic and thermal contributions.

**Eruption Forecasting** — Seismic-to-eruption probability via cumulative moment ratio: $P = 1 - \exp(-b\,M_0/M_c)$. Tidal triggering stress $\sigma_t = 2GMR/d^3$ from solid Earth tides. Coulomb failure stress $\text{CFS} = \tau - \mu_f(\sigma_n - P_p) - c$ for fault reactivation near volcanoes.

**Magma Ascent** — Stokes-like ascent velocity $v = \Delta\rho\,g\,r^2/(8\mu)$ in a cylindrical conduit. Volatile exsolution depth $z = P_{\text{sat}}/(\rho g)$ marks the onset of vesiculation. Fragmentation occurs when porosity exceeds a critical vesicularity. Degassing rate and thermal erosion (lava-substrate interaction) complete the volcanic process chain.

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
