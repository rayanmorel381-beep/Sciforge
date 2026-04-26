# Constants Source Code Guide

This page documents the source implementation behind `sciforge::constants`, including constant tables and lookup helpers.

## Source Coverage

### What is explained
- File-level implementation layout in `src/constants/`.
- Main constant groups and lookup mechanisms.
- Runtime usage path through Hub when constants are consumed in domain dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/constants/`
- Module entry point: `src/constants/mod.rs`
- Hub routing (when applicable): `(n/a - constants are direct)`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent constants or lookup block.
3. Domain dispatchers and Hub prelude re-exports consume these constants internally.

### What to verify in source code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `astronomy` — astronomical constants (solar/planetary masses, distances, CMB, dark matter, stellar, galactic)
- `biology` — biological constants (bioenergetics, neuroscience, physiology, radiobiology)
- `geology` — geological constants (impact parameters, radioactive decay, seawater composition)
- `maths` — mathematical constants (approximation coefficients, ODE tables, quadrature weights)
- `meteorology` — meteorological constants (atmospheric properties)
- `physics` — physical constants (atomic, elements, fundamental, units)

---

### astronomy/astrophysics (22 items)

| Name | Type | Value |
|------|------|-------|
| `ML_LOW_THRESHOLD` | `f64` | `0.43` |
| `ML_LOW_COEFF` | `f64` | `0.23` |
| `ML_LOW_EXPONENT` | `f64` | `2.3` |
| `ML_MID_THRESHOLD` | `f64` | `2.0` |
| `ML_MID_EXPONENT` | `f64` | `4.0` |
| `ML_HIGH_THRESHOLD` | `f64` | `55.0` |
| `ML_HIGH_COEFF` | `f64` | `1.4` |
| `ML_HIGH_EXPONENT` | `f64` | `3.5` |
| `ML_VERY_HIGH_COEFF` | `f64` | `32000.0` |
| `WIEN_DISPLACEMENT` | `f64` | `2.897_771_955e-3` |
| `MAIN_SEQUENCE_LIFETIME_SCALE_YR` | `f64` | `1e10` |
| `CHANDRASEKHAR_LIMIT_SOLAR` | `f64` | `1.4` |
| `SPECTRAL_TEMP_SCALE` | `f64` | `42000.0` |
| `SPECTRAL_INDEX_OFFSET` | `f64` | `0.92` |
| `BOLOMETRIC_CORRECTION_QUAD` | `f64` | `-8.499` |
| `BOLOMETRIC_CORRECTION_CENTER` | `f64` | `4.012` |
| `BOLOMETRIC_CORRECTION_OFFSET` | `f64` | `-0.09` |
| `HABITABLE_ZONE_INNER_FLUX` | `f64` | `1.1` |
| `HABITABLE_ZONE_OUTER_FLUX` | `f64` | `0.53` |
| `NUTATION_AMPLITUDE_ARCSEC` | `f64` | `9.2` |
| `VERNAL_EQUINOX_DOY` | `f64` | `81.0` |
| `TIDAL_DISSIPATION_COEFF` | `f64` | `10.5` |

---

### astronomy/astro (196 items)

| Name | Type | Value |
|------|------|-------|
| `AU` | `f64` | `1.495_978_707e11` |
| `PARSEC` | `f64` | `3.085_677_581e16` |
| `LIGHT_YEAR` | `f64` | `9.460_730_472_580_8e15` |
| `SOLAR_MASS` | `f64` | `1.989_1e30` |
| `SOLAR_RADIUS` | `f64` | `6.957e8` |
| `SOLAR_LUMINOSITY` | `f64` | `3.828e26` |
| `EARTH_MASS` | `f64` | `5.972_37e24` |
| `EARTH_RADIUS` | `f64` | `6.371_0e6` |
| `HUBBLE_CONSTANT` | `f64` | `67.4` |
| `CMB_TEMPERATURE` | `f64` | `2.725` |
| `SOLAR_DENSITY` | `f64` | `SOLAR_MASS / (4.0 / 3.0 * std::f64::consts::PI * SOLAR_RADIUS * SOLAR_RADIUS * SOLAR_RADIUS)` |
| `SOLAR_GRAVITY` | `f64` | `crate::constants::G * SOLAR_MASS / (SOLAR_RADIUS * SOLAR_RADIUS)` |
| `EARTH_GRAVITY` | `f64` | `9.806_65` |
| `COSMOLOGICAL_LAMBDA` | `f64` | `1.105_6e-52` |
| `MERCURY_MASS` | `f64` | `3.301_1e23` |
| `MERCURY_RADIUS` | `f64` | `2.439_7e6` |
| `MERCURY_FLATTENING` | `f64` | `0.000_9` |
| `MERCURY_ORBITAL_PERIOD` | `f64` | `7.600_5e6` |
| `MERCURY_SEMI_MAJOR_AXIS` | `f64` | `5.790_9e10` |
| `MERCURY_ECCENTRICITY` | `f64` | `0.205_6` |
| `MERCURY_INCLINATION` | `f64` | `7.005` |
| `MERCURY_AXIAL_TILT` | `f64` | `0.034` |
| `MERCURY_SIDEREAL_DAY` | `f64` | `5.067_4e6` |
| `MERCURY_SURFACE_GRAVITY` | `f64` | `3.7` |
| `MERCURY_ESCAPE_VELOCITY` | `f64` | `4250.0` |
| `MERCURY_MEAN_DENSITY` | `f64` | `5427.0` |
| `MERCURY_BOND_ALBEDO` | `f64` | `0.088` |
| `VENUS_MASS` | `f64` | `4.867_5e24` |
| `VENUS_RADIUS` | `f64` | `6.051_8e6` |
| `VENUS_FLATTENING` | `f64` | `0.0` |
| `VENUS_ORBITAL_PERIOD` | `f64` | `1.941_4e7` |
| `VENUS_SEMI_MAJOR_AXIS` | `f64` | `1.082_1e11` |
| `VENUS_ECCENTRICITY` | `f64` | `0.006_8` |
| `VENUS_INCLINATION` | `f64` | `3.394_6` |
| `VENUS_AXIAL_TILT` | `f64` | `177.36` |
| `VENUS_SIDEREAL_DAY` | `f64` | `2.099_7e7` |
| `VENUS_SURFACE_GRAVITY` | `f64` | `8.87` |
| `VENUS_ESCAPE_VELOCITY` | `f64` | `10_360.0` |
| `VENUS_MEAN_DENSITY` | `f64` | `5243.0` |
| `VENUS_BOND_ALBEDO` | `f64` | `0.76` |
| `MARS_MASS` | `f64` | `6.417e23` |
| `MARS_RADIUS` | `f64` | `3.389_5e6` |
| `MARS_FLATTENING` | `f64` | `0.005_89` |
| `MARS_ORBITAL_PERIOD` | `f64` | `5.935_5e7` |
| `MARS_SEMI_MAJOR_AXIS` | `f64` | `2.279_4e11` |
| `MARS_ECCENTRICITY` | `f64` | `0.093_4` |
| `MARS_INCLINATION` | `f64` | `1.850` |
| `MARS_AXIAL_TILT` | `f64` | `25.19` |
| `MARS_SIDEREAL_DAY` | `f64` | `88_643.0` |
| `MARS_SURFACE_GRAVITY` | `f64` | `3.72` |
| `MARS_ESCAPE_VELOCITY` | `f64` | `5030.0` |
| `MARS_MEAN_DENSITY` | `f64` | `3934.0` |
| `MARS_BOND_ALBEDO` | `f64` | `0.250` |
| `JUPITER_MASS` | `f64` | `1.898_2e27` |
| `JUPITER_RADIUS` | `f64` | `6.991_1e7` |
| `JUPITER_FLATTENING` | `f64` | `0.064_87` |
| `JUPITER_ORBITAL_PERIOD` | `f64` | `3.743_5e8` |
| `JUPITER_SEMI_MAJOR_AXIS` | `f64` | `7.785_7e11` |
| `JUPITER_ECCENTRICITY` | `f64` | `0.048_9` |
| `JUPITER_INCLINATION` | `f64` | `1.303` |
| `JUPITER_AXIAL_TILT` | `f64` | `3.13` |
| `JUPITER_SIDEREAL_DAY` | `f64` | `35_730.0` |
| `JUPITER_SURFACE_GRAVITY` | `f64` | `24.79` |
| `JUPITER_ESCAPE_VELOCITY` | `f64` | `59_500.0` |
| `JUPITER_MEAN_DENSITY` | `f64` | `1326.0` |
| `JUPITER_BOND_ALBEDO` | `f64` | `0.343` |
| `SATURN_MASS` | `f64` | `5.683_4e26` |
| `SATURN_RADIUS` | `f64` | `5.823_2e7` |
| `SATURN_FLATTENING` | `f64` | `0.097_96` |
| `SATURN_ORBITAL_PERIOD` | `f64` | `9.295_6e8` |
| `SATURN_SEMI_MAJOR_AXIS` | `f64` | `1.433_5e12` |
| `SATURN_ECCENTRICITY` | `f64` | `0.056_5` |
| `SATURN_INCLINATION` | `f64` | `2.485` |
| `SATURN_AXIAL_TILT` | `f64` | `26.73` |
| `SATURN_SIDEREAL_DAY` | `f64` | `38_520.0` |
| `SATURN_SURFACE_GRAVITY` | `f64` | `10.44` |
| `SATURN_ESCAPE_VELOCITY` | `f64` | `35_500.0` |
| `SATURN_MEAN_DENSITY` | `f64` | `687.0` |
| `SATURN_BOND_ALBEDO` | `f64` | `0.342` |
| `URANUS_MASS` | `f64` | `8.681_0e25` |
| `URANUS_RADIUS` | `f64` | `2.536_2e7` |
| `URANUS_FLATTENING` | `f64` | `0.022_93` |
| `URANUS_ORBITAL_PERIOD` | `f64` | `2.651_3e9` |
| `URANUS_SEMI_MAJOR_AXIS` | `f64` | `2.872_5e12` |
| `URANUS_ECCENTRICITY` | `f64` | `0.045_7` |
| `URANUS_INCLINATION` | `f64` | `0.773` |
| `URANUS_AXIAL_TILT` | `f64` | `97.77` |
| `URANUS_SIDEREAL_DAY` | `f64` | `62_064.0` |
| `URANUS_SURFACE_GRAVITY` | `f64` | `8.69` |
| `URANUS_ESCAPE_VELOCITY` | `f64` | `21_300.0` |
| `URANUS_MEAN_DENSITY` | `f64` | `1270.0` |
| `URANUS_BOND_ALBEDO` | `f64` | `0.300` |
| `NEPTUNE_MASS` | `f64` | `1.024_13e26` |
| `NEPTUNE_RADIUS` | `f64` | `2.462_2e7` |
| `NEPTUNE_FLATTENING` | `f64` | `0.017_08` |
| `NEPTUNE_ORBITAL_PERIOD` | `f64` | `5.200_7e9` |
| `NEPTUNE_SEMI_MAJOR_AXIS` | `f64` | `4.495_1e12` |
| `NEPTUNE_ECCENTRICITY` | `f64` | `0.011_3` |
| `NEPTUNE_INCLINATION` | `f64` | `1.770` |
| `NEPTUNE_AXIAL_TILT` | `f64` | `28.32` |
| `NEPTUNE_SIDEREAL_DAY` | `f64` | `57_996.0` |
| `NEPTUNE_SURFACE_GRAVITY` | `f64` | `11.15` |
| `NEPTUNE_ESCAPE_VELOCITY` | `f64` | `23_500.0` |
| `NEPTUNE_MEAN_DENSITY` | `f64` | `1638.0` |
| `NEPTUNE_BOND_ALBEDO` | `f64` | `0.290` |
| `LUNAR_MASS` | `f64` | `7.342e22` |
| `LUNAR_RADIUS` | `f64` | `1.737_4e6` |
| `EARTH_MOON_DISTANCE` | `f64` | `3.844e8` |
| `LUNAR_ORBITAL_PERIOD` | `f64` | `2.360_6e6` |
| `IO_MASS` | `f64` | `8.931_9e22` |
| `IO_RADIUS` | `f64` | `1.821_6e6` |
| `EUROPA_MASS` | `f64` | `4.799_8e22` |
| `EUROPA_RADIUS` | `f64` | `1.560_8e6` |
| `GANYMEDE_MASS` | `f64` | `1.481_9e23` |
| `GANYMEDE_RADIUS` | `f64` | `2.634_1e6` |
| `CALLISTO_MASS` | `f64` | `1.075_9e23` |
| `CALLISTO_RADIUS` | `f64` | `2.410_3e6` |
| `TITAN_MASS` | `f64` | `1.345_2e23` |
| `TITAN_RADIUS` | `f64` | `2.574_7e6` |
| `ENCELADUS_MASS` | `f64` | `1.080_2e20` |
| `ENCELADUS_RADIUS` | `f64` | `2.521e5` |
| `TRITON_MASS` | `f64` | `2.139_0e22` |
| `TRITON_RADIUS` | `f64` | `1.353_4e6` |
| `PHOBOS_MASS` | `f64` | `1.065_9e16` |
| `PHOBOS_RADIUS` | `f64` | `1.126_7e4` |
| `DEIMOS_MASS` | `f64` | `1.476_2e15` |
| `DEIMOS_RADIUS` | `f64` | `6.2e3` |
| `OBERON_MASS` | `f64` | `3.014e21` |
| `OBERON_RADIUS` | `f64` | `7.614e5` |
| `TITANIA_MASS` | `f64` | `3.527e21` |
| `TITANIA_RADIUS` | `f64` | `7.889e5` |
| `TETHYS_MASS` | `f64` | `6.174_96e20` |
| `TETHYS_RADIUS` | `f64` | `5.311e5` |
| `DIONE_MASS` | `f64` | `1.095_452e21` |
| `DIONE_RADIUS` | `f64` | `5.613e5` |
| `RHEA_MASS` | `f64` | `2.306_518e21` |
| `RHEA_RADIUS` | `f64` | `7.638e5` |
| `IAPETUS_MASS` | `f64` | `1.805_635e21` |
| `IAPETUS_RADIUS` | `f64` | `7.346e5` |
| `HYPERION_MASS` | `f64` | `5.584e18` |
| `HYPERION_RADIUS` | `f64` | `1.35e5` |
| `MIMAS_MASS` | `f64` | `3.749_4e19` |
| `MIMAS_RADIUS` | `f64` | `1.982e5` |
| `PROMETHEUS_MASS` | `f64` | `1.595e17` |
| `PROMETHEUS_RADIUS` | `f64` | `4.31e4` |
| `PANDORA_MASS` | `f64` | `1.371e17` |
| `PANDORA_RADIUS` | `f64` | `4.06e4` |
| `ATLAS_MASS` | `f64` | `6.6e15` |
| `ATLAS_RADIUS` | `f64` | `1.53e4` |
| `PAN_MASS` | `f64` | `4.95e15` |
| `PAN_RADIUS` | `f64` | `1.42e4` |
| `EPIMETHEUS_MASS` | `f64` | `5.266e17` |
| `EPIMETHEUS_RADIUS` | `f64` | `5.83e4` |
| `JANUS_MASS` | `f64` | `1.898e18` |
| `JANUS_RADIUS` | `f64` | `8.95e4` |
| `AMALTHEA_MASS` | `f64` | `2.08e18` |
| `AMALTHEA_RADIUS` | `f64` | `8.35e4` |
| `THEBE_MASS` | `f64` | `4.3e17` |
| `THEBE_RADIUS` | `f64` | `4.93e4` |
| `HIMALIA_MASS` | `f64` | `4.2e18` |
| `HIMALIA_RADIUS` | `f64` | `8.5e4` |
| `METIS_MASS` | `f64` | `3.6e16` |
| `METIS_RADIUS` | `f64` | `2.15e4` |
| `ADRASTEA_MASS` | `f64` | `2.0e15` |
| `ADRASTEA_RADIUS` | `f64` | `8.2e3` |
| `J2_EARTH` | `f64` | `1.082_63e-3` |
| `J2_JUPITER` | `f64` | `1.473_6e-2` |
| `J2_SATURN` | `f64` | `1.629_8e-2` |
| `J2_MARS` | `f64` | `1.960_45e-3` |
| `J2_SUN` | `f64` | `2.0e-7` |
| `MERCURY_ORBITAL_VELOCITY` | `f64` | `47_362.0` |
| `VENUS_ORBITAL_VELOCITY` | `f64` | `35_020.0` |
| `EARTH_ORBITAL_VELOCITY` | `f64` | `29_783.0` |
| `MARS_ORBITAL_VELOCITY` | `f64` | `24_007.0` |
| `JUPITER_ORBITAL_VELOCITY` | `f64` | `13_070.0` |
| `SATURN_ORBITAL_VELOCITY` | `f64` | `9680.0` |
| `URANUS_ORBITAL_VELOCITY` | `f64` | `6810.0` |
| `NEPTUNE_ORBITAL_VELOCITY` | `f64` | `5430.0` |
| `SUN_CORE_TEMPERATURE` | `f64` | `1.57e7` |
| `SUN_SURFACE_TEMPERATURE` | `f64` | `5778.0` |
| `SUN_CORE_DENSITY` | `f64` | `1.622e5` |
| `SUN_AGE` | `f64` | `1.44e17` |
| `SUN_ROTATION_PERIOD` | `f64` | `2.164e6` |
| `YOSHIDA4_W0` | `f64` | `-1.702_414_383_919_315_3` |
| `YOSHIDA4_W1` | `f64` | `1.351_207_191_959_657_8` |
| `YOSHIDA4_C1` | `f64` | `YOSHIDA4_W1 / 2.0` |
| `YOSHIDA4_C2` | `f64` | `(YOSHIDA4_W0 + YOSHIDA4_W1) / 2.0` |
| `YOSHIDA4_C3` | `f64` | `YOSHIDA4_C2` |
| `YOSHIDA4_C4` | `f64` | `YOSHIDA4_C1` |
| `YOSHIDA4_D1` | `f64` | `YOSHIDA4_W1` |
| `YOSHIDA4_D2` | `f64` | `YOSHIDA4_W0` |
| `YOSHIDA4_D3` | `f64` | `YOSHIDA4_W1` |
| `C_KM_S` | `f64` | `crate::constants::C / 1e3` |
| `MPC_IN_M` | `f64` | `PARSEC * 1e6` |
| `MPC_IN_KM` | `f64` | `PARSEC * 1e3` |
| `SEC_PER_GYR` | `f64` | `crate::constants::SECONDS_PER_DAY * crate::constants::JULIAN_CENTURY * 1e7` |

---

### astronomy/darkmatter (21 items)

| Name | Type | Value |
|------|------|-------|
| `LOCAL_DM_DENSITY` | `f64` | `0.3` |
| `LOCAL_DM_DENSITY_SI` | `f64` | `5.35e-22` |
| `OMEGA_DM` | `f64` | `0.265` |
| `OMEGA_BARYON` | `f64` | `0.049` |
| `OMEGA_MATTER` | `f64` | `OMEGA_DM + OMEGA_BARYON` |
| `OMEGA_LAMBDA` | `f64` | `1.0 - OMEGA_MATTER` |
| `THERMAL_RELIC_SIGMA_V` | `f64` | `3.0e-26` |
| `WIMP_MASS_MIN_GEV` | `f64` | `10.0` |
| `WIMP_MASS_MAX_GEV` | `f64` | `1000.0` |
| `WIMP_SIGMA_UPPER` | `f64` | `1.0e-46` |
| `AXION_MASS_MIN_EV` | `f64` | `1.0e-6` |
| `AXION_MASS_MAX_EV` | `f64` | `1.0e-3` |
| `AXION_DECAY_CONSTANT` | `f64` | `1.0e12` |
| `NFW_RHO_S_TYPICAL` | `f64` | `4.88e6` |
| `NFW_RS_MW` | `f64` | `21.5` |
| `MW_VIRIAL_MASS` | `f64` | `1.3e12` |
| `MW_VIRIAL_RADIUS` | `f64` | `287.0` |
| `MW_CONCENTRATION` | `f64` | `13.3` |
| `Z_MATTER_RADIATION_EQ` | `f64` | `3402.0` |
| `Z_RECOMBINATION` | `f64` | `1089.0` |
| `AGE_UNIVERSE` | `f64` | `4.35e17` |

---

### astronomy/galactic (15 items)

| Name | Type | Value |
|------|------|-------|
| `MW_MASS` | `f64` | `1.5e12` |
| `MW_DISK_RADIUS` | `f64` | `2.63e20` |
| `MW_DISK_SCALE_HEIGHT` | `f64` | `9.26e18` |
| `MW_BULGE_RADIUS` | `f64` | `6.17e19` |
| `SUN_GALACTIC_DISTANCE` | `f64` | `2.47e20` |
| `SUN_GALACTIC_VELOCITY` | `f64` | `220_000.0` |
| `SUN_GALACTIC_PERIOD` | `f64` | `7.25e15` |
| `MW_STELLAR_MASS` | `f64` | `5.0e10` |
| `MW_STAR_COUNT` | `f64` | `2.0e11` |
| `SGR_A_STAR_MASS` | `f64` | `4.15e6` |
| `SGR_A_STAR_DISTANCE` | `f64` | `2.47e20` |
| `M31_MASS` | `f64` | `1.5e12` |
| `M31_DISTANCE` | `f64` | `2.37e22` |
| `MW_M31_RADIAL_VELOCITY` | `f64` | `110_000.0` |
| `HUBBLE_FLOW_VELOCITY_PER_MPC` | `f64` | `crate::constants::HUBBLE_CONSTANT * 1e3` |

---

### astronomy/stellar (22 items)

| Name | Type | Value |
|------|------|-------|
| `CHANDRASEKHAR_LIMIT` | `f64` | `crate::constants::CHANDRASEKHAR_LIMIT_SOLAR * crate::constants::SOLAR_MASS` |
| `TOV_LIMIT` | `f64` | `4.58e30` |
| `EDDINGTON_PREFACTOR` | `f64` | `3.2e4` |
| `SOLAR_TEFF` | `f64` | `crate::constants::SUN_SURFACE_TEMPERATURE` |
| `SOLAR_ABS_MAGNITUDE` | `f64` | `4.83` |
| `SOLAR_METALLICITY` | `f64` | `0.013_4` |
| `PP_CHAIN_ENERGY` | `f64` | `4.20e-12` |
| `CNO_CYCLE_ENERGY` | `f64` | `3.98e-12` |
| `H_BURNING_EFFICIENCY` | `f64` | `0.007` |
| `SOLAR_MS_LIFETIME` | `f64` | `3.15e17` |
| `WD_RADIUS_TYPICAL` | `f64` | `5.5e6` |
| `NS_RADIUS_TYPICAL` | `f64` | `1.1e4` |
| `NS_MASS_TYPICAL` | `f64` | `2.78e30` |
| `PULSAR_B_TYPICAL` | `f64` | `1.0e8` |
| `MAGNETAR_B_TYPICAL` | `f64` | `1.0e11` |
| `HYDROGEN_FRACTION_SOLAR` | `f64` | `0.734_6` |
| `HELIUM_FRACTION_SOLAR` | `f64` | `0.248_5` |
| `METAL_FRACTION_SOLAR` | `f64` | `SOLAR_METALLICITY` |
| `HELIUM4_MASS` | `f64` | `6.644_657_3e-27` |
| `ADIABATIC_INDEX_IDEAL` | `f64` | `5.0 / 3.0` |
| `ADIABATIC_INDEX_RADIATION` | `f64` | `4.0 / 3.0` |
| `OPACITY_ELECTRON_SCATTERING` | `f64` | `0.2` |

---

### biology/bioenergetics (12 items)

| Name | Type | Value |
|------|------|-------|
| `KLEIBER_CONSTANT` | `f64` | `70.0` |
| `KLEIBER_EXPONENT` | `f64` | `0.75` |
| `ATP_YIELD_AEROBIC` | `f64` | `30.0` |
| `ATP_YIELD_ANAEROBIC` | `f64` | `2.0` |
| `CAC_NADH_PER_ACETYL_COA` | `f64` | `3.0` |
| `CAC_FADH2_PER_ACETYL_COA` | `f64` | `1.0` |
| `BETA_OX_FADH2_ATP` | `f64` | `1.5` |
| `BETA_OX_NADH_ATP` | `f64` | `2.5` |
| `BETA_OX_ACETYL_COA_ATP` | `f64` | `10.0` |
| `BETA_OX_ACTIVATION_COST` | `f64` | `2.0` |
| `BODY_TEMP_KELVIN` | `f64` | `310.15` |
| `REFERENCE_TEMP_KELVIN` | `f64` | `298.15` |

---

### biology/neuroscience (62 items)

| Name | Type | Value |
|------|------|-------|
| `HH_RESTING_POTENTIAL` | `f64` | `-65.0` |
| `HH_INIT_M` | `f64` | `0.05` |
| `HH_INIT_H` | `f64` | `0.6` |
| `HH_INIT_N` | `f64` | `0.32` |
| `HH_MEMBRANE_CAPACITANCE` | `f64` | `1.0` |
| `HH_G_NA` | `f64` | `120.0` |
| `HH_G_K` | `f64` | `36.0` |
| `HH_G_L` | `f64` | `0.3` |
| `HH_E_NA` | `f64` | `50.0` |
| `HH_E_K` | `f64` | `-77.0` |
| `HH_E_L` | `f64` | `-54.4` |
| `HH_ALPHA_M_V_SHIFT` | `f64` | `40.0` |
| `HH_ALPHA_M_COEFF` | `f64` | `0.1` |
| `HH_BETA_M_MULTIPLIER` | `f64` | `4.0` |
| `HH_BETA_M_EXP_COEFF` | `f64` | `0.0556` |
| `HH_GATING_V_SHIFT` | `f64` | `65.0` |
| `HH_ALPHA_H_COEFF` | `f64` | `0.07` |
| `HH_ALPHA_H_EXP_COEFF` | `f64` | `0.05` |
| `HH_BETA_H_EXP_COEFF` | `f64` | `0.1` |
| `HH_BETA_H_V_SHIFT` | `f64` | `35.0` |
| `HH_ALPHA_N_V_SHIFT` | `f64` | `55.0` |
| `HH_ALPHA_N_COEFF` | `f64` | `0.01` |
| `HH_BETA_N_COEFF` | `f64` | `0.125` |
| `HH_BETA_N_EXP_COEFF` | `f64` | `0.0125` |
| `IZHI_RESTING_POTENTIAL` | `f64` | `-65.0` |
| `IZHI_INIT_U` | `f64` | `-14.0` |
| `IZHI_QUAD_COEFF` | `f64` | `0.04` |
| `IZHI_LINEAR_COEFF` | `f64` | `5.0` |
| `IZHI_CONSTANT_COEFF` | `f64` | `140.0` |
| `IZHI_SPIKE_THRESHOLD` | `f64` | `30.0` |
| `IZHI_RS_A` | `f64` | `0.02` |
| `IZHI_RS_B` | `f64` | `0.2` |
| `IZHI_RS_C` | `f64` | `-65.0` |
| `IZHI_RS_D` | `f64` | `8.0` |
| `IZHI_FS_A` | `f64` | `0.1` |
| `IZHI_FS_B` | `f64` | `0.2` |
| `IZHI_FS_C` | `f64` | `-65.0` |
| `IZHI_FS_D` | `f64` | `2.0` |
| `IZHI_BURST_A` | `f64` | `0.02` |
| `IZHI_BURST_B` | `f64` | `0.2` |
| `IZHI_BURST_C` | `f64` | `-50.0` |
| `IZHI_BURST_D` | `f64` | `2.0` |
| `ML_INIT_V` | `f64` | `-60.0` |
| `ML_CM` | `f64` | `20.0` |
| `ML_G_CA` | `f64` | `4.4` |
| `ML_G_K` | `f64` | `8.0` |
| `ML_G_L` | `f64` | `2.0` |
| `ML_E_CA` | `f64` | `120.0` |
| `ML_E_K` | `f64` | `-84.0` |
| `ML_E_L` | `f64` | `-60.0` |
| `ML_V1` | `f64` | `-1.2` |
| `ML_V2` | `f64` | `18.0` |
| `ML_V3` | `f64` | `2.0` |
| `ML_V4` | `f64` | `30.0` |
| `ML_PHI` | `f64` | `0.04` |
| `LIF_RESTING_POTENTIAL` | `f64` | `-65.0` |
| `LIF_THRESHOLD` | `f64` | `-50.0` |
| `LIF_TAU` | `f64` | `10.0` |
| `LIF_RESISTANCE` | `f64` | `10.0` |
| `NMDA_MG_KD` | `f64` | `3.57` |
| `NMDA_VOLTAGE_COEFF` | `f64` | `0.062` |
| `SYNAPTIC_CA_HALF_SAT` | `f64` | `0.5` |

---

### biology/physiology (22 items)

| Name | Type | Value |
|------|------|-------|
| `WINDCHILL_CONSTANT` | `f64` | `13.12` |
| `WINDCHILL_TEMP_COEFF` | `f64` | `0.6215` |
| `WINDCHILL_WIND_COEFF` | `f64` | `11.37` |
| `WINDCHILL_WIND_EXP` | `f64` | `0.16` |
| `WINDCHILL_INTERACTION` | `f64` | `0.3965` |
| `HEAT_INDEX_CONSTANT` | `f64` | `-42.379` |
| `HEAT_INDEX_TEMP_COEFF` | `f64` | `2.049_015_23` |
| `HEAT_INDEX_HUMIDITY_COEFF` | `f64` | `10.143_331_27` |
| `HEAT_INDEX_INTERACTION` | `f64` | `-0.224_755_41` |
| `HEAT_INDEX_TEMP2` | `f64` | `-6.837_83e-3` |
| `HEAT_INDEX_HUMIDITY2` | `f64` | `-5.481_717e-2` |
| `HEAT_INDEX_T2H` | `f64` | `1.228_74e-3` |
| `HEAT_INDEX_TH2` | `f64` | `8.528_2e-4` |
| `HEAT_INDEX_T2H2` | `f64` | `-1.99e-6` |
| `BSA_DUBOIS_COEFF` | `f64` | `0.007_184` |
| `BSA_WEIGHT_EXP` | `f64` | `0.425` |
| `BSA_HEIGHT_EXP` | `f64` | `0.725` |
| `HB_O2_CAPACITY` | `f64` | `1.34` |
| `DISSOLVED_O2_COEFF` | `f64` | `0.003` |
| `O2_DELIVERY_SCALING` | `f64` | `10.0` |
| `MMHG_TO_PA` | `f64` | `133.322` |
| `WINDKESSEL_SYSTOLIC_FRACTION` | `f64` | `0.35` |

---

### biology/radiobiology (10 items)

| Name | Type | Value |
|------|------|-------|
| `DSB_YIELD_PER_GY` | `f64` | `35.0` |
| `SSB_YIELD_PER_GY` | `f64` | `1000.0` |
| `OXIDATIVE_DAMAGE_COEFF` | `f64` | `200.0` |
| `OXIDATIVE_DAMAGE_O2_KM` | `f64` | `3.0` |
| `DNA_DAMAGE_DSB_WEIGHT` | `f64` | `10.0` |
| `DNA_DAMAGE_BASE_WEIGHT` | `f64` | `2.0` |
| `LN_2` | `f64` | `std::f64::consts::LN_2` |
| `FARQUHAR_WJ_CI_COEFF` | `f64` | `4.0` |
| `FARQUHAR_WJ_GAMMA_COEFF` | `f64` | `8.0` |
| `TUMOR_CARRYING_CAPACITY` | `f64` | `1e12` |

---

### geology/impact (7 items)

| Name | Type | Value |
|------|------|-------|
| `CRATER_SCALING_COEFF` | `f64` | `1.161` |
| `CRATER_DENSITY_EXPONENT` | `f64` | `1.0 / 3.0` |
| `CRATER_PROJECTILE_EXPONENT` | `f64` | `0.78` |
| `CRATER_VELOCITY_EXPONENT` | `f64` | `0.44` |
| `CRATER_GRAVITY_EXPONENT` | `f64` | `-0.22` |
| `FIREBALL_RADIUS_COEFF` | `f64` | `55.0` |
| `FIREBALL_ENERGY_EXPONENT` | `f64` | `0.4` |

---

### geology/radioactive (10 items)

| Name | Type | Value |
|------|------|-------|
| `LAMBDA_U238` | `f64` | `1.551_25e-10` |
| `LAMBDA_U235` | `f64` | `9.8485e-10` |
| `LAMBDA_TH232` | `f64` | `4.947_5e-11` |
| `LAMBDA_K40_TOTAL` | `f64` | `5.543e-10` |
| `LAMBDA_K40_AR` | `f64` | `LAMBDA_K40_TOTAL * K40_BRANCH_RATIO_AR` |
| `K40_BRANCH_RATIO_AR` | `f64` | `0.1048` |
| `C14_MEAN_LIFE` | `f64` | `8_267.0` |
| `HEAT_PRODUCTION_U238` | `f64` | `9.46e-5` |
| `HEAT_PRODUCTION_TH232` | `f64` | `2.64e-5` |
| `HEAT_PRODUCTION_K40` | `f64` | `2.92e-5` |

---

### geology/seawater (39 items)

| Name | Type | Value |
|------|------|-------|
| `SEAWATER_RHO_0` | `f64` | `999.842_594` |
| `SEAWATER_RHO_T1` | `f64` | `6.793_952e-2` |
| `SEAWATER_RHO_T2` | `f64` | `-9.095_290e-3` |
| `SEAWATER_RHO_T3` | `f64` | `1.001_685e-4` |
| `SEAWATER_RHO_T4` | `f64` | `-1.120_083e-6` |
| `SEAWATER_RHO_T5` | `f64` | `6.536_332e-9` |
| `SEAWATER_RHO_S0` | `f64` | `8.244_93e-1` |
| `SEAWATER_RHO_S_T1` | `f64` | `-4.089_9e-3` |
| `SEAWATER_RHO_S_T2` | `f64` | `7.643_8e-5` |
| `SEAWATER_RHO_S_T3` | `f64` | `-8.246_7e-7` |
| `SEAWATER_RHO_S_T4` | `f64` | `5.387_5e-9` |
| `SEAWATER_RHO_B0` | `f64` | `-5.724_66e-3` |
| `SEAWATER_RHO_B_T1` | `f64` | `1.022_7e-4` |
| `SEAWATER_RHO_B_T2` | `f64` | `-1.654_6e-6` |
| `SEAWATER_RHO_C0` | `f64` | `4.831_4e-4` |
| `SOUND_SPEED_BASE` | `f64` | `1448.96` |
| `SOUND_SPEED_T1` | `f64` | `4.591` |
| `SOUND_SPEED_T2` | `f64` | `-5.304e-2` |
| `SOUND_SPEED_T3` | `f64` | `2.374e-4` |
| `SOUND_SPEED_S1` | `f64` | `1.340` |
| `SOUND_SPEED_D1` | `f64` | `1.630e-2` |
| `SOUND_SPEED_D2` | `f64` | `1.675e-7` |
| `SOUND_SPEED_TS` | `f64` | `-1.025e-2` |
| `SOUND_SPEED_TD` | `f64` | `-7.139e-13` |
| `PHILLIPS_ALPHA` | `f64` | `0.0081` |
| `PHILLIPS_BETA` | `f64` | `0.74` |
| `JONSWAP_ALPHA_COEFF` | `f64` | `0.076` |
| `JONSWAP_ALPHA_EXPONENT` | `f64` | `0.22` |
| `JONSWAP_FREQ_COEFF` | `f64` | `22.0` |
| `JONSWAP_GAMMA` | `f64` | `3.3` |
| `JONSWAP_SIGMA_A` | `f64` | `0.07` |
| `JONSWAP_SIGMA_B` | `f64` | `0.09` |
| `WAVE_HEIGHT_COEFF` | `f64` | `0.0016` |
| `WAVE_HEIGHT_EXPONENT` | `f64` | `0.5` |
| `WAVE_PERIOD_COEFF` | `f64` | `0.286` |
| `HALINE_CONTRACTION_0` | `f64` | `7.5e-4` |
| `HALINE_CONTRACTION_1` | `f64` | `2.0e-6` |
| `THERMAL_EXPANSION_0` | `f64` | `1.67e-4` |
| `THERMAL_EXPANSION_1` | `f64` | `1.0e-5` |

---

### maths/approximation (16 items)

| Name | Type | Value |
|------|------|-------|
| `ERF_A1` | `f64` | `0.254_829_592` |
| `ERF_A2` | `f64` | `-0.284_496_736` |
| `ERF_A3` | `f64` | `1.421_413_741` |
| `ERF_A4` | `f64` | `-1.453_152_027` |
| `ERF_A5` | `f64` | `1.061_405_429` |
| `ERF_P` | `f64` | `0.327_591_1` |
| `LANCZOS_COEFFS_6` | `[f64` | `[f64` |
| `LANCZOS_SERIES_INIT` | `f64` | `1.000_000_000_190_015` |
| `LANCZOS_SQRT_2PI` | `f64` | `2.506_628_274_631_000_5` |
| `STIRLING_LANCZOS_G` | `f64` | `7.0` |
| `STIRLING_LANCZOS_COEFFS` | `[f64` | `[f64` |
| `GOLDEN_RATIO_CONJUGATE` | `f64` | `0.618_033_988_749_894_9` |
| `SAVITZKY_GOLAY_5_COEFFS` | `[f64` | `[f64` |
| `SAVITZKY_GOLAY_5_NORM` | `f64` | `35.0` |
| `SPEED_OF_LIGHT_KM_S` | `f64` | `crate::constants::C_KM_S` |
| `CMB_RECOMBINATION_REDSHIFT` | `f64` | `crate::constants::Z_RECOMBINATION` |

---

### maths/ode (30 items)

| Name | Type | Value |
|------|------|-------|
| `RK45_A2` | `f64` | `1.0 / 4.0` |
| `RK45_A3` | `f64` | `3.0 / 8.0` |
| `RK45_A4` | `f64` | `12.0 / 13.0` |
| `RK45_A5` | `f64` | `1.0` |
| `RK45_A6` | `f64` | `0.5` |
| `RK45_B21` | `f64` | `1.0 / 4.0` |
| `RK45_B31` | `f64` | `3.0 / 32.0` |
| `RK45_B32` | `f64` | `9.0 / 32.0` |
| `RK45_B41` | `f64` | `1932.0 / 2197.0` |
| `RK45_B42` | `f64` | `-7200.0 / 2197.0` |
| `RK45_B43` | `f64` | `7296.0 / 2197.0` |
| `RK45_B51` | `f64` | `439.0 / 216.0` |
| `RK45_B52` | `f64` | `-8.0` |
| `RK45_B53` | `f64` | `3680.0 / 513.0` |
| `RK45_B54` | `f64` | `-845.0 / 4104.0` |
| `RK45_B61` | `f64` | `-8.0 / 27.0` |
| `RK45_B62` | `f64` | `2.0` |
| `RK45_B63` | `f64` | `-3544.0 / 2565.0` |
| `RK45_B64` | `f64` | `1859.0 / 4104.0` |
| `RK45_B65` | `f64` | `-11.0 / 40.0` |
| `RK45_C1` | `f64` | `16.0 / 135.0` |
| `RK45_C3` | `f64` | `6656.0 / 12825.0` |
| `RK45_C4` | `f64` | `28561.0 / 56430.0` |
| `RK45_C5` | `f64` | `-9.0 / 50.0` |
| `RK45_C6` | `f64` | `2.0 / 55.0` |
| `RK45_D1` | `f64` | `25.0 / 216.0` |
| `RK45_D3` | `f64` | `1408.0 / 2565.0` |
| `RK45_D4` | `f64` | `2197.0 / 4104.0` |
| `RK45_D5` | `f64` | `-1.0 / 5.0` |
| `ADAMS_BASHFORTH_4` | `[f64` | `[f64` |

---

### maths/quadrature (11 items)

| Name | Type | Value |
|------|------|-------|
| `GAUSS_LEGENDRE_5_NODES` | `[f64` | `[f64` |
| `GAUSS_LEGENDRE_5_WEIGHTS` | `[f64` | `[f64` |
| `GAUSS_LAGUERRE_5_NODES` | `[f64` | `[f64` |
| `GAUSS_LAGUERRE_5_WEIGHTS` | `[f64` | `[f64` |
| `GAUSS_HERMITE_5_NODES` | `[f64` | `[f64` |
| `GAUSS_HERMITE_5_WEIGHTS` | `[f64` | `[f64` |
| `LOBATTO_5_NODES` | `[f64` | `[f64` |
| `LOBATTO_5_WEIGHTS` | `[f64` | `[f64` |
| `GAUSS_KRONROD_15_NODES` | `[f64` | `[f64` |
| `GAUSS_KRONROD_15_WEIGHTS` | `[f64` | `[f64` |
| `GAUSS_7_WEIGHTS` | `[f64` | `[f64` |

---

### meteorology/atmospheric (18 items)

| Name | Type | Value |
|------|------|-------|
| `SPECIFIC_GAS_CONSTANT_DRY_AIR` | `f64` | `287.058` |
| `SPECIFIC_GAS_CONSTANT_WATER_VAPOR` | `f64` | `461.5` |
| `SPECIFIC_HEAT_DRY_AIR` | `f64` | `1003.5` |
| `DRY_ADIABATIC_LAPSE_RATE` | `f64` | `crate::constants::EARTH_GRAVITY / SPECIFIC_HEAT_DRY_AIR` |
| `VIRTUAL_TEMP_FACTOR` | `f64` | `0.608` |
| `MIXING_RATIO_FACTOR` | `f64` | `0.622` |
| `MAGNUS_A` | `f64` | `17.67` |
| `MAGNUS_B` | `f64` | `243.5` |
| `SATURATION_VAPOR_PRESSURE_0C` | `f64` | `6.112` |
| `DENSITY_ALTITUDE_SCALE` | `f64` | `120.0` |
| `ISA_SEA_LEVEL_TEMP_C` | `f64` | `15.0` |
| `ISA_LAPSE_RATE_PER_M` | `f64` | `0.002` |
| `VON_KARMAN` | `f64` | `0.41` |
| `STANDARD_AIR_DENSITY_SEA_LEVEL` | `f64` | `1.225` |
| `RADIATIVE_FORCING_CO2_COEFF` | `f64` | `5.35` |
| `SOLAR_CONSTANT` | `f64` | `1361.0` |
| `BEAUFORT_TO_MS_COEFF` | `f64` | `0.836` |
| `BEAUFORT_TO_MS_EXPONENT` | `f64` | `1.5` |

---

### physics/atomic (18 items)

| Name | Type | Value |
|------|------|-------|
| `PROTON_MASS_KG` | `f64` | `1.672_621_923_69e-27` |
| `NEUTRON_MASS_KG` | `f64` | `1.674_927_498_04e-27` |
| `ELECTRON_MASS_KG` | `f64` | `9.109_383_701_5e-31` |
| `PROTON_MASS_AMU` | `f64` | `1.007_276_466_621` |
| `NEUTRON_MASS_AMU` | `f64` | `1.008_664_915_95` |
| `ELECTRON_MASS_AMU` | `f64` | `0.000_548_579_909_065` |
| `AMU_TO_KG` | `f64` | `1.660_539_066_60e-27` |
| `AMU_TO_MEV` | `f64` | `931.494_102_42` |
| `ELECTRON_REST_MASS_MEV` | `f64` | `ELECTRON_MASS_KG * super::fundamental::C * super::fundamental::C / (super::units::EV_TO_JOULE * 1e6)` |
| `BOHR_RADIUS` | `f64` | `5.291_772_109_03e-11` |
| `RYDBERG_ENERGY` | `f64` | `13.605_693_122_994` |
| `BOHR_MAGNETON` | `f64` | `9.274_010_078_3e-24` |
| `NUCLEAR_MAGNETON` | `f64` | `5.050_783_746_1e-27` |
| `COMPTON_WAVELENGTH` | `f64` | `super::fundamental::H / (ELECTRON_MASS_KG * super::fundamental::C)` |
| `MUON_MASS` | `f64` | `1.883_531_627e-28` |
| `TAU_MASS` | `f64` | `3.167_47e-27` |
| `NEUTRINO_MASS_UPPER` | `f64` | `2.2e-36` |
| `R_INF` | `f64` | `10_973_731.568_16` |

---

### physics/elements (4 items)

| Name | Type | Value |
|------|------|-------|
| `by_atomic_number` | fn | `(z: u32) -> Option<&'static Element>` |
| `by_symbol` | fn | `(sym: &str) -> Option<&'static Element>` |
| `atomic_mass` | fn | `(z: u32) -> f64` |
| `electronegativity` | fn | `(z: u32) -> Option<f64>` |

---

### physics/fundamental (20 items)

| Name | Type | Value |
|------|------|-------|
| `C` | `f64` | `299_792_458.0` |
| `G` | `f64` | `6.674_30e-11` |
| `H` | `f64` | `6.626_070_15e-34` |
| `HBAR` | `f64` | `H / (2.0 * std::f64::consts::PI)` |
| `K_B` | `f64` | `1.380_649e-23` |
| `N_A` | `f64` | `6.022_140_76e23` |
| `E_CHARGE` | `f64` | `1.602_176_634e-19` |
| `EPSILON_0` | `f64` | `8.854_187_812_8e-12` |
| `MU_0` | `f64` | `1.0 / (EPSILON_0 * C * C)` |
| `K_COULOMB` | `f64` | `1.0 / (4.0 * std::f64::consts::PI * EPSILON_0)` |
| `SIGMA_SB` | `f64` | `5.670_374_419e-8` |
| `R_GAS` | `f64` | `K_B * N_A` |
| `FARADAY` | `f64` | `N_A * E_CHARGE` |
| `ALPHA_FINE` | `f64` | `7.297_352_569_3e-3` |
| `PLANCK_LENGTH` | `f64` | `1.616_255e-35` |
| `PLANCK_MASS` | `f64` | `2.176_434e-8` |
| `PLANCK_TIME` | `f64` | `5.391_247e-44` |
| `PLANCK_TEMP` | `f64` | `1.416_784e32` |
| `G_F` | `f64` | `1.166_378_7e-5` |
| `ALPHA_S` | `f64` | `0.1181` |

---

### physics/units (20 items)

| Name | Type | Value |
|------|------|-------|
| `EV_TO_JOULE` | `f64` | `super::fundamental::E_CHARGE` |
| `JOULE_TO_EV` | `f64` | `1.0 / EV_TO_JOULE` |
| `HC_EV_NM` | `f64` | `super::fundamental::H * super::fundamental::C / EV_TO_JOULE * 1e9` |
| `KEV_TO_KELVIN` | `f64` | `1e3 / KELVIN_TO_KEV` |
| `KELVIN_TO_KEV` | `f64` | `super::fundamental::K_B / super::fundamental::E_CHARGE` |
| `CALORIE_TO_JOULE` | `f64` | `4.184` |
| `ATM_TO_PASCAL` | `f64` | `101_325.0` |
| `BAR_TO_PASCAL` | `f64` | `1.0e5` |
| `DEGREE_TO_RAD` | `f64` | `std::f64::consts::PI / 180.0` |
| `RAD_TO_DEGREE` | `f64` | `180.0 / std::f64::consts::PI` |
| `BARN` | `f64` | `1.0e-28` |
| `ANGSTROM` | `f64` | `1.0e-10` |
| `FERMI` | `f64` | `1.0e-15` |
| `KWH_TO_J` | `f64` | `3.6e6` |
| `MEV_TO_J` | `f64` | `EV_TO_JOULE * 1e6` |
| `SECONDS_PER_DAY` | `f64` | `86_400.0` |
| `J2000_EPOCH_JD` | `f64` | `2_451_545.0` |
| `JULIAN_CENTURY` | `f64` | `36_525.0` |
| `SIDEREAL_YEAR` | `f64` | `3.155_815e7` |
| `TROPICAL_YEAR` | `f64` | `3.155_693e7` |

---

