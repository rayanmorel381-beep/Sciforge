# Constants — ChangeLog

---

## v0.0.4

### New — Particle Physics Constants — `constants::physics` — 6 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Fermi coupling constant | `pub const G_F: f64` | $G_F / (\hbar c)^3 \approx 1.1664 \times 10^{-5}\,\text{GeV}^{-2}$ (SI: J·m³) | `constants::physics::fundamental` |
| Strong coupling constant | `pub const ALPHA_S: f64` | $\alpha_s(M_Z) \approx 0.1179$ | `constants::physics::fundamental` |
| Muon mass | `pub const MUON_MASS: f64` | $m_\mu \approx 1.8836 \times 10^{-28}\,\text{kg}$ | `constants::physics::atomic` |
| Tau mass | `pub const TAU_MASS: f64` | $m_\tau \approx 3.1675 \times 10^{-27}\,\text{kg}$ | `constants::physics::atomic` |
| Neutrino mass upper bound | `pub const NEUTRINO_MASS_UPPER: f64` | $m_{\nu,\text{max}} \approx 1.1 \times 10^{-36}\,\text{kg}$ | `constants::physics::atomic` |
| Rydberg constant | `pub const R_INF: f64` | $R_\infty \approx 1.097_373 \times 10^7\,\text{m}^{-1}$ | `constants::physics::atomic` |

### New — Planetary Constants — `constants::astronomy` — Mercury through Neptune

Full per-planet constant sets added for all 7 planets (Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune), each providing mass, radius, mean density, surface gravity, orbital velocity, and geometric albedo.

---

## v0.0.3

### Restructuring

5 flat files → 6 domain submodules (20 source files, ~527 pub const)

| Metric | Value |
|---|---|
| Flat files removed | 5 (astro.rs, atomic.rs, fundamental.rs, units.rs, elements.rs) |
| Domain submodules created | 6 (astronomy, biology, geology, maths, meteorology, physics) |
| Total source files | 20 |
| Total pub const | ~527 |

No new functions — see `testing.md` for test details.

### 1️⃣ New — Astronomy Constants — `constants::astronomy` — 5 files, 287 pub const

#### `astronomy::astro` — 207 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Astronomical unit | `pub const AU: f64 = 1.495_978_707e11` | $1\,\text{AU}$ in metres | `astronomy::astro` |
| Parsec | `pub const PARSEC: f64 = 3.085_677_581e16` | 1 pc in metres | `astronomy::astro` |
| Light-year | `pub const LIGHT_YEAR: f64 = 9.460_730_472_580_8e15` | 1 ly in metres | `astronomy::astro` |
| Solar mass | `pub const SOLAR_MASS: f64 = 1.989_1e30` | $M_\odot$ (kg) | `astronomy::astro` |
| Solar radius | `pub const SOLAR_RADIUS: f64 = 6.957e8` | $R_\odot$ (m) | `astronomy::astro` |
| Solar luminosity | `pub const SOLAR_LUMINOSITY: f64 = 3.828e26` | $L_\odot$ (W) | `astronomy::astro` |
| Solar density | `pub const SOLAR_DENSITY: f64` | Derived $\rho_\odot$ | `astronomy::astro` |
| Solar gravity | `pub const SOLAR_GRAVITY: f64` | Derived $g_\odot$ | `astronomy::astro` |
| Earth mass | `pub const EARTH_MASS: f64 = 5.972_37e24` | $M_\oplus$ (kg) | `astronomy::astro` |
| Earth radius | `pub const EARTH_RADIUS: f64 = 6.372_0e6` | $R_\oplus$ (m) | `astronomy::astro` |
| Sun surface temperature | `pub const SUN_SURFACE_TEMPERATURE: f64 = 5778.0` | $T_{\text{eff}}$ (K) | `astronomy::astro` |
| Sun core temperature | `pub const SUN_CORE_TEMPERATURE: f64 = 1.57e7` | Core $T$ (K) | `astronomy::astro` |
| Sun core density | `pub const SUN_CORE_DENSITY: f64 = 1.622e5` | Core $\rho$ (kg/m³) | `astronomy::astro` |
| Sun age | `pub const SUN_AGE: f64 = 1.44e17` | Age (s) | `astronomy::astro` |
| Sun rotation period | `pub const SUN_ROTATION_PERIOD: f64 = 2.164e6` | Sidereal rotation (s) | `astronomy::astro` |
| Hubble constant | `pub const HUBBLE_CONSTANT: f64 = 67.4` | $H_0$ (km/s/Mpc) | `astronomy::astro` |
| CMB temperature | `pub const CMB_TEMPERATURE: f64 = 2.725` | $T_{\text{CMB}}$ (K) | `astronomy::astro` |
| Cosmological constant | `pub const COSMOLOGICAL_LAMBDA: f64 = 1.105_6e-52` | $\Lambda$ (m⁻²) | `astronomy::astro` |

+189 planetary/satellite constants: Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Moon — mass, radius, density, gravity, orbital velocity, J2; Yoshida integrator coefficients.

