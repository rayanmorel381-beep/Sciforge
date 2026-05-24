pub fn fundamental_frequency_string(l: f64, tension: f64, mu: f64) -> f64 {
    (tension / mu).sqrt() / (2.0 * l)
}

pub fn harmonic_frequency(fundamental: f64, n: u32) -> f64 {
    fundamental * n as f64
}

pub fn resonant_frequency_pipe_open(l: f64, c: f64, n: u32) -> f64 {
    n as f64 * c / (2.0 * l)
}

pub fn resonant_frequency_pipe_closed(l: f64, c: f64, n: u32) -> f64 {
    (2 * n - 1) as f64 * c / (4.0 * l)
}

pub fn quality_factor(f0: f64, bandwidth: f64) -> f64 {
    f0 / bandwidth.max(1e-30)
}

pub fn helmholtz_resonator(c: f64, a: f64, v: f64, l: f64) -> f64 {
    c / (2.0 * std::f64::consts::PI) * (a / (v * l)).sqrt()
}

pub fn beat_frequency(f1: f64, f2: f64) -> f64 {
    (f1 - f2).abs()
}

pub fn standing_wave_nodes(l: f64, wavelength: f64) -> u32 {
    (2.0 * l / wavelength).round() as u32
}

pub fn reverberation_time_sabine(v: f64, a: f64) -> f64 {
    0.161 * v / a.max(1e-30)
}

pub fn room_mode_frequency(c: f64, lx: f64, ly: f64, lz: f64, nx: u32, ny: u32, nz: u32) -> f64 {
    c / 2.0
        * ((nx as f64 / lx).powi(2) + (ny as f64 / ly).powi(2) + (nz as f64 / lz).powi(2)).sqrt()
}

pub fn schroeder_frequency(rt60: f64, v: f64) -> f64 {
    2000.0 * (rt60 / v).sqrt()
}
