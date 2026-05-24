pub fn diode_shockley(is: f64, v: f64, n: f64, vt: f64) -> f64 {
    is * ((v / (n * vt)).exp() - 1.0)
}

pub fn zener_voltage_regulation(v_in: f64, v_zener: f64) -> f64 {
    if v_in > v_zener { v_zener } else { v_in }
}

pub fn bjt_ic_active(beta: f64, ib: f64) -> f64 {
    beta * ib
}
pub fn bjt_ie(ic: f64, ib: f64) -> f64 {
    ic + ib
}
pub fn bjt_alpha(beta: f64) -> f64 {
    beta / (1.0 + beta)
}

pub fn mosfet_drain_current_saturation(kn: f64, vgs: f64, vth: f64) -> f64 {
    if vgs <= vth {
        0.0
    } else {
        0.5 * kn * (vgs - vth).powi(2)
    }
}

pub fn mosfet_drain_current_linear(kn: f64, vgs: f64, vth: f64, vds: f64) -> f64 {
    if vgs <= vth {
        return 0.0;
    }
    kn * ((vgs - vth) * vds - 0.5 * vds * vds)
}

pub fn mosfet_threshold_body_effect(vth0: f64, gamma: f64, vsb: f64, phi: f64) -> f64 {
    vth0 + gamma * ((2.0 * phi + vsb).sqrt() - (2.0 * phi).sqrt())
}

pub fn solar_cell_iv(i_photo: f64, i0: f64, v: f64, n: f64, vt: f64, r_s: f64) -> f64 {
    i_photo - i0 * ((v + i_photo * r_s) / (n * vt)).exp()
}

pub fn led_resistor(v_supply: f64, v_led: f64, i_led: f64) -> f64 {
    (v_supply - v_led) / i_led
}

pub fn photodiode_responsivity(i_photo: f64, p_optical: f64) -> f64 {
    i_photo / p_optical
}

pub fn tunnel_diode_current(ip: f64, iv: f64, vp: f64, vv: f64, v: f64) -> f64 {
    if v <= vp {
        ip * v / vp
    } else if v <= vv {
        ip - (ip - iv) * (v - vp) / (vv - vp)
    } else {
        iv * ((v - vv) / vp).exp()
    }
}

pub fn pn_junction_capacitance(c0: f64, v: f64, v_bi: f64, m: f64) -> f64 {
    c0 / (1.0 - v / v_bi).powf(m)
}

pub fn early_effect(ic0: f64, vce: f64, va: f64) -> f64 {
    ic0 * (1.0 + vce / va)
}

pub fn drain_induced_barrier_lowering(vth0: f64, sigma: f64, vds: f64) -> f64 {
    vth0 - sigma * vds
}
