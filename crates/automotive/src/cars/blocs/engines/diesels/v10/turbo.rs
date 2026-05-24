use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 10;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![crate::components::powertrain::engines::thermals::parts::culasses::Culasse::ohv_2v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::sohc_2v_1p_pre_chamber(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v10_4_0_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_4_0_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_4_0_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_4_2_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_4_2_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_4_2_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_5_0_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_5_0_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_5_0_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_5_2_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_5_2_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_5_2_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_5_7_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_5_7_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_5_7_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_8_3_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_8_3_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_8_3_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "VG_d",  16.0, 2.2) }
pub fn v10_8_4_ss_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "SS_d", 17.5, 1.5) }
pub fn v10_8_4_ts_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "TS_d",  16.5, 1.9) }
pub fn v10_8_4_vg_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "VG_d",  16.0, 2.2) }

