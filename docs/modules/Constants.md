# Constants Module

The constants module provides **6 domain-organized submodules** containing **571 curated scientific constants** as compile-time `pub const f64` declarations. All values are sourced from **CODATA 2018**, **NASA/JPL**, **IUPAC**, and **IAU** references. No runtime overhead — every constant is inlined at compilation. The periodic table (118 elements) is loaded separately via compile-time YAML parsing.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This module centralizes every numerical constant used by the SciForge computation modules. Constants are grouped by the scientific domain they serve, ensuring that each computation module imports only from its corresponding constants subdirectory.

### Modeling Assumptions
- All values are in SI base units unless explicitly documented otherwise (e.g., AU in meters, not km).
- Derived constants (e.g., $\mu_0 = 1/(\varepsilon_0 c^2)$) are computed from fundamental values at compile time for self-consistency.
- Precision is limited to `f64` (≈ 15-16 significant digits).

### Reading Strategy
1. Identify the scientific domain your computation belongs to.
2. Locate the corresponding constants subdirectory below.
3. Cross-reference with `docs/code/Constants.md` for the complete list of individual constant names.


## Reading Guide

### Goal
This page explains the scientific context, origin, and relationships of the constants from a domain perspective.

### How To Read This Document
- Each section covers one constants subdirectory and its source files.
- Key relationships between constants are highlighted with equations.
- For the exhaustive list of constant names and values, see `docs/code/Constants.md`.

### Conventions
- Units: SI unless explicitly stated.
- Sources: CODATA 2018 for fundamental/atomic; NASA/JPL for astronomical; IUPAC for chemical/elemental.
- All constants are `pub const f64` — zero-cost compile-time inlining.


## Chapter 2 - Submodules and Models

### 1. Physics (58 constants)

**Source files:** `fundamental.rs`, `atomic.rs`, `elements.rs`, `units.rs`

Core physical constants, atomic-scale parameters, element data infrastructure, and unit conversion factors — the foundation of the entire crate.

**Fundamental Constants** — Speed of light $c = 2.997\,924\,58 \times 10^8\;\text{m/s}$, gravitational constant $G = 6.674\,30 \times 10^{-11}\;\text{m}^3\text{kg}^{-1}\text{s}^{-2}$, Planck constant $h$, reduced Planck $\hbar = h/2\pi$, Boltzmann $k_B$, Avogadro $N_A$, elementary charge $e$, vacuum permittivity $\varepsilon_0$, vacuum permeability $\mu_0 = 1/(\varepsilon_0 c^2)$, Coulomb constant $k_e = 1/(4\pi\varepsilon_0)$, Stefan-Boltzmann $\sigma$, gas constant $R = k_B N_A$, Faraday constant $F = N_A e$, fine-structure constant $\alpha$, strong coupling $\alpha_s$, Fermi coupling $G_F$, Wien displacement constant $b$.

**Planck Units** — Natural units at the quantum-gravity boundary: $\ell_P = \sqrt{\hbar G/c^3}$, $m_P = \sqrt{\hbar c/G}$, $t_P = \ell_P/c$, $T_P = m_P c^2/k_B$. These set the scales ($10^{-35}$ m, $10^{-8}$ kg, $10^{-44}$ s, $10^{32}$ K) where general relativity and quantum mechanics simultaneously matter.

**Atomic Constants** — Particle masses ($m_p$, $m_n$, $m_e$, u), mass-energy equivalence ($1\;\text{u} = 931.494\;\text{MeV}/c^2$), Bohr radius $a_0 = \hbar/(m_e c\alpha)$, Rydberg energy $Ry = 13.606\;\text{eV}$, Compton wavelength $\lambda_C = h/(m_e c)$, Bohr magneton $\mu_B$, nuclear magneton $\mu_N$. Hydrogen energy levels follow $E_n = -Ry/n^2$.

**Elements** — Compile-time infrastructure for the periodic table: 118 elements loaded from YAML files in `tableau-periodique/` via SciForge's built-in parser. Each element provides atomic number, symbol, standard atomic mass (IUPAC), electronegativity (Pauling), group, and period. Organized into 11 IUPAC categories (alkali metals, transition metals, lanthanides, actinides, etc.). Thread-safe lazy initialization via `OnceLock`.

