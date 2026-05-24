use crate::constants::{
    ADRASTEA_MASS, ADRASTEA_RADIUS, AMALTHEA_MASS, AMALTHEA_RADIUS, ATLAS_MASS, ATLAS_RADIUS, AU,
    CALLISTO_MASS, CALLISTO_RADIUS, DEGREE_TO_RAD, DEIMOS_MASS, DEIMOS_RADIUS, DIONE_MASS,
    DIONE_RADIUS, EARTH_MOON_DISTANCE, ENCELADUS_MASS, ENCELADUS_RADIUS, EPIMETHEUS_MASS,
    EPIMETHEUS_RADIUS, EUROPA_MASS, EUROPA_RADIUS, G, GANYMEDE_MASS, GANYMEDE_RADIUS,
    HABITABLE_ZONE_INNER_FLUX, HABITABLE_ZONE_OUTER_FLUX, HIMALIA_MASS, HIMALIA_RADIUS,
    HYPERION_MASS, HYPERION_RADIUS, IAPETUS_MASS, IAPETUS_RADIUS, IO_MASS, IO_RADIUS,
    J2000_EPOCH_JD, JANUS_MASS, JANUS_RADIUS, JULIAN_CENTURY_DAYS, LIGHT_YEAR, LUNAR_MASS,
    LUNAR_ORBITAL_PERIOD, LUNAR_RADIUS, METIS_MASS, METIS_RADIUS, MIMAS_MASS, MIMAS_RADIUS,
    NUTATION_AMPLITUDE_ARCSEC, OBERON_MASS, OBERON_RADIUS, PAN_MASS, PAN_RADIUS, PANDORA_MASS,
    PANDORA_RADIUS, PHOBOS_MASS, PHOBOS_RADIUS, PROMETHEUS_MASS, PROMETHEUS_RADIUS, RAD_TO_DEGREE,
    RHEA_MASS, RHEA_RADIUS, SECONDS_PER_DAY, SIDEREAL_YEAR, TETHYS_MASS, TETHYS_RADIUS, THEBE_MASS,
    THEBE_RADIUS, TIDAL_DISSIPATION_COEFF, TITAN_MASS, TITAN_RADIUS, TITANIA_MASS, TITANIA_RADIUS,
    TRITON_MASS, TRITON_RADIUS, TROPICAL_YEAR, VERNAL_EQUINOX_DOY,
};

pub fn gravitational_force(m1: f64, m2: f64, r: f64) -> f64 {
    G * m1 * m2 / (r * r)
}

pub fn tidal_force(m: f64, r: f64, delta_r: f64) -> f64 {
    2.0 * G * m * delta_r / (r * r * r)
}

pub fn synodic_period(p1: f64, p2: f64) -> f64 {
    (1.0 / p1 - 1.0 / p2).abs().recip()
}

pub fn apparent_angular_size(diameter: f64, distance: f64) -> f64 {
    2.0 * (diameter / (2.0 * distance)).atan()
}

pub fn parallax_distance(parallax_arcsec: f64) -> f64 {
    1.0 / parallax_arcsec
}

pub fn barycenter(m1: f64, m2: f64, d: f64) -> f64 {
    d * m2 / (m1 + m2)
}

pub fn lagrange_l1(d: f64, m1: f64, m2: f64) -> f64 {
    d * (m2 / (3.0 * m1)).powf(1.0 / 3.0)
}

pub fn hill_sphere(a: f64, m: f64, m_star: f64, e: f64) -> f64 {
    a * (1.0 - e) * (m / (3.0 * m_star)).powf(1.0 / 3.0)
}

pub fn surface_gravity(m: f64, r: f64) -> f64 {
    G * m / (r * r)
}

pub fn sidereal_to_solar(sidereal_period: f64, orbital_period: f64) -> f64 {
    1.0 / (1.0 / sidereal_period - 1.0 / orbital_period)
}

pub fn habitable_zone_inner(luminosity_solar: f64) -> f64 {
    (luminosity_solar / HABITABLE_ZONE_INNER_FLUX).sqrt()
}

pub fn habitable_zone_outer(luminosity_solar: f64) -> f64 {
    (luminosity_solar / HABITABLE_ZONE_OUTER_FLUX).sqrt()
}

