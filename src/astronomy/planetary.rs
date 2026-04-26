use crate::constants::{
    EARTH_MASS, EARTH_ORBITAL_VELOCITY, EARTH_RADIUS, G, JUPITER_AXIAL_TILT, JUPITER_BOND_ALBEDO,
    JUPITER_ECCENTRICITY, JUPITER_ESCAPE_VELOCITY, JUPITER_FLATTENING, JUPITER_INCLINATION,
    JUPITER_MASS, JUPITER_MEAN_DENSITY, JUPITER_ORBITAL_PERIOD, JUPITER_ORBITAL_VELOCITY,
    JUPITER_RADIUS, JUPITER_SEMI_MAJOR_AXIS, JUPITER_SIDEREAL_DAY, JUPITER_SURFACE_GRAVITY, K_B,
    MARS_AXIAL_TILT, MARS_BOND_ALBEDO, MARS_ECCENTRICITY, MARS_ESCAPE_VELOCITY, MARS_FLATTENING,
    MARS_INCLINATION, MARS_MASS, MARS_MEAN_DENSITY, MARS_ORBITAL_PERIOD, MARS_ORBITAL_VELOCITY,
    MARS_RADIUS, MARS_SEMI_MAJOR_AXIS, MARS_SIDEREAL_DAY, MARS_SURFACE_GRAVITY, MERCURY_AXIAL_TILT,
    MERCURY_BOND_ALBEDO, MERCURY_ECCENTRICITY, MERCURY_ESCAPE_VELOCITY, MERCURY_FLATTENING,
    MERCURY_INCLINATION, MERCURY_MASS, MERCURY_MEAN_DENSITY, MERCURY_ORBITAL_PERIOD,
    MERCURY_ORBITAL_VELOCITY, MERCURY_RADIUS, MERCURY_SEMI_MAJOR_AXIS, MERCURY_SIDEREAL_DAY,
    MERCURY_SURFACE_GRAVITY, MU_0, NEPTUNE_AXIAL_TILT, NEPTUNE_BOND_ALBEDO, NEPTUNE_ECCENTRICITY,
    NEPTUNE_ESCAPE_VELOCITY, NEPTUNE_FLATTENING, NEPTUNE_INCLINATION, NEPTUNE_MASS,
    NEPTUNE_MEAN_DENSITY, NEPTUNE_ORBITAL_PERIOD, NEPTUNE_ORBITAL_VELOCITY, NEPTUNE_RADIUS,
    NEPTUNE_SEMI_MAJOR_AXIS, NEPTUNE_SIDEREAL_DAY, NEPTUNE_SURFACE_GRAVITY, SATURN_AXIAL_TILT,
    SATURN_BOND_ALBEDO, SATURN_ECCENTRICITY, SATURN_ESCAPE_VELOCITY, SATURN_FLATTENING,
    SATURN_INCLINATION, SATURN_MASS, SATURN_MEAN_DENSITY, SATURN_ORBITAL_PERIOD,
    SATURN_ORBITAL_VELOCITY, SATURN_RADIUS, SATURN_SEMI_MAJOR_AXIS, SATURN_SIDEREAL_DAY,
    SATURN_SURFACE_GRAVITY, SIGMA_SB, URANUS_AXIAL_TILT, URANUS_BOND_ALBEDO, URANUS_ECCENTRICITY,
    URANUS_ESCAPE_VELOCITY, URANUS_FLATTENING, URANUS_INCLINATION, URANUS_MASS,
    URANUS_MEAN_DENSITY, URANUS_ORBITAL_PERIOD, URANUS_ORBITAL_VELOCITY, URANUS_RADIUS,
    URANUS_SEMI_MAJOR_AXIS, URANUS_SIDEREAL_DAY, URANUS_SURFACE_GRAVITY, VENUS_AXIAL_TILT,
    VENUS_BOND_ALBEDO, VENUS_ECCENTRICITY, VENUS_ESCAPE_VELOCITY, VENUS_FLATTENING,
    VENUS_INCLINATION, VENUS_MASS, VENUS_MEAN_DENSITY, VENUS_ORBITAL_PERIOD,
    VENUS_ORBITAL_VELOCITY, VENUS_RADIUS, VENUS_SEMI_MAJOR_AXIS, VENUS_SIDEREAL_DAY,
    VENUS_SURFACE_GRAVITY,
};

