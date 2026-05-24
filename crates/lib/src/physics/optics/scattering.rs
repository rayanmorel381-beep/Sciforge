pub fn rayleigh_cross_section(wavelength: f64, refractive_index: f64, depolarization: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let n2_minus_1 = refractive_index.powi(2) - 1.0;
    let king_factor = (6.0 + 3.0 * depolarization) / (6.0 - 7.0 * depolarization);
    8.0 * pi.powi(3) * n2_minus_1.powi(2) * king_factor / (3.0 * wavelength.powi(4))
}

pub fn rayleigh_scattering_coefficient(
    number_density: f64,
    wavelength: f64,
    refractive_index: f64,
    depolarization: f64,
) -> f64 {
    number_density * rayleigh_cross_section(wavelength, refractive_index, depolarization)
}

pub fn rayleigh_phase_function(cos_theta: f64) -> f64 {
    3.0 / (16.0 * std::f64::consts::PI) * (1.0 + cos_theta.powi(2))
}

pub fn mie_extinction_efficiency(size_parameter: f64, refractive_index_real: f64) -> f64 {
    if size_parameter < 0.01 {
        return 0.0;
    }
    let m = refractive_index_real;
    let x = size_parameter;

    if x < 0.5 {
        let m2 = m * m;
        let factor = (m2 - 1.0) / (m2 + 2.0);
        return 8.0 / 3.0 * x.powi(4) * factor.powi(2);
    }

    2.0 + 4.0 / x * (2.0 * x * (m - 1.0)).sin()
        - 4.0 / x.powi(2) * (1.0 - (2.0 * x * (m - 1.0)).cos())
}

pub fn mie_scattering_coefficient(
    number_density: f64,
    particle_radius: f64,
    wavelength: f64,
    refractive_index_real: f64,
) -> f64 {
    let x = 2.0 * std::f64::consts::PI * particle_radius / wavelength;
    let q_ext = mie_extinction_efficiency(x, refractive_index_real);
    number_density * std::f64::consts::PI * particle_radius.powi(2) * q_ext
}

pub fn henyey_greenstein(cos_theta: f64, g: f64) -> f64 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * std::f64::consts::PI * (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5))
}

pub fn optical_depth_integral(
    scattering_coefficient: f64,
    scale_height: f64,
    altitude_start: f64,
    altitude_end: f64,
) -> f64 {
    let h = scale_height;
    scattering_coefficient * h * ((-altitude_start / h).exp() - (-altitude_end / h).exp())
}

pub fn transmittance(optical_depth: f64) -> f64 {
    (-optical_depth).exp()
}

pub fn single_scattering_albedo(scattering_coeff: f64, absorption_coeff: f64) -> f64 {
    scattering_coeff / (scattering_coeff + absorption_coeff)
}

pub fn atmospheric_refraction(zenith_angle: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
    let p_mbar = pressure_pa / 100.0;
    let t_c = temperature_k - 273.15;
    let correction = (p_mbar / 1010.0) * (283.0 / (273.0 + t_c));
    let tan_z = zenith_angle.tan();
    (1.02 / (tan_z + 10.3 / (zenith_angle.to_degrees() + 5.11))) * correction / 60.0
}

pub fn color_temperature_to_rgb(temperature_k: f64) -> (f64, f64, f64) {
    let t = (temperature_k / 100.0).clamp(10.0, 400.0);

    let r = if t <= 66.0 {
        1.0
    } else {
        let x = t - 60.0;
        (329.698727446 * x.powf(-0.1332047592) / 255.0).clamp(0.0, 1.0)
    };

    let g = if t <= 66.0 {
        let x = t;
        (99.4708025861 * x.ln() - 161.1195681661).clamp(0.0, 255.0) / 255.0
    } else {
        let x = t - 60.0;
        (288.1221695283 * x.powf(-0.0755148492) / 255.0).clamp(0.0, 1.0)
    };

    let b = if t >= 66.0 {
        1.0
    } else if t <= 19.0 {
        0.0
    } else {
        let x = t - 10.0;
        (138.5177312231 * x.ln() - 305.0447927307).clamp(0.0, 255.0) / 255.0
    };

    (r, g, b)
}

