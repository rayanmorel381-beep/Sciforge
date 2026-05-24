use crate::powertrain::transmissions::clutches::WetClutch;

const I: u8 = 0;
const F: u8 = 1;
const V: u8 = 2;
const W: u8 = 3;
const VR: u8 = 4;

fn tier_for(disp_l: f64) -> usize {
    if disp_l < 1.0 { 0 }
    else if disp_l < 1.6 { 1 }
    else if disp_l < 2.2 { 2 }
    else if disp_l < 3.0 { 3 }
    else if disp_l < 4.5 { 4 }
    else if disp_l < 6.5 { 5 }
    else if disp_l < 10.0 { 6 }
    else { 7 }
}

fn layout_mul(layout: u8) -> f64 {
    match layout {
        F => 0.98,
        W | VR => 1.04,
        V => 1.05,
        _ => 1.00,
    }
}

fn build(disp_l: f64, layout: u8) -> WetClutch {
    let torques: [f64; 8] = [80.0, 130.0, 190.0, 270.0, 380.0, 540.0, 720.0, 980.0];
    let diams: [f64; 8] = [140.0, 155.0, 165.0, 175.0, 185.0, 195.0, 205.0, 215.0];
    let tier = tier_for(disp_l);
    let max_torque_nm = (torques[tier] * 1.3 * layout_mul(layout) * 100.0).round() / 100.0;
    WetClutch::motorcycle(1, diams[tier], max_torque_nm)
}

pub fn i3_0_6() -> WetClutch { build(0.6, I) }
pub fn i3_0_7() -> WetClutch { build(0.7, I) }
pub fn i3_0_8() -> WetClutch { build(0.8, I) }
pub fn i3_0_9() -> WetClutch { build(0.9, I) }
pub fn i3_1_0() -> WetClutch { build(1.0, I) }
pub fn i3_1_1() -> WetClutch { build(1.1, I) }
pub fn i3_1_2() -> WetClutch { build(1.2, I) }
pub fn i3_1_3() -> WetClutch { build(1.3, I) }
pub fn i3_1_4() -> WetClutch { build(1.4, I) }
pub fn i3_1_5() -> WetClutch { build(1.5, I) }

pub fn i4_1_0() -> WetClutch { build(1.0, I) }
pub fn i4_1_1() -> WetClutch { build(1.1, I) }
pub fn i4_1_2() -> WetClutch { build(1.2, I) }
pub fn i4_1_3() -> WetClutch { build(1.3, I) }
pub fn i4_1_4() -> WetClutch { build(1.4, I) }
pub fn i4_1_5() -> WetClutch { build(1.5, I) }
pub fn i4_1_6() -> WetClutch { build(1.6, I) }
pub fn i4_1_7() -> WetClutch { build(1.7, I) }
pub fn i4_1_8() -> WetClutch { build(1.8, I) }
pub fn i4_1_9() -> WetClutch { build(1.9, I) }
pub fn i4_2_0() -> WetClutch { build(2.0, I) }
pub fn i4_2_1() -> WetClutch { build(2.1, I) }
pub fn i4_2_2() -> WetClutch { build(2.2, I) }
pub fn i4_2_3() -> WetClutch { build(2.3, I) }
pub fn i4_2_4() -> WetClutch { build(2.4, I) }

pub fn i5_2_0() -> WetClutch { build(2.0, I) }
pub fn i5_2_1() -> WetClutch { build(2.1, I) }
pub fn i5_2_2() -> WetClutch { build(2.2, I) }
pub fn i5_2_3() -> WetClutch { build(2.3, I) }
pub fn i5_2_4() -> WetClutch { build(2.4, I) }
pub fn i5_2_5() -> WetClutch { build(2.5, I) }

pub fn i6_2_0() -> WetClutch { build(2.0, I) }
pub fn i6_2_3() -> WetClutch { build(2.3, I) }
pub fn i6_2_5() -> WetClutch { build(2.5, I) }
pub fn i6_2_8() -> WetClutch { build(2.8, I) }
pub fn i6_3_0() -> WetClutch { build(3.0, I) }
pub fn i6_3_2() -> WetClutch { build(3.2, I) }
pub fn i6_3_5() -> WetClutch { build(3.5, I) }