#### `astronomy::astrophysics` — 22 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Mass-luminosity low threshold | `pub const ML_LOW_THRESHOLD: f64 = 0.43` | $M < 0.43 M_\odot$ boundary | `astronomy::astrophysics` |
| ML low coefficient | `pub const ML_LOW_COEFF: f64 = 0.23` | Low-mass $L \propto 0.23 M^{2.3}$ | `astronomy::astrophysics` |
| ML low exponent | `pub const ML_LOW_EXPONENT: f64 = 2.3` | Low-mass exponent | `astronomy::astrophysics` |
| ML mid threshold | `pub const ML_MID_THRESHOLD: f64 = 2.0` | Mid-mass boundary | `astronomy::astrophysics` |
| ML mid exponent | `pub const ML_MID_EXPONENT: f64 = 4.0` | $L \propto M^4$ | `astronomy::astrophysics` |
| ML high threshold | `pub const ML_HIGH_THRESHOLD: f64 = 55.0` | High-mass boundary | `astronomy::astrophysics` |
| ML high coefficient | `pub const ML_HIGH_COEFF: f64 = 1.4` | High-mass coefficient | `astronomy::astrophysics` |
| ML high exponent | `pub const ML_HIGH_EXPONENT: f64 = 3.5` | High-mass exponent | `astronomy::astrophysics` |
| ML very high coefficient | `pub const ML_VERY_HIGH_COEFF: f64 = 32000.0` | Very high mass coefficient | `astronomy::astrophysics` |
| Wien displacement | `pub const WIEN_DISPLACEMENT: f64 = 2.897_771_955e-3` | $\lambda_{\max} T = b$ (m·K) | `astronomy::astrophysics` |
| Main-sequence lifetime scale | `pub const MAIN_SEQUENCE_LIFETIME_SCALE_YR: f64 = 1e10` | $\tau_{\text{MS}}$ reference (yr) | `astronomy::astrophysics` |
| Chandrasekhar limit (solar) | `pub const CHANDRASEKHAR_LIMIT_SOLAR: f64 = 1.4` | $1.4\,M_\odot$ | `astronomy::astrophysics` |
| Spectral temp scale | `pub const SPECTRAL_TEMP_SCALE: f64 = 42000.0` | Spectral classification scale | `astronomy::astrophysics` |
| Spectral index offset | `pub const SPECTRAL_INDEX_OFFSET: f64 = 0.92` | Spectral index offset | `astronomy::astrophysics` |
| Bolometric correction quad | `pub const BOLOMETRIC_CORRECTION_QUAD: f64 = -8.499` | BC quadratic term | `astronomy::astrophysics` |
| Bolometric correction center | `pub const BOLOMETRIC_CORRECTION_CENTER: f64 = 4.012` | BC center $\log T$ | `astronomy::astrophysics` |
| Bolometric correction offset | `pub const BOLOMETRIC_CORRECTION_OFFSET: f64 = -0.09` | BC offset | `astronomy::astrophysics` |
| Habitable zone inner flux | `pub const HABITABLE_ZONE_INNER_FLUX: f64 = 1.1` | Inner HZ boundary ($S/S_\odot$) | `astronomy::astrophysics` |
| Habitable zone outer flux | `pub const HABITABLE_ZONE_OUTER_FLUX: f64 = 0.53` | Outer HZ boundary ($S/S_\odot$) | `astronomy::astrophysics` |
| Nutation amplitude | `pub const NUTATION_AMPLITUDE_ARCSEC: f64 = 9.2` | Nutation amplitude (arcsec) | `astronomy::astrophysics` |
| Vernal equinox DOY | `pub const VERNAL_EQUINOX_DOY: f64 = 81.0` | Day of year | `astronomy::astrophysics` |
| Tidal dissipation coefficient | `pub const TIDAL_DISSIPATION_COEFF: f64 = 10.5` | Tidal $Q$ scaling | `astronomy::astrophysics` |

#### `astronomy::darkmatter` — 21 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Local DM density | `pub const LOCAL_DM_DENSITY: f64 = 0.3` | $\rho_{\text{DM}}$ (GeV/cm³) | `astronomy::darkmatter` |
| Local DM density SI | `pub const LOCAL_DM_DENSITY_SI: f64 = 5.35e-22` | $\rho_{\text{DM}}$ (kg/m³) | `astronomy::darkmatter` |
| $\Omega_{\text{DM}}$ | `pub const OMEGA_DM: f64 = 0.265` | Dark matter density parameter | `astronomy::darkmatter` |
| $\Omega_b$ | `pub const OMEGA_BARYON: f64 = 0.049` | Baryon density parameter | `astronomy::darkmatter` |
| $\Omega_m$ | `pub const OMEGA_MATTER: f64` | Derived $\Omega_{\text{DM}} + \Omega_b$ | `astronomy::darkmatter` |
| $\Omega_\Lambda$ | `pub const OMEGA_LAMBDA: f64` | Derived $1 - \Omega_m$ | `astronomy::darkmatter` |
| Thermal relic $\langle\sigma v\rangle$ | `pub const THERMAL_RELIC_SIGMA_V: f64 = 3.0e-26` | cm³/s | `astronomy::darkmatter` |
| WIMP mass min | `pub const WIMP_MASS_MIN_GEV: f64 = 10.0` | Lower bound (GeV) | `astronomy::darkmatter` |
| WIMP mass max | `pub const WIMP_MASS_MAX_GEV: f64 = 1000.0` | Upper bound (GeV) | `astronomy::darkmatter` |
| WIMP $\sigma$ upper | `pub const WIMP_SIGMA_UPPER: f64 = 1.0e-46` | Cross-section upper bound (cm²) | `astronomy::darkmatter` |
| Axion mass min | `pub const AXION_MASS_MIN_EV: f64 = 1.0e-6` | Lower bound (eV) | `astronomy::darkmatter` |
| Axion mass max | `pub const AXION_MASS_MAX_EV: f64 = 1.0e-3` | Upper bound (eV) | `astronomy::darkmatter` |
| Axion decay constant | `pub const AXION_DECAY_CONSTANT: f64 = 1.0e12` | $f_a$ (GeV) | `astronomy::darkmatter` |
| NFW $\rho_s$ typical | `pub const NFW_RHO_S_TYPICAL: f64 = 4.88e6` | NFW scale density ($M_\odot$/kpc³) | `astronomy::darkmatter` |
| NFW $r_s$ MW | `pub const NFW_RS_MW: f64 = 21.5` | MW scale radius (kpc) | `astronomy::darkmatter` |
| MW virial mass | `pub const MW_VIRIAL_MASS: f64 = 1.3e12` | $M_{\text{vir}}$ ($M_\odot$) | `astronomy::darkmatter` |
| MW virial radius | `pub const MW_VIRIAL_RADIUS: f64 = 287.0` | $R_{\text{vir}}$ (kpc) | `astronomy::darkmatter` |
| MW concentration | `pub const MW_CONCENTRATION: f64 = 13.3` | $c = R_{\text{vir}} / r_s$ | `astronomy::darkmatter` |
| $z_{\text{eq}}$ | `pub const Z_MATTER_RADIATION_EQ: f64 = 3402.0` | Matter-radiation equality | `astronomy::darkmatter` |
| $z_{\text{rec}}$ | `pub const Z_RECOMBINATION: f64 = 1089.0` | Recombination redshift | `astronomy::darkmatter` |
| Age of universe | `pub const AGE_UNIVERSE: f64 = 4.35e17` | $t_0$ (s) | `astronomy::darkmatter` |