pub fn hydrostatic_pressure(density: f64, g_surface: f64, depth: f64) -> f64 {
    density * g_surface * depth
}

pub fn central_pressure(density: f64, g_surface: f64, radius: f64) -> f64 {
    3.0 * g_surface * density * radius / 2.0
}

pub fn adiabatic_temperature_gradient(g_local: f64, specific_heat: f64) -> f64 {
    g_local / specific_heat
}

pub fn planetary_moment_of_inertia_factor(
    core_density: f64,
    mantle_density: f64,
    core_radius: f64,
    total_radius: f64,
) -> f64 {
    let rc5 = core_radius.powi(5);
    let rt5 = total_radius.powi(5);
    let rc3 = core_radius.powi(3);
    let rt3 = total_radius.powi(3);
    let numerator = core_density * rc5 + mantle_density * (rt5 - rc5);
    let denominator = core_density * rc3 + mantle_density * (rt3 - rc3);
    0.4 * numerator / (denominator * total_radius.powi(2) / rt3)
}

pub fn love_number_k2(rigidity: f64, density: f64, radius: f64) -> f64 {
    1.5 / (1.0 + 19.0 * rigidity / (2.0 * G * density.powi(2) * radius.powi(2) / 3.0))
}

pub fn tidal_heating(
    radius: f64,
    eccentricity: f64,
    mean_motion: f64,
    k2: f64,
    tidal_q: f64,
    perturber_mass: f64,
    semi_major_axis: f64,
) -> f64 {
    21.0 / 2.0 * k2 / tidal_q * G * perturber_mass.powi(2) * radius.powi(5) * mean_motion
        / semi_major_axis.powi(6)
        * eccentricity.powi(2)
}

pub fn tidal_locking_timescale(
    mass: f64,
    radius: f64,
    semi_major_axis: f64,
    perturber_mass: f64,
    tidal_q: f64,
    rigidity: f64,
    initial_spin: f64,
) -> f64 {
    let k2 = love_number_k2(
        rigidity,
        mass / (4.0 / 3.0 * std::f64::consts::PI * radius.powi(3)),
        radius,
    );
    let i = 0.4 * mass * radius.powi(2);
    i * tidal_q * semi_major_axis.powi(6) * initial_spin
        / (3.0 * k2 * G * perturber_mass.powi(2) * radius.powi(3))
}

pub fn roche_limit_fluid(primary_radius: f64, primary_density: f64, secondary_density: f64) -> f64 {
    2.456 * primary_radius * (primary_density / secondary_density).powf(1.0 / 3.0)
}

pub fn roche_limit_rigid(primary_radius: f64, primary_density: f64, secondary_density: f64) -> f64 {
    1.26 * primary_radius * (primary_density / secondary_density).powf(1.0 / 3.0)
}

pub fn equilibrium_temperature(stellar_luminosity: f64, semi_major_axis: f64, albedo: f64) -> f64 {
    (stellar_luminosity * (1.0 - albedo)
        / (16.0 * std::f64::consts::PI * SIGMA_SB * semi_major_axis.powi(2)))
    .powf(0.25)
}

pub fn greenhouse_surface_temperature(equilibrium_temp: f64, optical_depth_ir: f64) -> f64 {
    equilibrium_temp * (1.0 + 0.75 * optical_depth_ir).powf(0.25)
}

pub fn scale_height(temperature: f64, mean_molecular_mass: f64, g_surface: f64) -> f64 {
    K_B * temperature / (mean_molecular_mass * g_surface)
}

pub fn atmospheric_escape_jeans(
    temperature: f64,
    molecular_mass: f64,
    escape_velocity: f64,
) -> f64 {
    let v_thermal = (2.0 * K_B * temperature / molecular_mass).sqrt();
    let lambda = (escape_velocity / v_thermal).powi(2);
    (1.0 + lambda) * (-lambda).exp()
}

pub fn magnetopause_standoff(dipole_moment: f64, solar_wind_pressure: f64) -> f64 {
    (MU_0 * dipole_moment.powi(2) / (8.0 * std::f64::consts::PI.powi(2) * solar_wind_pressure))
        .powf(1.0 / 6.0)
}