pub fn flat4_1_1() -> WetClutch { build(1.1, F) }
pub fn flat4_1_2() -> WetClutch { build(1.2, F) }
pub fn flat4_1_3() -> WetClutch { build(1.3, F) }
pub fn flat4_1_4() -> WetClutch { build(1.4, F) }
pub fn flat4_1_5() -> WetClutch { build(1.5, F) }
pub fn flat4_1_6() -> WetClutch { build(1.6, F) }
pub fn flat4_1_7() -> WetClutch { build(1.7, F) }
pub fn flat4_1_8() -> WetClutch { build(1.8, F) }
pub fn flat4_1_9() -> WetClutch { build(1.9, F) }
pub fn flat4_2_0() -> WetClutch { build(2.0, F) }
pub fn flat4_2_1() -> WetClutch { build(2.1, F) }
pub fn flat4_2_2() -> WetClutch { build(2.2, F) }
pub fn flat4_2_3() -> WetClutch { build(2.3, F) }
pub fn flat4_2_4() -> WetClutch { build(2.4, F) }
pub fn flat4_2_5() -> WetClutch { build(2.5, F) }

pub fn flat6_2_0() -> WetClutch { build(2.0, F) }
pub fn flat6_2_2() -> WetClutch { build(2.2, F) }
pub fn flat6_2_4() -> WetClutch { build(2.4, F) }
pub fn flat6_2_7() -> WetClutch { build(2.7, F) }
pub fn flat6_3_0() -> WetClutch { build(3.0, F) }
pub fn flat6_3_2() -> WetClutch { build(3.2, F) }
pub fn flat6_3_3() -> WetClutch { build(3.3, F) }
pub fn flat6_3_4() -> WetClutch { build(3.4, F) }
pub fn flat6_3_6() -> WetClutch { build(3.6, F) }
pub fn flat6_3_8() -> WetClutch { build(3.8, F) }
pub fn flat6_4_0() -> WetClutch { build(4.0, F) }

pub fn flat8_2_5() -> WetClutch { build(2.5, F) }
pub fn flat8_3_0() -> WetClutch { build(3.0, F) }
pub fn flat8_3_3() -> WetClutch { build(3.3, F) }
pub fn flat8_3_5() -> WetClutch { build(3.5, F) }
pub fn flat8_4_0() -> WetClutch { build(4.0, F) }
pub fn flat8_4_5() -> WetClutch { build(4.5, F) }
pub fn flat8_4_9() -> WetClutch { build(4.9, F) }
pub fn flat8_5_0() -> WetClutch { build(5.0, F) }

pub fn v6_2_0() -> WetClutch { build(2.0, V) }
pub fn v6_2_3() -> WetClutch { build(2.3, V) }
pub fn v6_2_5() -> WetClutch { build(2.5, V) }
pub fn v6_2_7() -> WetClutch { build(2.7, V) }
pub fn v6_2_8() -> WetClutch { build(2.8, V) }
pub fn v6_3_0() -> WetClutch { build(3.0, V) }
pub fn v6_3_2() -> WetClutch { build(3.2, V) }
pub fn v6_3_5() -> WetClutch { build(3.5, V) }
pub fn v6_3_6() -> WetClutch { build(3.6, V) }
pub fn v6_3_7() -> WetClutch { build(3.7, V) }
pub fn v6_3_8() -> WetClutch { build(3.8, V) }
pub fn v6_4_0() -> WetClutch { build(4.0, V) }

