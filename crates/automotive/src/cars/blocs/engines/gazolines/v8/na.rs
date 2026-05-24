use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 8;

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

pub fn v8_3_5_std_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "STD_g", 11.0) }
pub fn v8_3_5_hc_g()  -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "HC_g",  13.0) }
pub fn v8_3_9_std_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "STD_g", 11.0) }
pub fn v8_3_9_hc_g()  -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "HC_g",  13.0) }
pub fn v8_4_0_std_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "STD_g", 11.0) }
pub fn v8_4_0_hc_g()  -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "HC_g",  13.0) }
pub fn v8_4_4_std_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "STD_g", 11.0) }
pub fn v8_4_4_hc_g()  -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "HC_g",  13.0) }
pub fn v8_4_5_std_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "STD_g", 11.0) }
pub fn v8_4_5_hc_g()  -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "HC_g",  13.0) }
pub fn v8_4_6_std_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "STD_g", 11.0) }
pub fn v8_4_6_hc_g()  -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "HC_g",  13.0) }
pub fn v8_4_8_std_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "STD_g", 11.0) }
pub fn v8_4_8_hc_g()  -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "HC_g",  13.0) }
pub fn v8_5_0_std_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "STD_g", 11.0) }
pub fn v8_5_0_hc_g()  -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "HC_g",  13.0) }
pub fn v8_5_3_std_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "STD_g", 11.0) }
pub fn v8_5_3_hc_g()  -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "HC_g",  13.0) }
pub fn v8_5_5_std_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "STD_g", 11.0) }
pub fn v8_5_5_hc_g()  -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "HC_g",  13.0) }
pub fn v8_5_7_std_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "STD_g", 11.0) }
pub fn v8_5_7_hc_g()  -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "HC_g",  13.0) }
pub fn v8_6_0_std_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "STD_g", 11.0) }
pub fn v8_6_0_hc_g()  -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "HC_g",  13.0) }
pub fn v8_6_2_std_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "STD_g", 11.0) }
pub fn v8_6_2_hc_g()  -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "HC_g",  13.0) }
pub fn v8_6_4_std_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "STD_g", 11.0) }
pub fn v8_6_4_hc_g()  -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "HC_g",  13.0) }
pub fn v8_7_0_std_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "STD_g", 11.0) }
pub fn v8_7_0_hc_g()  -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        v8_3_5_std_g(), v8_3_5_hc_g(), v8_3_9_std_g(), v8_3_9_hc_g(),
        v8_4_0_std_g(), v8_4_0_hc_g(), v8_4_4_std_g(), v8_4_4_hc_g(),
        v8_4_5_std_g(), v8_4_5_hc_g(), v8_4_6_std_g(), v8_4_6_hc_g(),
        v8_4_8_std_g(), v8_4_8_hc_g(), v8_5_0_std_g(), v8_5_0_hc_g(),
        v8_5_3_std_g(), v8_5_3_hc_g(), v8_5_5_std_g(), v8_5_5_hc_g(),
        v8_5_7_std_g(), v8_5_7_hc_g(), v8_6_0_std_g(), v8_6_0_hc_g(),
        v8_6_2_std_g(), v8_6_2_hc_g(), v8_6_4_std_g(), v8_6_4_hc_g(),
        v8_7_0_std_g(), v8_7_0_hc_g(),
    ]
}
