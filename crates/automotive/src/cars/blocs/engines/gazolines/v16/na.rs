use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

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

pub fn v16_6_0_std_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "STD_g", 11.0) }
pub fn v16_6_0_hc_g()  -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "HC_g",  13.0) }
pub fn v16_6_75_std_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "STD_g", 11.0) }
pub fn v16_6_75_hc_g()  -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "HC_g",  13.0) }
pub fn v16_7_0_std_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "STD_g", 11.0) }
pub fn v16_7_0_hc_g()  -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "HC_g",  13.0) }
pub fn v16_7_4_std_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "STD_g", 11.0) }
pub fn v16_7_4_hc_g()  -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "HC_g",  13.0) }
pub fn v16_8_0_std_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "STD_g", 11.0) }
pub fn v16_8_0_hc_g()  -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "HC_g",  13.0) }
pub fn v16_8_4_std_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "STD_g", 11.0) }
pub fn v16_8_4_hc_g()  -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "HC_g",  13.0) }
pub fn v16_9_0_std_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "STD_g", 11.0) }
pub fn v16_9_0_hc_g()  -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "HC_g",  13.0) }
pub fn v16_13_6_std_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "STD_g", 11.0) }
pub fn v16_13_6_hc_g()  -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        v16_6_0_std_g(), v16_6_0_hc_g(), v16_6_75_std_g(), v16_6_75_hc_g(),
        v16_7_0_std_g(), v16_7_0_hc_g(), v16_7_4_std_g(), v16_7_4_hc_g(),
        v16_8_0_std_g(), v16_8_0_hc_g(), v16_8_4_std_g(), v16_8_4_hc_g(),
        v16_9_0_std_g(), v16_9_0_hc_g(), v16_13_6_std_g(), v16_13_6_hc_g(),
    ]
}