**Unit Conversions** — Energy ($\text{eV} \leftrightarrow \text{J}$, $\text{cal} \to \text{J}$, $\text{erg}$, $\text{hartree}$), pressure ($\text{atm}$, $\text{bar}$, $\text{torr} \to \text{Pa}$), angular ($\text{deg} \leftrightarrow \text{rad}$), length ($\text{Å} = 10^{-10}\;\text{m}$, $\text{fm} = 10^{-15}\;\text{m}$, $\text{barn} = 10^{-28}\;\text{m}^2$), temperature shifts.

---

### 2. Astronomy (276 constants)

**Source files:** `astro.rs`, `astrophysics.rs`, `darkmatter.rs`, `galactic.rs`, `stellar.rs`

The largest constants subdirectory, covering the full range from solar-system scales to cosmological parameters.

**Solar System** — AU ($1.496 \times 10^{11}\;\text{m}$), parsec, light-year. Solar mass $M_\odot = 1.989\,1 \times 10^{30}\;\text{kg}$, radius $R_\odot$, luminosity $L_\odot$, effective temperature $T_\odot$. Earth mass $M_\oplus$, radius $R_\oplus$, gravity $g_\oplus$. Complete orbital and physical parameters for all 8 planets (Mercury–Neptune): mass, radius, flattening, orbital period, semi-major axis, eccentricity, inclination, axial tilt, sidereal day, surface gravity, escape velocity, mean density, Bond albedo, orbital velocity — sourced from NASA/JPL ephemeris.

**Stellar Physics** — Mass-luminosity exponents, spectral classification coefficients, Chandrasekhar limit ($1.4\,M_\odot$), Eddington luminosity prefactor. Wien constant $b = 2.898 \times 10^{-3}\;\text{m·K}$.

**Cosmology** — Hubble constant $H_0 = 67.4\;\text{km/s/Mpc}$, CMB temperature $T_{\text{CMB}} = 2.725\;\text{K}$, density parameters ($\Omega_m$, $\Omega_\Lambda$, $\Omega_r$, $\Omega_b$), dark energy equation-of-state $w_0$, matter-radiation equality redshift $z_{\text{eq}}$. Hubble flow velocity per Mpc.

**Galactic** — Milky Way mass $M_{\text{MW}}$, disk radius $R_d$, disk scale height $h_z$, stellar mass, star count, bulge radius. Solar galactocentric distance, orbital velocity, and period. Sgr A* mass and distance. M31 mass, distance, and radial approach velocity.

**Dark Matter** — NFW profile parameters (scale radius, central density), local dark matter density, WIMP-nucleon cross section, velocity dispersion.

**Impact Cratering** — Empirical scaling coefficients (crater diameter, fireball radius) with associated exponents from hydrocode simulations.

---

### 3. Biology (106 constants)

**Source files:** `bioenergetics.rs`, `neuroscience.rs`, `physiology.rs`, `radiobiology.rs`

Quantitative biology constants spanning cellular energetics, neuroscience, physiology, and radiation biology.