pub fn ring_roche_inner(planet_mass: f64, planet_radius: f64, particle_density: f64) -> f64 {
    let planet_density = planet_mass / (4.0 / 3.0 * std::f64::consts::PI * planet_radius.powi(3));
    roche_limit_fluid(planet_radius, planet_density, particle_density)
}

pub fn ring_optical_depth(
    surface_density: f64,
    particle_radius: f64,
    particle_density: f64,
) -> f64 {
    3.0 * surface_density / (4.0 * particle_density * particle_radius)
}

pub fn synchronous_orbit_radius(mass: f64, rotation_period: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI / rotation_period;
    (G * mass / omega.powi(2)).powf(1.0 / 3.0)
}

pub fn oblateness(rotation_rate: f64, equatorial_radius: f64, mass: f64) -> f64 {
    let q = rotation_rate.powi(2) * equatorial_radius.powi(3) / (G * mass);
    q / 2.0
}

pub fn precession_rate(
    oblateness_j2: f64,
    planet_radius: f64,
    semi_major_axis: f64,
    mean_motion: f64,
) -> f64 {
    -1.5 * mean_motion * oblateness_j2 * (planet_radius / semi_major_axis).powi(2)
}

pub fn bond_albedo_from_geometric(geometric_albedo: f64, phase_integral: f64) -> f64 {
    geometric_albedo * phase_integral
}

pub fn thermal_inertia(thermal_conductivity: f64, density: f64, specific_heat: f64) -> f64 {
    (thermal_conductivity * density * specific_heat).sqrt()
}

pub fn diurnal_skin_depth(thermal_diffusivity: f64, rotation_period: f64) -> f64 {
    (thermal_diffusivity * rotation_period / std::f64::consts::PI).sqrt()
}

pub fn subsolar_temperature(stellar_flux: f64, albedo: f64, emissivity: f64) -> f64 {
    (stellar_flux * (1.0 - albedo) / (emissivity * SIGMA_SB)).powf(0.25)
}

pub fn nightside_temperature(
    thermal_inertia_val: f64,
    subsolar_temp: f64,
    rotation_period: f64,
) -> f64 {
    let theta = thermal_inertia_val
        / (SIGMA_SB * subsolar_temp.powi(3) * (rotation_period / std::f64::consts::PI).sqrt());
    subsolar_temp * theta.powf(0.5).min(1.0)
}

pub fn sputtering_loss_rate(
    stellar_wind_flux: f64,
    sputtering_yield: f64,
    cross_section: f64,
) -> f64 {
    stellar_wind_flux * sputtering_yield * cross_section
}

pub fn hill_sphere_atmospheric(planet_mass: f64, stellar_mass: f64, semi_major_axis: f64) -> f64 {
    semi_major_axis * (planet_mass / (3.0 * stellar_mass)).powf(1.0 / 3.0)
}

pub struct PlanetData {
    pub mass: f64,
    pub radius: f64,
    pub flattening: f64,
    pub orbital_period: f64,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub axial_tilt: f64,
    pub sidereal_day: f64,
    pub surface_gravity: f64,
    pub escape_velocity: f64,
    pub mean_density: f64,
    pub bond_albedo: f64,
    pub orbital_velocity: f64,
}

