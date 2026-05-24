use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 6;

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

pub fn f6_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "STD_g", 11.0) }
pub fn f6_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "HC_g",  13.0) }
pub fn f6_2_2_std_g() -> Bloc { make(cc::_2_2::CC, 78.8, 75.0, "STD_g", 11.0) }
pub fn f6_2_2_hc_g()  -> Bloc { make(cc::_2_2::CC, 78.8, 75.0, "HC_g",  13.0) }
pub fn f6_2_4_std_g() -> Bloc { make(cc::_2_4::CC, 81.2, 77.3, "STD_g", 11.0) }
pub fn f6_2_4_hc_g()  -> Bloc { make(cc::_2_4::CC, 81.2, 77.3, "HC_g",  13.0) }
pub fn f6_2_7_std_g() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "STD_g", 11.0) }
pub fn f6_2_7_hc_g()  -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "HC_g",  13.0) }
pub fn f6_3_0_std_g() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "STD_g", 11.0) }
pub fn f6_3_0_hc_g()  -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "HC_g",  13.0) }
pub fn f6_3_2_std_g() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "STD_g", 11.0) }
pub fn f6_3_2_hc_g()  -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "HC_g",  13.0) }
pub fn f6_3_3_std_g() -> Bloc { make(cc::_3_3::CC, 90.3, 86.0, "STD_g", 11.0) }
pub fn f6_3_3_hc_g()  -> Bloc { make(cc::_3_3::CC, 90.3, 86.0, "HC_g",  13.0) }
pub fn f6_3_4_std_g() -> Bloc { make(cc::_3_4::CC, 91.2, 86.9, "STD_g", 11.0) }
pub fn f6_3_4_hc_g()  -> Bloc { make(cc::_3_4::CC, 91.2, 86.9, "HC_g",  13.0) }
pub fn f6_3_6_std_g() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "STD_g", 11.0) }
pub fn f6_3_6_hc_g()  -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "HC_g",  13.0) }
pub fn f6_3_8_std_g() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "STD_g", 11.0) }
pub fn f6_3_8_hc_g()  -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "HC_g",  13.0) }
pub fn f6_4_0_std_g() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "STD_g", 11.0) }
pub fn f6_4_0_hc_g()  -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        f6_2_0_std_g(), f6_2_0_hc_g(), f6_2_2_std_g(), f6_2_2_hc_g(),
        f6_2_4_std_g(), f6_2_4_hc_g(), f6_2_7_std_g(), f6_2_7_hc_g(),
        f6_3_0_std_g(), f6_3_0_hc_g(), f6_3_2_std_g(), f6_3_2_hc_g(),
        f6_3_3_std_g(), f6_3_3_hc_g(), f6_3_4_std_g(), f6_3_4_hc_g(),
        f6_3_6_std_g(), f6_3_6_hc_g(), f6_3_8_std_g(), f6_3_8_hc_g(),
        f6_4_0_std_g(), f6_4_0_hc_g(),
    ]
}
