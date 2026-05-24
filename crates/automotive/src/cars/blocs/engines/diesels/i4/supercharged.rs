use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 4;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_bowl(N), Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn i4_1_0_rt_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "RT_d", 20.0, 0.5) }
pub fn i4_1_0_tw_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "TW_d", 19.0, 0.9) }
pub fn i4_1_0_cf_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "CF_d", 18.5, 1.2) }
pub fn i4_1_1_rt_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "RT_d", 20.0, 0.5) }
pub fn i4_1_1_tw_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "TW_d", 19.0, 0.9) }
pub fn i4_1_1_cf_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "CF_d", 18.5, 1.2) }
pub fn i4_1_2_rt_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "RT_d", 20.0, 0.5) }
pub fn i4_1_2_tw_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "TW_d", 19.0, 0.9) }
pub fn i4_1_2_cf_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "CF_d", 18.5, 1.2) }
pub fn i4_1_3_rt_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "RT_d", 20.0, 0.5) }
pub fn i4_1_3_tw_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "TW_d", 19.0, 0.9) }
pub fn i4_1_3_cf_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "CF_d", 18.5, 1.2) }
pub fn i4_1_4_rt_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "RT_d", 20.0, 0.5) }
pub fn i4_1_4_tw_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "TW_d", 19.0, 0.9) }
pub fn i4_1_4_cf_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "CF_d", 18.5, 1.2) }