#### `astronomy::galactic` — 15 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| MW mass | `pub const MW_MASS: f64 = 1.5e12` | Milky Way total mass ($M_\odot$) | `astronomy::galactic` |
| MW disk radius | `pub const MW_DISK_RADIUS: f64 = 2.63e20` | Disk radius (m) | `astronomy::galactic` |
| MW disk scale height | `pub const MW_DISK_SCALE_HEIGHT: f64 = 9.26e18` | Scale height $h_z$ (m) | `astronomy::galactic` |
| MW bulge radius | `pub const MW_BULGE_RADIUS: f64 = 6.17e19` | Bulge radius (m) | `astronomy::galactic` |
| Sun galactic distance | `pub const SUN_GALACTIC_DISTANCE: f64 = 2.47e20` | $R_\odot$ to GC (m) | `astronomy::galactic` |
| Sun galactic velocity | `pub const SUN_GALACTIC_VELOCITY: f64 = 220_000.0` | Orbital velocity (m/s) | `astronomy::galactic` |
| Sun galactic period | `pub const SUN_GALACTIC_PERIOD: f64 = 7.25e15` | Orbital period (s) | `astronomy::galactic` |
| MW stellar mass | `pub const MW_STELLAR_MASS: f64 = 5.0e10` | $M_\star$ ($M_\odot$) | `astronomy::galactic` |
| MW star count | `pub const MW_STAR_COUNT: f64 = 2.0e11` | Estimated stars | `astronomy::galactic` |
| Sgr A* mass | `pub const SGR_A_STAR_MASS: f64 = 4.15e6` | $M_{\text{Sgr A*}}$ ($M_\odot$) | `astronomy::galactic` |
| Sgr A* distance | `pub const SGR_A_STAR_DISTANCE: f64 = 2.47e20` | Distance (m) | `astronomy::galactic` |
| M31 mass | `pub const M31_MASS: f64 = 1.5e12` | Andromeda mass ($M_\odot$) | `astronomy::galactic` |
| M31 distance | `pub const M31_DISTANCE: f64 = 2.37e22` | Distance (m) | `astronomy::galactic` |
| MW-M31 radial velocity | `pub const MW_M31_RADIAL_VELOCITY: f64 = 110_000.0` | Approach velocity (m/s) | `astronomy::galactic` |
| Hubble flow per Mpc | `pub const HUBBLE_FLOW_VELOCITY_PER_MPC: f64` | Derived from $H_0$ | `astronomy::galactic` |

#### `astronomy::stellar` — 22 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Chandrasekhar limit | `pub const CHANDRASEKHAR_LIMIT: f64` | $\approx 1.4\,M_\odot$ (kg) | `astronomy::stellar` |
| TOV limit | `pub const TOV_LIMIT: f64 = 4.58e30` | Neutron star mass limit (kg) | `astronomy::stellar` |
| Eddington prefactor | `pub const EDDINGTON_PREFACTOR: f64 = 3.2e4` | $L_{\text{Edd}} / M$ | `astronomy::stellar` |
| Solar $T_{\text{eff}}$ | `pub const SOLAR_TEFF: f64` | Derived from constants | `astronomy::stellar` |
| Solar absolute magnitude | `pub const SOLAR_ABS_MAGNITUDE: f64 = 4.83` | $M_V$ | `astronomy::stellar` |
| Solar metallicity | `pub const SOLAR_METALLICITY: f64 = 0.013_4` | $Z_\odot$ | `astronomy::stellar` |
| pp-chain energy | `pub const PP_CHAIN_ENERGY: f64 = 4.20e-12` | Energy per reaction (J) | `astronomy::stellar` |
| CNO cycle energy | `pub const CNO_CYCLE_ENERGY: f64 = 3.98e-12` | Energy per cycle (J) | `astronomy::stellar` |
| H-burning efficiency | `pub const H_BURNING_EFFICIENCY: f64 = 0.007` | $\epsilon = 0.007$ | `astronomy::stellar` |
| Solar MS lifetime | `pub const SOLAR_MS_LIFETIME: f64 = 3.15e17` | $\tau_{\text{MS},\odot}$ (s) | `astronomy::stellar` |
| WD radius typical | `pub const WD_RADIUS_TYPICAL: f64 = 5.5e6` | White dwarf radius (m) | `astronomy::stellar` |
| NS radius typical | `pub const NS_RADIUS_TYPICAL: f64 = 1.1e4` | Neutron star radius (m) | `astronomy::stellar` |
| NS mass typical | `pub const NS_MASS_TYPICAL: f64 = 2.78e30` | Neutron star mass (kg) | `astronomy::stellar` |
| Pulsar B typical | `pub const PULSAR_B_TYPICAL: f64 = 1.0e8` | Pulsar magnetic field (T) | `astronomy::stellar` |
| Magnetar B typical | `pub const MAGNETAR_B_TYPICAL: f64 = 1.0e11` | Magnetar field (T) | `astronomy::stellar` |
| Hydrogen fraction | `pub const HYDROGEN_FRACTION_SOLAR: f64 = 0.734_6` | $X_\odot$ | `astronomy::stellar` |
| Helium fraction | `pub const HELIUM_FRACTION_SOLAR: f64 = 0.248_5` | $Y_\odot$ | `astronomy::stellar` |
| Metal fraction | `pub const METAL_FRACTION_SOLAR: f64` | $Z_\odot = 1 - X - Y$ | `astronomy::stellar` |
| He-4 mass | `pub const HELIUM4_MASS: f64 = 6.644_657_3e-27` | $^4$He mass (kg) | `astronomy::stellar` |
| Adiabatic index (ideal) | `pub const ADIABATIC_INDEX_IDEAL: f64 = 5.0 / 3.0` | $\gamma = 5/3$ | `astronomy::stellar` |
| Adiabatic index (radiation) | `pub const ADIABATIC_INDEX_RADIATION: f64 = 4.0 / 3.0` | $\gamma = 4/3$ | `astronomy::stellar` |
| Opacity electron scattering | `pub const OPACITY_ELECTRON_SCATTERING: f64 = 0.2` | $\kappa_{\text{es}}$ (cm²/g) | `astronomy::stellar` |

### 2️⃣ New — Biology Constants — `constants::biology` — 4 files, 102 pub const