**Bioenergetics** — ATP hydrolysis free energy ($\Delta G^\circ = -30.5\;\text{kJ/mol}$), mitochondrial membrane potential, proton-motive force, electron transport chain redox potentials, photosynthetic quantum yield ($\Phi \approx 0.125$), Calvin cycle stoichiometry, metabolic scaling exponents (Kleiber's law $B \propto M^{0.75}$), enzyme kinetics reference parameters.

**Neuroscience** — Nernst equation constants, Goldman-Hodgkin-Katz voltage coefficients, resting membrane potential ($V_m \approx -70\;\text{mV}$), ion channel conductances ($g_{\text{Na}}$, $g_{\text{K}}$, $g_{\text{leak}}$), reversal potentials, action potential thresholds, synaptic time constants, neurotransmitter diffusion coefficients.

**Physiology** — Cardiac output, blood viscosity, vessel compliance parameters, respiratory volumes, gas exchange constants (O₂/CO₂ partial pressures, Henry's law coefficients for blood gases), hemoglobin binding cooperativity (Hill coefficient $n \approx 2.8$), alveolar ventilation.

**Radiobiology** — Linear-quadratic model coefficients ($\alpha$, $\beta$), tissue radiosensitivity parameters, oxygen enhancement ratio (OER), dose-rate constants for common isotopes, repair half-times, cell survival curve parameters.

---

### 4. Geology (56 constants)

**Source files:** `impact.rs`, `radioactive.rs`, `seawater.rs`

Earth-science constants for geochronology, seawater geochemistry, and impact cratering.

**Radioactive Decay** — Decay constants for all major geochronological systems: $\lambda_{^{238}\text{U}} = 1.551\,25 \times 10^{-10}\;\text{yr}^{-1}$, $\lambda_{^{235}\text{U}}$, $\lambda_{^{232}\text{Th}}$, $\lambda_{^{40}\text{K}}$ (total and branching ratios), $\lambda_{^{87}\text{Rb}}$, $\lambda_{^{147}\text{Sm}}$, $\lambda_{^{14}\text{C}}$. Heat production rates per unit mass for $^{238}$U, $^{232}$Th, $^{40}$K — essential for mantle thermal evolution models.

**Seawater** — UNESCO/TEOS-10 equation-of-state polynomial coefficients for seawater density $\rho(T,S)$: base density, thermal expansion terms ($T^1$ through $T^5$), salinity terms, cross-terms. Sound speed empirical coefficients (Medwin formula). Thermal expansion and haline contraction baseline coefficients.

**Impact** — Crater scaling law coefficients and exponents (density, projectile diameter, velocity, gravity). Fireball radius scaling from kiloton-equivalent energy.

---

### 5. Mathematics (57 constants)

**Source files:** `approximation.rs`, `ode.rs`, `quadrature.rs`

Numerical analysis constants — nodes, weights, and coefficients for integration, ODE solving, and function approximation.

**Quadrature** — Gauss-Legendre nodes and weights for various orders. Gauss-Kronrod extension nodes for adaptive integration with error estimation. Clenshaw-Curtis weights.

**ODE Solvers** — Butcher tableau coefficients for Runge-Kutta methods: RK4 (classic 4th order), Dormand-Prince 5(4) embedded pair (adaptive step control), and other multi-stage methods. The Dormand-Prince coefficients are the industry-standard choice for general-purpose adaptive ODE integration.

**Approximation** — Chebyshev polynomial nodes and weights for spectral interpolation. Padé approximant coefficients for rational function fitting. Remez algorithm reference constants.

---

### 6. Meteorology (18 constants)

**Source files:** `atmospheric.rs`

Atmospheric science constants used across all meteorology computation submodules.

**Thermodynamic** — Standard sea-level pressure $P_0 = 101\,325\;\text{Pa}$, sea-level air density $\rho_0 = 1.225\;\text{kg/m}^3$, specific gas constants for dry air $R_d = 287.05\;\text{J/(kg·K)}$ and water vapor $R_v = 461.5\;\text{J/(kg·K)}$, specific heat at constant pressure $c_p = 1004\;\text{J/(kg·K)}$, latent heat of vaporization $L_v = 2.501 \times 10^6\;\text{J/kg}$. Magnus formula coefficients ($A = 17.67$, $B = 243.5\;\text{°C}$) for saturation vapor pressure.

**Dynamic** — Earth rotation rate $\Omega = 7.292 \times 10^{-5}\;\text{rad/s}$, von Kármán constant $\kappa = 0.41$ (log-layer turbulence). Beaufort-to-m/s conversion coefficient and exponent.

**Oceanographic** — JONSWAP spectrum parameters ($\alpha$, $\gamma$, $\sigma_a$, $\sigma_b$, peak frequency coefficient). Phillips spectrum constants. Wave height and period empirical scaling coefficients.

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` precision limits all constants to ≈ 15-16 significant digits.
- Derived constants computed from other constants at compile time may accumulate floating-point rounding in the last digits.

### Recommended Verification
- Cross-reference critical values against CODATA 2018, NASA/JPL Horizons, and IUPAC databases.
- For high-precision calculations, verify that the constant precision matches your application's requirements.
- Check units carefully: all constants are in SI base units unless documented otherwise.

### Transition to Implementation
For the complete enumeration of every constant name, value, and unit, see `docs/code/Constants.md`.