pub fn v8_3_5() -> WetClutch { build(3.5, V) }
pub fn v8_3_9() -> WetClutch { build(3.9, V) }
pub fn v8_4_0() -> WetClutch { build(4.0, V) }
pub fn v8_4_4() -> WetClutch { build(4.4, V) }
pub fn v8_4_5() -> WetClutch { build(4.5, V) }
pub fn v8_4_6() -> WetClutch { build(4.6, V) }
pub fn v8_4_8() -> WetClutch { build(4.8, V) }
pub fn v8_5_0() -> WetClutch { build(5.0, V) }
pub fn v8_5_3() -> WetClutch { build(5.3, V) }
pub fn v8_5_5() -> WetClutch { build(5.5, V) }
pub fn v8_5_7() -> WetClutch { build(5.7, V) }
pub fn v8_6_0() -> WetClutch { build(6.0, V) }
pub fn v8_6_2() -> WetClutch { build(6.2, V) }
pub fn v8_6_4() -> WetClutch { build(6.4, V) }
pub fn v8_7_0() -> WetClutch { build(7.0, V) }

pub fn v10_4_0() -> WetClutch { build(4.0, V) }
pub fn v10_4_2() -> WetClutch { build(4.2, V) }
pub fn v10_5_0() -> WetClutch { build(5.0, V) }
pub fn v10_5_2() -> WetClutch { build(5.2, V) }
pub fn v10_5_7() -> WetClutch { build(5.7, V) }
pub fn v10_8_3() -> WetClutch { build(8.3, V) }
pub fn v10_8_4() -> WetClutch { build(8.4, V) }

pub fn v12_5_0() -> WetClutch { build(5.0, V) }
pub fn v12_5_5() -> WetClutch { build(5.5, V) }
pub fn v12_6_0() -> WetClutch { build(6.0, V) }
pub fn v12_6_2() -> WetClutch { build(6.2, V) }
pub fn v12_6_3() -> WetClutch { build(6.3, V) }
pub fn v12_6_5() -> WetClutch { build(6.5, V) }
pub fn v12_6_75() -> WetClutch { build(6.75, V) }
pub fn v12_7_0() -> WetClutch { build(7.0, V) }

pub fn v16_6_0() -> WetClutch { build(6.0, V) }
pub fn v16_6_75() -> WetClutch { build(6.75, V) }
pub fn v16_7_0() -> WetClutch { build(7.0, V) }
pub fn v16_7_4() -> WetClutch { build(7.4, V) }
pub fn v16_8_0() -> WetClutch { build(8.0, V) }
pub fn v16_8_4() -> WetClutch { build(8.4, V) }
pub fn v16_9_0() -> WetClutch { build(9.0, V) }
pub fn v16_13_6() -> WetClutch { build(13.6, V) }

pub fn w6_2_8() -> WetClutch { build(2.8, W) }
pub fn w6_3_2() -> WetClutch { build(3.2, W) }
pub fn w6_3_6() -> WetClutch { build(3.6, W) }

pub fn w8_3_6() -> WetClutch { build(3.6, W) }
pub fn w8_4_0() -> WetClutch { build(4.0, W) }
pub fn w8_4_2() -> WetClutch { build(4.2, W) }

pub fn w10_4_5() -> WetClutch { build(4.5, W) }
pub fn w10_5_0() -> WetClutch { build(5.0, W) }
pub fn w10_5_5() -> WetClutch { build(5.5, W) }

pub fn w12_5_0() -> WetClutch { build(5.0, W) }
pub fn w12_6_0() -> WetClutch { build(6.0, W) }
pub fn w12_6_3() -> WetClutch { build(6.3, W) }

pub fn w16_7_0() -> WetClutch { build(7.0, W) }
pub fn w16_8_0() -> WetClutch { build(8.0, W) }
pub fn w16_8_4() -> WetClutch { build(8.4, W) }

pub fn vr6_2_8() -> WetClutch { build(2.8, VR) }
pub fn vr6_3_2() -> WetClutch { build(3.2, VR) }
pub fn vr6_3_6() -> WetClutch { build(3.6, VR) }

pub fn vr8_4_0() -> WetClutch { build(4.0, VR) }
pub fn vr8_4_4() -> WetClutch { build(4.4, VR) }