#### `biology::bioenergetics` — 12 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Kleiber constant | `pub const KLEIBER_CONSTANT: f64 = 70.0` | Metabolic scaling coefficient | `biology::bioenergetics` |
| Kleiber exponent | `pub const KLEIBER_EXPONENT: f64 = 0.75` | $P \propto M^{0.75}$ | `biology::bioenergetics` |
| ATP yield aerobic | `pub const ATP_YIELD_AEROBIC: f64 = 30.0` | ATP per glucose (aerobic) | `biology::bioenergetics` |
| ATP yield anaerobic | `pub const ATP_YIELD_ANAEROBIC: f64 = 2.0` | ATP per glucose (anaerobic) | `biology::bioenergetics` |
| CAC NADH per acetyl-CoA | `pub const CAC_NADH_PER_ACETYL_COA: f64 = 3.0` | Citric acid cycle yield | `biology::bioenergetics` |
| CAC FADH₂ per acetyl-CoA | `pub const CAC_FADH2_PER_ACETYL_COA: f64 = 1.0` | Citric acid cycle yield | `biology::bioenergetics` |
| β-oxidation FADH₂ ATP | `pub const BETA_OX_FADH2_ATP: f64 = 1.5` | ATP equivalent | `biology::bioenergetics` |
| β-oxidation NADH ATP | `pub const BETA_OX_NADH_ATP: f64 = 2.5` | ATP equivalent | `biology::bioenergetics` |
| β-oxidation acetyl-CoA ATP | `pub const BETA_OX_ACETYL_COA_ATP: f64 = 10.0` | ATP equivalent | `biology::bioenergetics` |
| β-oxidation activation cost | `pub const BETA_OX_ACTIVATION_COST: f64 = 2.0` | ATP cost | `biology::bioenergetics` |
| Body temperature | `pub const BODY_TEMP_KELVIN: f64 = 310.15` | 37°C in K | `biology::bioenergetics` |
| Reference temperature | `pub const REFERENCE_TEMP_KELVIN: f64 = 298.15` | 25°C in K | `biology::bioenergetics` |

#### `biology::neuroscience` — 58 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| HH resting potential | `pub const HH_RESTING_POTENTIAL: f64 = -65.0` | Hodgkin-Huxley $V_{\text{rest}}$ (mV) | `biology::neuroscience` |
| HH init m | `pub const HH_INIT_M: f64 = 0.05` | Na⁺ activation gate | `biology::neuroscience` |
| HH init h | `pub const HH_INIT_H: f64 = 0.6` | Na⁺ inactivation gate | `biology::neuroscience` |
| HH init n | `pub const HH_INIT_N: f64 = 0.32` | K⁺ activation gate | `biology::neuroscience` |
| HH membrane capacitance | `pub const HH_MEMBRANE_CAPACITANCE: f64 = 1.0` | $C_m$ (µF/cm²) | `biology::neuroscience` |
| HH $g_{\text{Na}}$ | `pub const HH_G_NA: f64 = 120.0` | Na⁺ max conductance (mS/cm²) | `biology::neuroscience` |
| HH $g_K$ | `pub const HH_G_K: f64 = 36.0` | K⁺ max conductance (mS/cm²) | `biology::neuroscience` |
| HH $g_L$ | `pub const HH_G_L: f64 = 0.3` | Leak conductance (mS/cm²) | `biology::neuroscience` |
| HH $E_{\text{Na}}$ | `pub const HH_E_NA: f64 = 50.0` | Na⁺ reversal potential (mV) | `biology::neuroscience` |
| HH $E_K$ | `pub const HH_E_K: f64 = -77.0` | K⁺ reversal potential (mV) | `biology::neuroscience` |
| HH $E_L$ | `pub const HH_E_L: f64 = -54.4` | Leak reversal potential (mV) | `biology::neuroscience` |

+12 HH gating coefficients, +21 Izhikevich model parameters (RS, FS, BURST types), +11 Morris-Lecar model parameters, +4 Leaky Integrate-and-Fire parameters, +3 synaptic plasticity constants.

#### `biology::physiology` — 22 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Windchill constant | `pub const WINDCHILL_CONSTANT: f64 = 13.12` | Windchill index base | `biology::physiology` |
| Windchill temp coeff | `pub const WINDCHILL_TEMP_COEFF: f64 = 0.6215` | Temperature coefficient | `biology::physiology` |
| Windchill wind coeff | `pub const WINDCHILL_WIND_COEFF: f64 = 11.37` | Wind speed coefficient | `biology::physiology` |
| Windchill wind exp | `pub const WINDCHILL_WIND_EXP: f64 = 0.16` | Wind speed exponent | `biology::physiology` |
| Windchill interaction | `pub const WINDCHILL_INTERACTION: f64 = 0.3965` | Interaction term | `biology::physiology` |
| Heat index constant | `pub const HEAT_INDEX_CONSTANT: f64 = -42.379` | Heat index base | `biology::physiology` |
| Heat index temp coeff | `pub const HEAT_INDEX_TEMP_COEFF: f64 = 2.049_015_23` | Temperature coefficient | `biology::physiology` |
| BSA DuBois coefficient | `pub const BSA_DUBOIS_COEFF: f64 = 0.007_184` | Body surface area coefficient | `biology::physiology` |
| BSA weight exponent | `pub const BSA_WEIGHT_EXP: f64 = 0.425` | Weight exponent | `biology::physiology` |
| BSA height exponent | `pub const BSA_HEIGHT_EXP: f64 = 0.725` | Height exponent | `biology::physiology` |
| Hb O₂ capacity | `pub const HB_O2_CAPACITY: f64 = 1.34` | mL O₂ per g Hb | `biology::physiology` |
| Dissolved O₂ coeff | `pub const DISSOLVED_O2_COEFF: f64 = 0.003` | Henry's law coefficient | `biology::physiology` |
| O₂ delivery scaling | `pub const O2_DELIVERY_SCALING: f64 = 10.0` | Delivery factor | `biology::physiology` |
| mmHg to Pa | `pub const MMHG_TO_PA: f64 = 133.322` | Pressure conversion | `biology::physiology` |
| Windkessel systolic fraction | `pub const WINDKESSEL_SYSTOLIC_FRACTION: f64 = 0.35` | Cardiac cycle fraction | `biology::physiology` |

+7 additional heat index regression coefficients.

