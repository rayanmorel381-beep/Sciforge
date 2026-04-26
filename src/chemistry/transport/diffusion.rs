pub fn fick_first_law(d: f64, dc_dx: f64) -> f64 {
    -d * dc_dx
}

pub fn fick_second_law_solution(c0: f64, cs: f64, x: f64, d: f64, t: f64) -> f64 {
    cs - (cs - c0) * erf_approx(x / (2.0 * (d * t).max(1e-30).sqrt()))
}

fn erf_approx(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.3275911 * x.abs());
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let result = 1.0 - poly * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}

pub fn diffusion_coefficient_stokes_einstein(t: f64, viscosity: f64, r: f64) -> f64 {
    crate::constants::K_B * t / (6.0 * std::f64::consts::PI * viscosity * r).max(1e-30)
}

pub fn wilke_chang(t: f64, viscosity: f64, mw_solvent: f64, phi: f64, v_solute: f64) -> f64 {
    7.4e-8 * (phi * mw_solvent).sqrt() * t / (viscosity * v_solute.powf(0.6)).max(1e-30)
}

pub fn knudsen_diffusivity(r_pore: f64, t: f64, mw: f64) -> f64 {
    r_pore * 2.0 / 3.0
        * (8.0 * crate::constants::R_GAS * t / (std::f64::consts::PI * mw).max(1e-30)).sqrt()
}

pub fn effective_diffusivity(d_bulk: f64, porosity: f64, tortuosity: f64) -> f64 {
    d_bulk * porosity / tortuosity.max(1e-30)
}

pub fn diffusion_time_estimate(length: f64, d: f64) -> f64 {
    length * length / (2.0 * d).max(1e-30)
}
