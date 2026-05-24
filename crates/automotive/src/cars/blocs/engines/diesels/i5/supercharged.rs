use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 5;

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

pub fn i5_2_0_rt_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "RT_d", 20.0, 0.5) }
pub fn i5_2_0_tw_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "TW_d", 19.0, 0.9) }
pub fn i5_2_0_cf_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "CF_d", 18.5, 1.2) }
pub fn i5_2_1_rt_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "RT_d", 20.0, 0.5) }
pub fn i5_2_1_tw_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "TW_d", 19.0, 0.9) }
pub fn i5_2_1_cf_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "CF_d", 18.5, 1.2) }