#### `biology::radiobiology` — 10 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| DSB yield per Gy | `pub const DSB_YIELD_PER_GY: f64 = 35.0` | Double-strand breaks per Gy | `biology::radiobiology` |
| SSB yield per Gy | `pub const SSB_YIELD_PER_GY: f64 = 1000.0` | Single-strand breaks per Gy | `biology::radiobiology` |
| Oxidative damage coeff | `pub const OXIDATIVE_DAMAGE_COEFF: f64 = 200.0` | Oxidative damage coefficient | `biology::radiobiology` |
| Oxidative damage O₂ Km | `pub const OXIDATIVE_DAMAGE_O2_KM: f64 = 3.0` | Michaelis constant (mmHg) | `biology::radiobiology` |
| DNA damage DSB weight | `pub const DNA_DAMAGE_DSB_WEIGHT: f64 = 10.0` | DSB weighting factor | `biology::radiobiology` |
| DNA damage base weight | `pub const DNA_DAMAGE_BASE_WEIGHT: f64 = 2.0` | Base damage weight | `biology::radiobiology` |
| ln 2 | `pub const LN_2: f64 = std::f64::consts::LN_2` | $\ln 2$ | `biology::radiobiology` |
| Farquhar Wj Ci coeff | `pub const FARQUHAR_WJ_CI_COEFF: f64 = 4.0` | Photosynthesis model | `biology::radiobiology` |
| Farquhar Wj Γ coeff | `pub const FARQUHAR_WJ_GAMMA_COEFF: f64 = 8.0` | Photosynthesis model | `biology::radiobiology` |
| Tumor carrying capacity | `pub const TUMOR_CARRYING_CAPACITY: f64 = 1e12` | Gompertz model $K$ | `biology::radiobiology` |

### 3️⃣ New — Geology Constants — `constants::geology` — 3 files, 53 pub const

#### `geology::impact` — 7 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Crater scaling coeff | `pub const CRATER_SCALING_COEFF: f64 = 1.161` | Pi-scaling exponent | `geology::impact` |
| Crater density exp | `pub const CRATER_DENSITY_EXPONENT: f64 = 1.0 / 3.0` | Target density exponent | `geology::impact` |
| Crater projectile exp | `pub const CRATER_PROJECTILE_EXPONENT: f64 = 0.78` | Projectile diameter exponent | `geology::impact` |
| Crater velocity exp | `pub const CRATER_VELOCITY_EXPONENT: f64 = 0.44` | Impact velocity exponent | `geology::impact` |
| Crater gravity exp | `pub const CRATER_GRAVITY_EXPONENT: f64 = -0.22` | Gravity exponent | `geology::impact` |
| Fireball radius coeff | `pub const FIREBALL_RADIUS_COEFF: f64 = 55.0` | Fireball scaling | `geology::impact` |
| Fireball energy exp | `pub const FIREBALL_ENERGY_EXPONENT: f64 = 0.4` | Energy exponent | `geology::impact` |

#### `geology::radioactive` — 10 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| $\lambda_{238}$ | `pub const LAMBDA_U238: f64 = 1.551_25e-10` | $^{238}$U decay constant (yr⁻¹) | `geology::radioactive` |
| $\lambda_{235}$ | `pub const LAMBDA_U235: f64 = 9.8485e-10` | $^{235}$U decay constant (yr⁻¹) | `geology::radioactive` |
| $\lambda_{232}$ | `pub const LAMBDA_TH232: f64 = 4.947_5e-11` | $^{232}$Th decay constant (yr⁻¹) | `geology::radioactive` |
| $\lambda_{40K}$ total | `pub const LAMBDA_K40_TOTAL: f64 = 5.543e-10` | $^{40}$K total decay (yr⁻¹) | `geology::radioactive` |
| $\lambda_{40K}$ Ar | `pub const LAMBDA_K40_AR: f64` | Derived Ar branch | `geology::radioactive` |
| K-40 branch ratio Ar | `pub const K40_BRANCH_RATIO_AR: f64 = 0.1048` | Ar branching ratio | `geology::radioactive` |
| C-14 mean life | `pub const C14_MEAN_LIFE: f64 = 8_267.0` | $^{14}$C mean life (yr) | `geology::radioactive` |
| Heat production U-238 | `pub const HEAT_PRODUCTION_U238: f64 = 9.46e-5` | W/kg | `geology::radioactive` |
| Heat production Th-232 | `pub const HEAT_PRODUCTION_TH232: f64 = 2.64e-5` | W/kg | `geology::radioactive` |
| Heat production K-40 | `pub const HEAT_PRODUCTION_K40: f64 = 2.92e-5` | W/kg | `geology::radioactive` |

#### `geology::seawater` — 36 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Seawater $\rho_0$ | `pub const SEAWATER_RHO_0: f64 = 999.842_594` | Reference density (kg/m³) | `geology::seawater` |
| Sound speed base | `pub const SOUND_SPEED_BASE: f64 = 1448.96` | UNESCO formula base (m/s) | `geology::seawater` |
| Phillips $\alpha$ | `pub const PHILLIPS_ALPHA: f64 = 0.0081` | Wave spectrum constant | `geology::seawater` |
| Phillips $\beta$ | `pub const PHILLIPS_BETA: f64 = 0.74` | Wave spectrum constant | `geology::seawater` |
| Haline contraction | `pub const HALINE_CONTRACTION_0: f64 = 7.5e-4` | $\beta_S$ (PSU⁻¹) | `geology::seawater` |
| Thermal expansion | `pub const THERMAL_EXPANSION_0: f64 = 1.67e-4` | $\alpha_T$ (K⁻¹) | `geology::seawater` |

+14 density polynomial coefficients, +8 sound speed correction terms, +4 JONSWAP and additional ocean parameters.

### 4️⃣ New — Mathematics Constants — `constants::maths` — 3 files, 57 pub const

#### `maths::approximation` — 16 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| erf $a_1$ | `pub const ERF_A1: f64 = 0.254_829_592` | Error function approximation | `maths::approximation` |
| erf $a_2$ | `pub const ERF_A2: f64 = -0.284_496_736` | Error function approximation | `maths::approximation` |
| erf $a_3$ | `pub const ERF_A3: f64 = 1.421_413_741` | Error function approximation | `maths::approximation` |
| erf $a_4$ | `pub const ERF_A4: f64 = -1.453_152_027` | Error function approximation | `maths::approximation` |
| erf $a_5$ | `pub const ERF_A5: f64 = 1.061_405_429` | Error function approximation | `maths::approximation` |
| erf $p$ | `pub const ERF_P: f64 = 0.327_591_1` | Error function scaling | `maths::approximation` |
| Lanczos coefficients | `pub const LANCZOS_COEFFS_6: [f64; 6]` | Lanczos gamma 6-term | `maths::approximation` |
| Lanczos series init | `pub const LANCZOS_SERIES_INIT: f64 = 1.000_000_000_190_015` | Series initial value | `maths::approximation` |
| Lanczos $\sqrt{2\pi}$ | `pub const LANCZOS_SQRT_2PI: f64 = 2.506_628_274_631_000_5` | $\sqrt{2\pi}$ | `maths::approximation` |
| Stirling-Lanczos $g$ | `pub const STIRLING_LANCZOS_G: f64 = 7.0` | Stirling parameter | `maths::approximation` |
| Stirling-Lanczos coeffs | `pub const STIRLING_LANCZOS_COEFFS: [f64; 9]` | 9-term coefficients | `maths::approximation` |
| Golden ratio conjugate | `pub const GOLDEN_RATIO_CONJUGATE: f64 = 0.618_033_988_749_894_9` | $\phi - 1$ | `maths::approximation` |
| Savitzky-Golay 5 coeffs | `pub const SAVITZKY_GOLAY_5_COEFFS: [f64; 5]` | 5-point smoothing | `maths::approximation` |
| Savitzky-Golay 5 norm | `pub const SAVITZKY_GOLAY_5_NORM: f64 = 35.0` | Normalization factor | `maths::approximation` |
| Speed of light (km/s) | `pub const SPEED_OF_LIGHT_KM_S: f64` | Derived from fundamental | `maths::approximation` |
| CMB recombination $z$ | `pub const CMB_RECOMBINATION_REDSHIFT: f64` | Derived from darkmatter | `maths::approximation` |