pub fn planck_spectral_radiance(wavelength: f64, temperature: f64) -> f64 {
    let c1 = 1.191_042_953e-16;
    let c2 = 1.438_776_877e-2;
    let w5 = wavelength.powi(5);
    c1 / (w5 * ((c2 / (wavelength * temperature)).exp() - 1.0))
}

pub fn rayleigh_sky_color(
    wavelength_r: f64,
    wavelength_g: f64,
    wavelength_b: f64,
    optical_depth_zenith: f64,
    cos_zenith: f64,
) -> (f64, f64, f64) {
    let phase = rayleigh_phase_function(cos_zenith);
    let w_ref = wavelength_b;

    let tau_r = optical_depth_zenith;
    let tau_g = tau_r * (w_ref / wavelength_g).powi(4);
    let tau_b = tau_r * (w_ref / wavelength_b).powi(4);
    let tau_r_actual = tau_r * (w_ref / wavelength_r).powi(4);

    let r = phase * (1.0 - (-tau_r_actual / cos_zenith.abs().max(0.01)).exp());
    let g = phase * (1.0 - (-tau_g / cos_zenith.abs().max(0.01)).exp());
    let b = phase * (1.0 - (-tau_b / cos_zenith.abs().max(0.01)).exp());

    (r, g, b)
}

pub fn limb_darkening(cos_angle: f64, coefficient: f64) -> f64 {
    1.0 - coefficient * (1.0 - cos_angle)
}

pub fn absorption_coefficient_gas(cross_section: f64, number_density: f64) -> f64 {
    cross_section * number_density
}

pub fn chapman_function(zenith_angle: f64, scale_height_ratio: f64) -> f64 {
    let x = scale_height_ratio;
    let chi = zenith_angle;
    if chi < std::f64::consts::FRAC_PI_2 {
        return 1.0 / chi.cos();
    }
    let y = x * chi.cos();
    (std::f64::consts::PI * x / 2.0).sqrt() * (x * (1.0 - chi.sin()) - y.powi(2) / (2.0 * x)).exp()
}

pub fn glory_angle(particle_radius: f64, wavelength: f64) -> f64 {
    let x = 2.0 * std::f64::consts::PI * particle_radius / wavelength;
    if x < 1.0 {
        return std::f64::consts::PI;
    }
    std::f64::consts::PI - 2.0 / x
}

pub fn rainbow_angle(refractive_index: f64) -> f64 {
    let n = refractive_index;
    let cos_i = ((n * n - 1.0) / 3.0).sqrt();
    let i = cos_i.acos();
    let r = (cos_i / n).acos();
    std::f64::consts::PI + 2.0 * i - 4.0 * r
}

pub fn wavelength_to_energy_ev(wavelength_nm: f64) -> f64 {
    crate::constants::HC_EV_NM / wavelength_nm
}

pub fn energy_ev_to_wavelength_nm(energy_ev: f64) -> f64 {
    crate::constants::HC_EV_NM / energy_ev
}

pub fn wavelength_angstrom_to_m(wavelength_angstrom: f64) -> f64 {
    wavelength_angstrom * crate::constants::ANGSTROM
}

pub fn wavelength_m_to_angstrom(wavelength_m: f64) -> f64 {
    wavelength_m / crate::constants::ANGSTROM
}

pub fn photon_energy_joule_to_ev(energy_j: f64) -> f64 {
    energy_j * crate::constants::JOULE_TO_EV
}

pub fn size_parameter(radius: f64, wavelength: f64) -> f64 {
    2.0 * std::f64::consts::PI * radius / wavelength
}
