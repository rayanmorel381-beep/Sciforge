use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 2;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L,
        cylinders: N,
        displacement_cc: cc_val,
        bore_mm: bore,
        stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1),
        variant,
        compression_ratio: cr,
        max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn f2_0_6_ss_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "SS_g", 10.0, 1.3) }
pub fn f2_0_6_ts_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "TS_g",  9.5, 1.6) }
pub fn f2_0_6_vg_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "VG_g",  8.5, 1.8) }
pub fn f2_0_8_ss_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "SS_g", 10.0, 1.3) }
pub fn f2_0_8_ts_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "TS_g",  9.5, 1.6) }
pub fn f2_0_8_vg_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "VG_g",  8.5, 1.8) }
pub fn f2_1_0_ss_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "SS_g", 10.0, 1.3) }
pub fn f2_1_0_ts_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "TS_g",  9.5, 1.6) }
pub fn f2_1_0_vg_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "VG_g",  8.5, 1.8) }
pub fn f2_1_2_ss_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "SS_g", 10.0, 1.3) }
pub fn f2_1_2_ts_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "TS_g",  9.5, 1.6) }
pub fn f2_1_2_vg_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "VG_g",  8.5, 1.8) }
pub fn f2_1_4_ss_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "SS_g", 10.0, 1.3) }
pub fn f2_1_4_ts_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "TS_g",  9.5, 1.6) }
pub fn f2_1_4_vg_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "VG_g",  8.5, 1.8) }
pub fn f2_1_6_ss_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "SS_g", 10.0, 1.3) }
pub fn f2_1_6_ts_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "TS_g",  9.5, 1.6) }
pub fn f2_1_6_vg_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "VG_g",  8.5, 1.8) }
pub fn f2_1_8_ss_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "SS_g", 10.0, 1.3) }
pub fn f2_1_8_ts_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "TS_g",  9.5, 1.6) }
pub fn f2_1_8_vg_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "VG_g",  8.5, 1.8) }
pub fn f2_2_0_ss_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "SS_g", 10.0, 1.3) }
pub fn f2_2_0_ts_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "TS_g",  9.5, 1.6) }
pub fn f2_2_0_vg_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        f2_0_6_ss_g(), f2_0_6_ts_g(), f2_0_6_vg_g(),
        f2_0_8_ss_g(), f2_0_8_ts_g(), f2_0_8_vg_g(),
        f2_1_0_ss_g(), f2_1_0_ts_g(), f2_1_0_vg_g(),
        f2_1_2_ss_g(), f2_1_2_ts_g(), f2_1_2_vg_g(),
        f2_1_4_ss_g(), f2_1_4_ts_g(), f2_1_4_vg_g(),
        f2_1_6_ss_g(), f2_1_6_ts_g(), f2_1_6_vg_g(),
        f2_1_8_ss_g(), f2_1_8_ts_g(), f2_1_8_vg_g(),
        f2_2_0_ss_g(), f2_2_0_ts_g(), f2_2_0_vg_g(),
    ]
}