#### `maths::ode` — 30 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| RK45 $a_2$ — $a_6$ | `pub const RK45_A2..RK45_A6: f64` | Runge-Kutta-Fehlberg nodes (5 const) | `maths::ode` |
| RK45 $b_{ij}$ | `pub const RK45_B21..RK45_B65: f64` | RKF coupling coefficients (15 const) | `maths::ode` |
| RK45 $c_i$ | `pub const RK45_C1..RK45_C6: f64` | RKF 4th-order weights (5 const) | `maths::ode` |
| RK45 $d_i$ | `pub const RK45_D1..RK45_D5: f64` | RKF 5th-order weights (4 const) | `maths::ode` |
| Adams-Bashforth 4 | `pub const ADAMS_BASHFORTH_4: [f64; 4]` | 4-step AB coefficients | `maths::ode` |

#### `maths::quadrature` — 11 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Gauss-Legendre 5 nodes | `pub const GAUSS_LEGENDRE_5_NODES: [f64; 5]` | GL quadrature points | `maths::quadrature` |
| Gauss-Legendre 5 weights | `pub const GAUSS_LEGENDRE_5_WEIGHTS: [f64; 5]` | GL quadrature weights | `maths::quadrature` |
| Gauss-Laguerre 5 nodes | `pub const GAUSS_LAGUERRE_5_NODES: [f64; 5]` | GL quadrature points | `maths::quadrature` |
| Gauss-Laguerre 5 weights | `pub const GAUSS_LAGUERRE_5_WEIGHTS: [f64; 5]` | GL quadrature weights | `maths::quadrature` |
| Gauss-Hermite 5 nodes | `pub const GAUSS_HERMITE_5_NODES: [f64; 5]` | GH quadrature points | `maths::quadrature` |
| Gauss-Hermite 5 weights | `pub const GAUSS_HERMITE_5_WEIGHTS: [f64; 5]` | GH quadrature weights | `maths::quadrature` |
| Lobatto 5 nodes | `pub const LOBATTO_5_NODES: [f64; 5]` | Lobatto quadrature points | `maths::quadrature` |
| Lobatto 5 weights | `pub const LOBATTO_5_WEIGHTS: [f64; 5]` | Lobatto quadrature weights | `maths::quadrature` |
| Gauss-Kronrod 15 nodes | `pub const GAUSS_KRONROD_15_NODES: [f64; 15]` | GK quadrature points | `maths::quadrature` |
| Gauss-Kronrod 15 weights | `pub const GAUSS_KRONROD_15_WEIGHTS: [f64; 15]` | GK quadrature weights | `maths::quadrature` |
| Gauss 7 weights | `pub const GAUSS_7_WEIGHTS: [f64; 7]` | G7 quadrature weights | `maths::quadrature` |

### 5️⃣ New — Meteorology Constants — `constants::meteorology` — 1 file, 18 pub const

#### `meteorology::atmospheric` — 18 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| $R_d$ | `pub const SPECIFIC_GAS_CONSTANT_DRY_AIR: f64 = 287.058` | Dry air gas constant (J/kg·K) | `meteorology::atmospheric` |
| $R_v$ | `pub const SPECIFIC_GAS_CONSTANT_WATER_VAPOR: f64 = 461.5` | Water vapor gas constant (J/kg·K) | `meteorology::atmospheric` |
| $c_{pd}$ | `pub const SPECIFIC_HEAT_DRY_AIR: f64 = 1003.5` | Specific heat dry air (J/kg·K) | `meteorology::atmospheric` |
| Dry adiabatic lapse rate | `pub const DRY_ADIABATIC_LAPSE_RATE: f64` | $\Gamma_d = g/c_{pd}$ (K/m) | `meteorology::atmospheric` |
| Virtual temp factor | `pub const VIRTUAL_TEMP_FACTOR: f64 = 0.608` | $T_v = T(1 + 0.608 q)$ | `meteorology::atmospheric` |
| Mixing ratio factor | `pub const MIXING_RATIO_FACTOR: f64 = 0.622` | $\epsilon = R_d/R_v$ | `meteorology::atmospheric` |
| Magnus $A$ | `pub const MAGNUS_A: f64 = 17.67` | Saturation vapor pressure | `meteorology::atmospheric` |
| Magnus $B$ | `pub const MAGNUS_B: f64 = 243.5` | Saturation vapor pressure (°C) | `meteorology::atmospheric` |
| $e_{s0}$ | `pub const SATURATION_VAPOR_PRESSURE_0C: f64 = 6.112` | $e_s$ at 0°C (hPa) | `meteorology::atmospheric` |
| Density altitude scale | `pub const DENSITY_ALTITUDE_SCALE: f64 = 120.0` | Scaling factor (ft/°C) | `meteorology::atmospheric` |
| ISA sea-level temp | `pub const ISA_SEA_LEVEL_TEMP_C: f64 = 15.0` | ISA reference (°C) | `meteorology::atmospheric` |
| ISA lapse rate | `pub const ISA_LAPSE_RATE_PER_M: f64 = 0.002` | ISA lapse (°C/m) | `meteorology::atmospheric` |
| von Kármán | `pub const VON_KARMAN: f64 = 0.41` | $\kappa$ | `meteorology::atmospheric` |
| Air density sea level | `pub const STANDARD_AIR_DENSITY_SEA_LEVEL: f64 = 1.225` | $\rho_0$ (kg/m³) | `meteorology::atmospheric` |
| CO₂ radiative forcing | `pub const RADIATIVE_FORCING_CO2_COEFF: f64 = 5.35` | $\Delta F = 5.35 \ln(C/C_0)$ (W/m²) | `meteorology::atmospheric` |
| Solar constant | `pub const SOLAR_CONSTANT: f64 = 1361.0` | TSI (W/m²) | `meteorology::atmospheric` |
| Beaufort to m/s coeff | `pub const BEAUFORT_TO_MS_COEFF: f64 = 0.836` | Beaufort conversion | `meteorology::atmospheric` |
| Beaufort to m/s exp | `pub const BEAUFORT_TO_MS_EXPONENT: f64 = 1.5` | Beaufort conversion | `meteorology::atmospheric` |

