use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 2;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64) -> Bloc {
    Bloc {
        layout: L,
        cylinders: N,
        displacement_cc: cc_val,
        bore_mm: bore,
        stroke_mm: stroke,
        aspiration: Aspiration::Na,
        variant,
        compression_ratio: cr,
        max_boost_bar: 0.0,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn f2_0_6_std_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "STD_g", 11.0) }
pub fn f2_0_6_hc_g()  -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "HC_g",  13.0) }
pub fn f2_0_8_std_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "STD_g", 11.0) }
pub fn f2_0_8_hc_g()  -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "HC_g",  13.0) }
pub fn f2_1_0_std_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "STD_g", 11.0) }
pub fn f2_1_0_hc_g()  -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "HC_g",  13.0) }
pub fn f2_1_2_std_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "STD_g", 11.0) }
pub fn f2_1_2_hc_g()  -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "HC_g",  13.0) }
pub fn f2_1_4_std_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "STD_g", 11.0) }
pub fn f2_1_4_hc_g()  -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "HC_g",  13.0) }
pub fn f2_1_6_std_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "STD_g", 11.0) }
pub fn f2_1_6_hc_g()  -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "HC_g",  13.0) }
pub fn f2_1_8_std_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "STD_g", 11.0) }
pub fn f2_1_8_hc_g()  -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "HC_g",  13.0) }
pub fn f2_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "STD_g", 11.0) }
pub fn f2_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        f2_0_6_std_g(), f2_0_6_hc_g(),
        f2_0_8_std_g(), f2_0_8_hc_g(),
        f2_1_0_std_g(), f2_1_0_hc_g(),
        f2_1_2_std_g(), f2_1_2_hc_g(),
        f2_1_4_std_g(), f2_1_4_hc_g(),
        f2_1_6_std_g(), f2_1_6_hc_g(),
        f2_1_8_std_g(), f2_1_8_hc_g(),
        f2_2_0_std_g(), f2_2_0_hc_g(),
    ]
}