pub fn planet_data(name: &str) -> Option<PlanetData> {
    match name.to_ascii_lowercase().as_str() {
        "mercury" => Some(PlanetData {
            mass: MERCURY_MASS,
            radius: MERCURY_RADIUS,
            flattening: MERCURY_FLATTENING,
            orbital_period: MERCURY_ORBITAL_PERIOD,
            semi_major_axis: MERCURY_SEMI_MAJOR_AXIS,
            eccentricity: MERCURY_ECCENTRICITY,
            inclination: MERCURY_INCLINATION,
            axial_tilt: MERCURY_AXIAL_TILT,
            sidereal_day: MERCURY_SIDEREAL_DAY,
            surface_gravity: MERCURY_SURFACE_GRAVITY,
            escape_velocity: MERCURY_ESCAPE_VELOCITY,
            mean_density: MERCURY_MEAN_DENSITY,
            bond_albedo: MERCURY_BOND_ALBEDO,
            orbital_velocity: MERCURY_ORBITAL_VELOCITY,
        }),
        "venus" => Some(PlanetData {
            mass: VENUS_MASS,
            radius: VENUS_RADIUS,
            flattening: VENUS_FLATTENING,
            orbital_period: VENUS_ORBITAL_PERIOD,
            semi_major_axis: VENUS_SEMI_MAJOR_AXIS,
            eccentricity: VENUS_ECCENTRICITY,
            inclination: VENUS_INCLINATION,
            axial_tilt: VENUS_AXIAL_TILT,
            sidereal_day: VENUS_SIDEREAL_DAY,
            surface_gravity: VENUS_SURFACE_GRAVITY,
            escape_velocity: VENUS_ESCAPE_VELOCITY,
            mean_density: VENUS_MEAN_DENSITY,
            bond_albedo: VENUS_BOND_ALBEDO,
            orbital_velocity: VENUS_ORBITAL_VELOCITY,
        }),
        "earth" => Some(PlanetData {
            mass: EARTH_MASS,
            radius: EARTH_RADIUS,
            flattening: 0.003_353_36,
            orbital_period: 365.256_363_004,
            semi_major_axis: crate::constants::AU,
            eccentricity: 0.016_708_6,
            inclination: 0.0,
            axial_tilt: 23.439_3,
            sidereal_day: 86164.1,
            surface_gravity: crate::constants::EARTH_GRAVITY,
            escape_velocity: 11186.0,
            mean_density: 5514.0,
            bond_albedo: 0.306,
            orbital_velocity: EARTH_ORBITAL_VELOCITY,
        }),
        "mars" => Some(PlanetData {
            mass: MARS_MASS,
            radius: MARS_RADIUS,
            flattening: MARS_FLATTENING,
            orbital_period: MARS_ORBITAL_PERIOD,
            semi_major_axis: MARS_SEMI_MAJOR_AXIS,
            eccentricity: MARS_ECCENTRICITY,
            inclination: MARS_INCLINATION,
            axial_tilt: MARS_AXIAL_TILT,
            sidereal_day: MARS_SIDEREAL_DAY,
            surface_gravity: MARS_SURFACE_GRAVITY,
            escape_velocity: MARS_ESCAPE_VELOCITY,
            mean_density: MARS_MEAN_DENSITY,
            bond_albedo: MARS_BOND_ALBEDO,
            orbital_velocity: MARS_ORBITAL_VELOCITY,
        }),
        "jupiter" => Some(PlanetData {
            mass: JUPITER_MASS,
            radius: JUPITER_RADIUS,
            flattening: JUPITER_FLATTENING,
            orbital_period: JUPITER_ORBITAL_PERIOD,
            semi_major_axis: JUPITER_SEMI_MAJOR_AXIS,
            eccentricity: JUPITER_ECCENTRICITY,
            inclination: JUPITER_INCLINATION,
            axial_tilt: JUPITER_AXIAL_TILT,
            sidereal_day: JUPITER_SIDEREAL_DAY,
            surface_gravity: JUPITER_SURFACE_GRAVITY,
            escape_velocity: JUPITER_ESCAPE_VELOCITY,
            mean_density: JUPITER_MEAN_DENSITY,
            bond_albedo: JUPITER_BOND_ALBEDO,
            orbital_velocity: JUPITER_ORBITAL_VELOCITY,
        }),
        "saturn" => Some(PlanetData {
            mass: SATURN_MASS,
            radius: SATURN_RADIUS,
            flattening: SATURN_FLATTENING,
            orbital_period: SATURN_ORBITAL_PERIOD,
            semi_major_axis: SATURN_SEMI_MAJOR_AXIS,
            eccentricity: SATURN_ECCENTRICITY,
            inclination: SATURN_INCLINATION,
            axial_tilt: SATURN_AXIAL_TILT,
            sidereal_day: SATURN_SIDEREAL_DAY,
            surface_gravity: SATURN_SURFACE_GRAVITY,
            escape_velocity: SATURN_ESCAPE_VELOCITY,
            mean_density: SATURN_MEAN_DENSITY,
            bond_albedo: SATURN_BOND_ALBEDO,
            orbital_velocity: SATURN_ORBITAL_VELOCITY,
        }),
        "uranus" => Some(PlanetData {
            mass: URANUS_MASS,
            radius: URANUS_RADIUS,
            flattening: URANUS_FLATTENING,
            orbital_period: URANUS_ORBITAL_PERIOD,
            semi_major_axis: URANUS_SEMI_MAJOR_AXIS,
            eccentricity: URANUS_ECCENTRICITY,
            inclination: URANUS_INCLINATION,
            axial_tilt: URANUS_AXIAL_TILT,
            sidereal_day: URANUS_SIDEREAL_DAY,
            surface_gravity: URANUS_SURFACE_GRAVITY,
            escape_velocity: URANUS_ESCAPE_VELOCITY,
            mean_density: URANUS_MEAN_DENSITY,
            bond_albedo: URANUS_BOND_ALBEDO,
            orbital_velocity: URANUS_ORBITAL_VELOCITY,
        }),
        "neptune" => Some(PlanetData {
            mass: NEPTUNE_MASS,
            radius: NEPTUNE_RADIUS,
            flattening: NEPTUNE_FLATTENING,
            orbital_period: NEPTUNE_ORBITAL_PERIOD,
            semi_major_axis: NEPTUNE_SEMI_MAJOR_AXIS,
            eccentricity: NEPTUNE_ECCENTRICITY,
            inclination: NEPTUNE_INCLINATION,
            axial_tilt: NEPTUNE_AXIAL_TILT,
            sidereal_day: NEPTUNE_SIDEREAL_DAY,
            surface_gravity: NEPTUNE_SURFACE_GRAVITY,
            escape_velocity: NEPTUNE_ESCAPE_VELOCITY,
            mean_density: NEPTUNE_MEAN_DENSITY,
            bond_albedo: NEPTUNE_BOND_ALBEDO,
            orbital_velocity: NEPTUNE_ORBITAL_VELOCITY,
        }),
        _ => None,
    }
}