pub fn julian_date_to_j2000_century(jd: f64) -> f64 {
    (jd - J2000_EPOCH_JD) / JULIAN_CENTURY_DAYS
}

pub fn j2000_century_to_julian_date(t: f64) -> f64 {
    t * JULIAN_CENTURY_DAYS + J2000_EPOCH_JD
}

pub fn mean_obliquity_ecliptic(t_century: f64) -> f64 {
    let epsilon_arcsec = 84381.448 - 46.8150 * t_century - 0.000_59 * t_century * t_century
        + 0.001_813 * t_century.powi(3);
    epsilon_arcsec / 3600.0 * DEGREE_TO_RAD
}

pub fn nutation_longitude(omega: f64) -> f64 {
    -NUTATION_AMPLITUDE_ARCSEC * omega.sin() / 3600.0 * DEGREE_TO_RAD
}

pub fn precession_rate_arcsec_per_year(t_century: f64) -> f64 {
    50.290_966 + 0.022_222 * t_century
}

pub fn equation_of_time(day_of_year: f64) -> f64 {
    let b = 2.0 * std::f64::consts::PI * (day_of_year - VERNAL_EQUINOX_DOY) / 365.25;
    -7.655 * b.sin() + 9.873 * (2.0 * b).sin() + 3.334 * (3.0 * b).sin()
}

pub fn sidereal_year_seconds() -> f64 {
    SIDEREAL_YEAR
}

pub fn tropical_year_seconds() -> f64 {
    TROPICAL_YEAR
}

pub fn precession_period() -> f64 {
    360.0 * 3600.0 / 50.290_966 * TROPICAL_YEAR
}

pub fn day_length_seconds() -> f64 {
    SECONDS_PER_DAY
}

pub fn solar_day_to_sidereal_day(solar_day_s: f64, orbital_period_s: f64) -> f64 {
    1.0 / (1.0 / solar_day_s + 1.0 / orbital_period_s)
}

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * DEGREE_TO_RAD
}

pub fn radians_to_degrees(rad: f64) -> f64 {
    rad * RAD_TO_DEGREE
}

pub fn tidal_dissipation_factor() -> f64 {
    TIDAL_DISSIPATION_COEFF
}

pub fn tidal_quality_factor(specific_dissipation: f64) -> f64 {
    1.0 / (2.0 * specific_dissipation * TIDAL_DISSIPATION_COEFF)
}

pub fn tidal_heating(mass_primary: f64, radius: f64, eccentricity: f64, a: f64, n: f64) -> f64 {
    TIDAL_DISSIPATION_COEFF
        * G
        * mass_primary
        * mass_primary
        * radius.powi(5)
        * n
        * eccentricity
        * eccentricity
        / a.powi(6)
}

pub fn au_to_meters(au: f64) -> f64 {
    au * AU
}

pub fn meters_to_au(m: f64) -> f64 {
    m / AU
}

pub fn light_years_to_meters(ly: f64) -> f64 {
    ly * LIGHT_YEAR
}

pub fn meters_to_light_years(m: f64) -> f64 {
    m / LIGHT_YEAR
}

pub fn au_to_light_years(au: f64) -> f64 {
    au * AU / LIGHT_YEAR
}

pub fn light_years_to_au(ly: f64) -> f64 {
    ly * LIGHT_YEAR / AU
}

pub fn earth_moon_distance() -> f64 {
    EARTH_MOON_DISTANCE
}

pub fn lunar_orbital_period() -> f64 {
    LUNAR_ORBITAL_PERIOD
}

pub struct MoonData {
    pub mass: f64,
    pub radius: f64,
}