### 6️⃣ Expanded — Physics Constants — `constants::physics` — 4 files, 59 pub const + 1 struct

#### `physics::fundamental` — 20 pub const (+3 new)

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Fermi constant | `pub const G_F: f64 = 1.166_378_7e-5` | $G_F$ (GeV⁻²) | `physics::fundamental` |
| Strong coupling | `pub const ALPHA_S: f64 = 0.1181` | $\alpha_s(M_Z)$ | `physics::fundamental` |
| Faraday constant | `pub const FARADAY: f64` | $F = N_A e$ (C/mol) | `physics::fundamental` |

#### `physics::atomic` — 18 pub const (+5 new)

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Muon mass | `pub const MUON_MASS: f64 = 1.883_531_627e-28` | $m_\mu$ (kg) | `physics::atomic` |
| Tau mass | `pub const TAU_MASS: f64 = 3.167_47e-27` | $m_\tau$ (kg) | `physics::atomic` |
| Neutrino mass upper | `pub const NEUTRINO_MASS_UPPER: f64 = 2.2e-36` | Upper bound (kg) | `physics::atomic` |
| Electron rest mass MeV | `pub const ELECTRON_REST_MASS_MEV: f64` | Derived (MeV) | `physics::atomic` |
| Rydberg constant | `pub const R_INF: f64 = 10_973_731.568_16` | $R_\infty$ (m⁻¹) | `physics::atomic` |

#### `physics::units` — 20 pub const (+8 new)

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| $hc$ in eV·nm | `pub const HC_EV_NM: f64` | $hc$ (eV·nm) | `physics::units` |
| MeV to J | `pub const MEV_TO_J: f64` | Derived conversion | `physics::units` |
| kWh to J | `pub const KWH_TO_J: f64 = 3.6e6` | Energy conversion | `physics::units` |
| Seconds per day | `pub const SECONDS_PER_DAY: f64 = 86_400.0` | Time conversion | `physics::units` |
| J2000 epoch JD | `pub const J2000_EPOCH_JD: f64 = 2_451_545.0` | Julian date reference | `physics::units` |
| Julian century | `pub const JULIAN_CENTURY: f64 = 36_525.0` | Days per Julian century | `physics::units` |
| Sidereal year | `pub const SIDEREAL_YEAR: f64 = 3.155_815e7` | Duration (s) | `physics::units` |
| Tropical year | `pub const TROPICAL_YEAR: f64 = 3.155_693e7` | Duration (s) | `physics::units` |

#### `physics::elements` — 1 pub struct, 4 pub fn

| Item | Declaration | Description | Module |
|---|---|---|---|
| Element struct | `pub struct Element { symbol, name, atomic_number, atomic_mass, electronegativity, group, period }` | 118 elements from YAML | `physics::elements` |
| By atomic number | `by_atomic_number(z: u32) → Option<&'static Element>` | Lookup by $Z$ | `physics::elements` |
| By symbol | `by_symbol(sym: &str) → Option<&'static Element>` | Lookup by symbol | `physics::elements` |
| Atomic mass | `atomic_mass(z: u32) → f64` | Mass by $Z$ | `physics::elements` |
| Electronegativity | `electronegativity(z: u32) → Option<f64>` | Pauling electronegativity by $Z$ | `physics::elements` |

---

## v0.0.2

### Changes

| Metric | Value |
|---|---|
| Structure | Identical to v0.0.1 (5 flat files) |

---

## v0.0.1

### 1️⃣ Fundamental Constants — `constants::fundamental` — 17 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Speed of light | `pub const C: f64 = 299_792_458.0` | $c$ (m/s) — CODATA | `constants::fundamental` |
| Gravitational constant | `pub const G: f64 = 6.674_30e-11` | $G$ (m³/kg·s²) | `constants::fundamental` |
| Planck constant | `pub const H: f64 = 6.626_070_15e-34` | $h$ (J·s) | `constants::fundamental` |
| Reduced Planck | `pub const HBAR: f64 = 1.054_571_817e-34` | $\hbar = h/2\pi$ (J·s) | `constants::fundamental` |
| Boltzmann constant | `pub const K_B: f64 = 1.380_649e-23` | $k_B$ (J/K) | `constants::fundamental` |
| Avogadro number | `pub const N_A: f64 = 6.022_140_76e23` | $N_A$ (mol⁻¹) | `constants::fundamental` |
| Elementary charge | `pub const E_CHARGE: f64 = 1.602_176_634e-19` | $e$ (C) | `constants::fundamental` |
| Vacuum permittivity | `pub const EPSILON_0: f64 = 8.854_187_812_8e-12` | $\varepsilon_0$ (F/m) | `constants::fundamental` |
| Vacuum permeability | `pub const MU_0: f64 = 1.256_637_062_12e-6` | $\mu_0$ (H/m) | `constants::fundamental` |
| Coulomb constant | `pub const K_COULOMB: f64 = 8.987_551_792_3e9` | $k_e = 1/4\pi\varepsilon_0$ (N·m²/C²) | `constants::fundamental` |
| Stefan-Boltzmann | `pub const SIGMA_SB: f64 = 5.670_374_419e-8` | $\sigma$ (W/m²·K⁴) | `constants::fundamental` |
| Gas constant | `pub const R_GAS: f64 = 8.314_462_618` | $R$ (J/mol·K) | `constants::fundamental` |
| Fine-structure constant | `pub const ALPHA_FINE: f64 = 7.297_352_569_3e-3` | $\alpha \approx 1/137$ | `constants::fundamental` |
| Planck length | `pub const PLANCK_LENGTH: f64 = 1.616_255e-35` | $l_P$ (m) | `constants::fundamental` |
| Planck mass | `pub const PLANCK_MASS: f64 = 2.176_434e-8` | $m_P$ (kg) | `constants::fundamental` |
| Planck time | `pub const PLANCK_TIME: f64 = 5.391_247e-44` | $t_P$ (s) | `constants::fundamental` |
| Planck temperature | `pub const PLANCK_TEMP: f64 = 1.416_784e32` | $T_P$ (K) | `constants::fundamental` |

