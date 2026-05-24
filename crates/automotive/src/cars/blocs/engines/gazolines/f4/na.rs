use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 4;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Na, variant,
        compression_ratio: cr, max_boost_bar: 0.0,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn f4_1_1_std_g() -> Bloc { make(cc::_1_1::CC, 70.5, 70.5, "STD_g", 11.0) }
pub fn f4_1_1_hc_g()  -> Bloc { make(cc::_1_1::CC, 70.5, 70.5, "HC_g",  13.0) }
pub fn f4_1_2_std_g() -> Bloc { make(cc::_1_2::CC, 72.6, 72.6, "STD_g", 11.0) }
pub fn f4_1_2_hc_g()  -> Bloc { make(cc::_1_2::CC, 72.6, 72.6, "HC_g",  13.0) }
pub fn f4_1_3_std_g() -> Bloc { make(cc::_1_3::CC, 74.5, 74.5, "STD_g", 11.0) }
pub fn f4_1_3_hc_g()  -> Bloc { make(cc::_1_3::CC, 74.5, 74.5, "HC_g",  13.0) }
pub fn f4_1_4_std_g() -> Bloc { make(cc::_1_4::CC, 76.4, 76.4, "STD_g", 11.0) }
pub fn f4_1_4_hc_g()  -> Bloc { make(cc::_1_4::CC, 76.4, 76.4, "HC_g",  13.0) }
pub fn f4_1_5_std_g() -> Bloc { make(cc::_1_5::CC, 78.2, 78.2, "STD_g", 11.0) }
pub fn f4_1_5_hc_g()  -> Bloc { make(cc::_1_5::CC, 78.2, 78.2, "HC_g",  13.0) }
pub fn f4_1_6_std_g() -> Bloc { make(cc::_1_6::CC, 79.9, 79.9, "STD_g", 11.0) }
pub fn f4_1_6_hc_g()  -> Bloc { make(cc::_1_6::CC, 79.9, 79.9, "HC_g",  13.0) }
pub fn f4_1_7_std_g() -> Bloc { make(cc::_1_7::CC, 81.5, 81.5, "STD_g", 11.0) }
pub fn f4_1_7_hc_g()  -> Bloc { make(cc::_1_7::CC, 81.5, 81.5, "HC_g",  13.0) }
pub fn f4_1_8_std_g() -> Bloc { make(cc::_1_8::CC, 83.1, 83.1, "STD_g", 11.0) }
pub fn f4_1_8_hc_g()  -> Bloc { make(cc::_1_8::CC, 83.1, 83.1, "HC_g",  13.0) }
pub fn f4_1_9_std_g() -> Bloc { make(cc::_1_9::CC, 84.6, 84.6, "STD_g", 11.0) }
pub fn f4_1_9_hc_g()  -> Bloc { make(cc::_1_9::CC, 84.6, 84.6, "HC_g",  13.0) }
pub fn f4_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 86.0, 86.0, "STD_g", 11.0) }
pub fn f4_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 86.0, 86.0, "HC_g",  13.0) }
pub fn f4_2_1_std_g() -> Bloc { make(cc::_2_1::CC, 87.4, 87.4, "STD_g", 11.0) }
pub fn f4_2_1_hc_g()  -> Bloc { make(cc::_2_1::CC, 87.4, 87.4, "HC_g",  13.0) }
pub fn f4_2_2_std_g() -> Bloc { make(cc::_2_2::CC, 88.8, 88.8, "STD_g", 11.0) }
pub fn f4_2_2_hc_g()  -> Bloc { make(cc::_2_2::CC, 88.8, 88.8, "HC_g",  13.0) }
pub fn f4_2_3_std_g() -> Bloc { make(cc::_2_3::CC, 90.1, 90.1, "STD_g", 11.0) }
pub fn f4_2_3_hc_g()  -> Bloc { make(cc::_2_3::CC, 90.1, 90.1, "HC_g",  13.0) }
pub fn f4_2_4_std_g() -> Bloc { make(cc::_2_4::CC, 91.4, 91.4, "STD_g", 11.0) }
pub fn f4_2_4_hc_g()  -> Bloc { make(cc::_2_4::CC, 91.4, 91.4, "HC_g",  13.0) }
pub fn f4_2_5_std_g() -> Bloc { make(cc::_2_5::CC, 92.7, 92.7, "STD_g", 11.0) }
pub fn f4_2_5_hc_g()  -> Bloc { make(cc::_2_5::CC, 92.7, 92.7, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        f4_1_1_std_g(), f4_1_1_hc_g(), f4_1_2_std_g(), f4_1_2_hc_g(),
        f4_1_3_std_g(), f4_1_3_hc_g(), f4_1_4_std_g(), f4_1_4_hc_g(),
        f4_1_5_std_g(), f4_1_5_hc_g(), f4_1_6_std_g(), f4_1_6_hc_g(),
        f4_1_7_std_g(), f4_1_7_hc_g(), f4_1_8_std_g(), f4_1_8_hc_g(),
        f4_1_9_std_g(), f4_1_9_hc_g(), f4_2_0_std_g(), f4_2_0_hc_g(),
        f4_2_1_std_g(), f4_2_1_hc_g(), f4_2_2_std_g(), f4_2_2_hc_g(),
        f4_2_3_std_g(), f4_2_3_hc_g(), f4_2_4_std_g(), f4_2_4_hc_g(),
        f4_2_5_std_g(), f4_2_5_hc_g(),
    ]
}