pub fn moon_data(name: &str) -> Option<MoonData> {
    match name.to_ascii_lowercase().as_str() {
        "moon" | "luna" => Some(MoonData {
            mass: LUNAR_MASS,
            radius: LUNAR_RADIUS,
        }),
        "io" => Some(MoonData {
            mass: IO_MASS,
            radius: IO_RADIUS,
        }),
        "europa" => Some(MoonData {
            mass: EUROPA_MASS,
            radius: EUROPA_RADIUS,
        }),
        "ganymede" => Some(MoonData {
            mass: GANYMEDE_MASS,
            radius: GANYMEDE_RADIUS,
        }),
        "callisto" => Some(MoonData {
            mass: CALLISTO_MASS,
            radius: CALLISTO_RADIUS,
        }),
        "titan" => Some(MoonData {
            mass: TITAN_MASS,
            radius: TITAN_RADIUS,
        }),
        "enceladus" => Some(MoonData {
            mass: ENCELADUS_MASS,
            radius: ENCELADUS_RADIUS,
        }),
        "triton" => Some(MoonData {
            mass: TRITON_MASS,
            radius: TRITON_RADIUS,
        }),
        "phobos" => Some(MoonData {
            mass: PHOBOS_MASS,
            radius: PHOBOS_RADIUS,
        }),
        "deimos" => Some(MoonData {
            mass: DEIMOS_MASS,
            radius: DEIMOS_RADIUS,
        }),
        "oberon" => Some(MoonData {
            mass: OBERON_MASS,
            radius: OBERON_RADIUS,
        }),
        "titania" => Some(MoonData {
            mass: TITANIA_MASS,
            radius: TITANIA_RADIUS,
        }),
        "tethys" => Some(MoonData {
            mass: TETHYS_MASS,
            radius: TETHYS_RADIUS,
        }),
        "dione" => Some(MoonData {
            mass: DIONE_MASS,
            radius: DIONE_RADIUS,
        }),
        "rhea" => Some(MoonData {
            mass: RHEA_MASS,
            radius: RHEA_RADIUS,
        }),
        "iapetus" => Some(MoonData {
            mass: IAPETUS_MASS,
            radius: IAPETUS_RADIUS,
        }),
        "hyperion" => Some(MoonData {
            mass: HYPERION_MASS,
            radius: HYPERION_RADIUS,
        }),
        "mimas" => Some(MoonData {
            mass: MIMAS_MASS,
            radius: MIMAS_RADIUS,
        }),
        "prometheus" => Some(MoonData {
            mass: PROMETHEUS_MASS,
            radius: PROMETHEUS_RADIUS,
        }),
        "pandora" => Some(MoonData {
            mass: PANDORA_MASS,
            radius: PANDORA_RADIUS,
        }),
        "atlas" => Some(MoonData {
            mass: ATLAS_MASS,
            radius: ATLAS_RADIUS,
        }),
        "pan" => Some(MoonData {
            mass: PAN_MASS,
            radius: PAN_RADIUS,
        }),
        "epimetheus" => Some(MoonData {
            mass: EPIMETHEUS_MASS,
            radius: EPIMETHEUS_RADIUS,
        }),
        "janus" => Some(MoonData {
            mass: JANUS_MASS,
            radius: JANUS_RADIUS,
        }),
        "amalthea" => Some(MoonData {
            mass: AMALTHEA_MASS,
            radius: AMALTHEA_RADIUS,
        }),
        "thebe" => Some(MoonData {
            mass: THEBE_MASS,
            radius: THEBE_RADIUS,
        }),
        "himalia" => Some(MoonData {
            mass: HIMALIA_MASS,
            radius: HIMALIA_RADIUS,
        }),
        "metis" => Some(MoonData {
            mass: METIS_MASS,
            radius: METIS_RADIUS,
        }),
        "adrastea" => Some(MoonData {
            mass: ADRASTEA_MASS,
            radius: ADRASTEA_RADIUS,
        }),
        _ => None,
    }
}

pub fn moon_mass(name: &str) -> Option<f64> {
    moon_data(name).map(|m| m.mass)
}

pub fn moon_radius(name: &str) -> Option<f64> {
    moon_data(name).map(|m| m.radius)
}

pub fn moon_surface_gravity(name: &str) -> Option<f64> {
    moon_data(name).map(|m| G * m.mass / (m.radius * m.radius))
}

pub fn moon_escape_velocity(name: &str) -> Option<f64> {
    moon_data(name).map(|m| (2.0 * G * m.mass / m.radius).sqrt())
}

pub fn moon_volume(name: &str) -> Option<f64> {
    moon_data(name).map(|m| 4.0 / 3.0 * std::f64::consts::PI * m.radius.powi(3))
}

pub fn moon_mean_density(name: &str) -> Option<f64> {
    moon_data(name).map(|m| m.mass / (4.0 / 3.0 * std::f64::consts::PI * m.radius.powi(3)))
}
