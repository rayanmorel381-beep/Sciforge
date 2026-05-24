use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 2;

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

pub fn i2_0_6_std_g() -> Bloc { make(cc::_0_6::CC, 70.1, 77.9, "STD_g", 11.0) }
pub fn i2_0_6_hc_g()  -> Bloc { make(cc::_0_6::CC, 70.1, 77.9, "HC_g",  13.0) }
pub fn i2_0_8_std_g() -> Bloc { make(cc::_0_8::CC, 77.1, 85.7, "STD_g", 11.0) }
pub fn i2_0_8_hc_g()  -> Bloc { make(cc::_0_8::CC, 77.1, 85.7, "HC_g",  13.0) }
pub fn i2_1_0_std_g() -> Bloc { make(cc::_1_0::CC, 83.1, 92.3, "STD_g", 11.0) }
pub fn i2_1_0_hc_g()  -> Bloc { make(cc::_1_0::CC, 83.1, 92.3, "HC_g",  13.0) }
pub fn i2_1_2_std_g() -> Bloc { make(cc::_1_2::CC, 88.3, 98.1, "STD_g", 11.0) }
pub fn i2_1_2_hc_g()  -> Bloc { make(cc::_1_2::CC, 88.3, 98.1, "HC_g",  13.0) }
pub fn i2_1_4_std_g() -> Bloc { make(cc::_1_4::CC, 92.9, 103.2, "STD_g", 11.0) }
pub fn i2_1_4_hc_g()  -> Bloc { make(cc::_1_4::CC, 92.9, 103.2, "HC_g",  13.0) }
pub fn i2_1_6_std_g() -> Bloc { make(cc::_1_6::CC, 97.1, 107.9, "STD_g", 11.0) }
pub fn i2_1_6_hc_g()  -> Bloc { make(cc::_1_6::CC, 97.1, 107.9, "HC_g",  13.0) }
pub fn i2_1_8_std_g() -> Bloc { make(cc::_1_8::CC, 101.0, 112.2, "STD_g", 11.0) }
pub fn i2_1_8_hc_g()  -> Bloc { make(cc::_1_8::CC, 101.0, 112.2, "HC_g",  13.0) }
pub fn i2_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 104.6, 116.2, "STD_g", 11.0) }
pub fn i2_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 104.6, 116.2, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        i2_0_6_std_g(), i2_0_6_hc_g(), i2_0_8_std_g(), i2_0_8_hc_g(),
        i2_1_0_std_g(), i2_1_0_hc_g(), i2_1_2_std_g(), i2_1_2_hc_g(),
        i2_1_4_std_g(), i2_1_4_hc_g(), i2_1_6_std_g(), i2_1_6_hc_g(),
        i2_1_8_std_g(), i2_1_8_hc_g(), i2_2_0_std_g(), i2_2_0_hc_g(),
    ]
}