pub fn planet_mass(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.mass)
}

pub fn planet_radius(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.radius)
}

pub fn planet_flattening(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.flattening)
}

pub fn planet_orbital_period(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.orbital_period)
}

pub fn planet_semi_major_axis(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.semi_major_axis)
}

pub fn planet_eccentricity(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.eccentricity)
}

pub fn planet_inclination(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.inclination)
}

pub fn planet_axial_tilt(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.axial_tilt)
}

pub fn planet_sidereal_day(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.sidereal_day)
}

pub fn planet_surface_gravity(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.surface_gravity)
}

pub fn planet_escape_velocity(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.escape_velocity)
}

pub fn planet_mean_density(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.mean_density)
}

pub fn planet_bond_albedo(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.bond_albedo)
}

pub fn planet_orbital_velocity(name: &str) -> Option<f64> {
    planet_data(name).map(|p| p.orbital_velocity)
}

pub fn planet_volume(name: &str) -> Option<f64> {
    planet_data(name).map(|p| 4.0 / 3.0 * std::f64::consts::PI * p.radius.powi(3))
}

pub fn planet_circumference(name: &str) -> Option<f64> {
    planet_data(name).map(|p| 2.0 * std::f64::consts::PI * p.radius)
}

pub fn planet_surface_area(name: &str) -> Option<f64> {
    planet_data(name).map(|p| 4.0 * std::f64::consts::PI * p.radius.powi(2))
}

pub fn planet_gravitational_parameter(name: &str) -> Option<f64> {
    planet_data(name).map(|p| G * p.mass)
}

pub fn planet_hill_sphere(name: &str, stellar_mass: f64) -> Option<f64> {
    planet_data(name).map(|p| {
        p.semi_major_axis * (1.0 - p.eccentricity) * (p.mass / (3.0 * stellar_mass)).powf(1.0 / 3.0)
    })
}

pub fn planet_roche_limit(name: &str, secondary_density: f64) -> Option<f64> {
    planet_data(name)
        .map(|p| 2.456 * p.radius * (p.mean_density / secondary_density).powf(1.0 / 3.0))
}

pub fn planet_synchronous_orbit(name: &str) -> Option<f64> {
    planet_data(name).map(|p| {
        let omega = 2.0 * std::f64::consts::PI / p.sidereal_day;
        (G * p.mass / omega.powi(2)).powf(1.0 / 3.0)
    })
}