### 2️⃣ Astronomical Constants — `constants::astro` — 10 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Astronomical unit | `pub const AU: f64 = 1.495_978_707e11` | $1\,\text{AU}$ (m) | `constants::astro` |
| Parsec | `pub const PARSEC: f64 = 3.085_677_581e16` | 1 pc (m) | `constants::astro` |
| Light-year | `pub const LIGHT_YEAR: f64 = 9.460_730_472_580_8e15` | 1 ly (m) | `constants::astro` |
| Solar mass | `pub const SOLAR_MASS: f64 = 1.989e30` | $M_\odot$ (kg) | `constants::astro` |
| Solar radius | `pub const SOLAR_RADIUS: f64 = 6.957e8` | $R_\odot$ (m) | `constants::astro` |
| Solar luminosity | `pub const SOLAR_LUMINOSITY: f64 = 3.828e26` | $L_\odot$ (W) | `constants::astro` |
| Earth mass | `pub const EARTH_MASS: f64 = 5.972e24` | $M_\oplus$ (kg) | `constants::astro` |
| Earth radius | `pub const EARTH_RADIUS: f64 = 6.371e6` | $R_\oplus$ (m) | `constants::astro` |
| Hubble constant | `pub const HUBBLE_CONSTANT: f64 = 67.4` | $H_0$ (km/s/Mpc) | `constants::astro` |
| CMB temperature | `pub const CMB_TEMPERATURE: f64 = 2.7255` | $T_{\text{CMB}}$ (K) | `constants::astro` |

### 3️⃣ Atomic Constants — `constants::atomic` — 13 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| Proton mass | `pub const PROTON_MASS: f64 = 1.672_621_923_69e-27` | $m_p$ (kg) | `constants::atomic` |
| Neutron mass | `pub const NEUTRON_MASS: f64 = 1.674_927_498_04e-27` | $m_n$ (kg) | `constants::atomic` |
| Electron mass | `pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31` | $m_e$ (kg) | `constants::atomic` |
| Proton mass (AMU) | `pub const PROTON_MASS_AMU: f64 = 1.007_276_466_621` | $m_p$ (u) | `constants::atomic` |
| Neutron mass (AMU) | `pub const NEUTRON_MASS_AMU: f64 = 1.008_664_915_95` | $m_n$ (u) | `constants::atomic` |
| Electron mass (AMU) | `pub const ELECTRON_MASS_AMU: f64 = 5.485_799_090_65e-4` | $m_e$ (u) | `constants::atomic` |
| AMU to kg | `pub const AMU_TO_KG: f64 = 1.660_539_066_60e-27` | Conversion factor | `constants::atomic` |
| AMU to MeV | `pub const AMU_TO_MEV: f64 = 931.494_102_42` | $E = mc^2$ conversion | `constants::atomic` |
| Bohr radius | `pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11` | $a_0$ (m) | `constants::atomic` |
| Rydberg energy | `pub const RYDBERG_ENERGY: f64 = 13.605_693_122_994` | $E_R$ (eV) | `constants::atomic` |
| Bohr magneton | `pub const BOHR_MAGNETON: f64 = 9.274_010_078_3e-24` | $\mu_B$ (J/T) | `constants::atomic` |
| Nuclear magneton | `pub const NUCLEAR_MAGNETON: f64 = 5.050_783_746_1e-27` | $\mu_N$ (J/T) | `constants::atomic` |
| Compton wavelength | `pub const COMPTON_WAVELENGTH: f64 = 2.426_310_238_67e-12` | $\lambda_C$ (m) | `constants::atomic` |

### 4️⃣ Unit Conversions — `constants::units` — 12 pub const

| Constant | Declaration | Value / Description | Module |
|---|---|---|---|
| eV to Joule | `pub const EV_TO_JOULE: f64 = 1.602_176_634e-19` | Energy conversion | `constants::units` |
| Joule to eV | `pub const JOULE_TO_EV: f64 = 6.241_509_074e18` | Energy conversion | `constants::units` |
| keV to Kelvin | `pub const KEV_TO_KELVIN: f64 = 1.160_451_812e7` | Thermal conversion | `constants::units` |
| Kelvin to keV | `pub const KELVIN_TO_KEV: f64 = 8.617_333_262e-5` | Thermal conversion | `constants::units` |
| Calorie to Joule | `pub const CALORIE_TO_JOULE: f64 = 4.184` | Energy conversion | `constants::units` |
| atm to Pascal | `pub const ATM_TO_PASCAL: f64 = 101_325.0` | Pressure conversion | `constants::units` |
| bar to Pascal | `pub const BAR_TO_PASCAL: f64 = 100_000.0` | Pressure conversion | `constants::units` |
| Degree to radian | `pub const DEGREE_TO_RAD: f64 = 0.017_453_292_519_943_3` | Angle conversion | `constants::units` |
| Radian to degree | `pub const RAD_TO_DEGREE: f64 = 57.295_779_513_082_32` | Angle conversion | `constants::units` |
| Barn | `pub const BARN: f64 = 1.0e-28` | Cross-section (m²) | `constants::units` |
| Ångström | `pub const ANGSTROM: f64 = 1.0e-10` | Length (m) | `constants::units` |
| Fermi | `pub const FERMI: f64 = 1.0e-15` | Length (m) | `constants::units` |

### 5️⃣ Elements — `constants::elements` — 1 pub struct, 118 elements

| Item | Declaration | Description | Module |
|---|---|---|---|
| Element struct | `pub struct Element { symbol: &'static str, name: &'static str, atomic_number: u32, atomic_mass: f64, electronegativity: Option<f64>, group: u32, period: u32 }` | Per-element data type | `constants::elements` |
| Element data | 118 `Element` instances | Loaded from `tableau-periodique/` YAML via `include_str!` | `constants::elements` |
| Parser | `src/parser/yaml/` | Zero-copy `YamlParser` at compile time | `constants::elements` |
