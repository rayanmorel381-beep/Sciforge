pub fn and_gate(a: bool, b: bool) -> bool {
    a && b
}
pub fn or_gate(a: bool, b: bool) -> bool {
    a || b
}
pub fn not_gate(a: bool) -> bool {
    !a
}
pub fn nand_gate(a: bool, b: bool) -> bool {
    !(a && b)
}
pub fn nor_gate(a: bool, b: bool) -> bool {
    !(a || b)
}
pub fn xor_gate(a: bool, b: bool) -> bool {
    a ^ b
}
pub fn xnor_gate(a: bool, b: bool) -> bool {
    !(a ^ b)
}

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (a ^ b, a && b)
}

pub fn full_adder(a: bool, b: bool, cin: bool) -> (bool, bool) {
    let (s1, c1) = half_adder(a, b);
    let (sum, c2) = half_adder(s1, cin);
    (sum, c1 || c2)
}

pub fn ripple_carry_adder(a: &[bool], b: &[bool]) -> (Vec<bool>, bool) {
    let n = a.len();
    let mut sum = vec![false; n];
    let mut carry = false;
    for i in 0..n {
        let (s, c) = full_adder(a[i], b[i], carry);
        sum[i] = s;
        carry = c;
    }
    (sum, carry)
}

pub fn multiplexer_2to1(a: bool, b: bool, sel: bool) -> bool {
    if sel { b } else { a }
}

pub fn demultiplexer_1to2(input: bool, sel: bool) -> (bool, bool) {
    if sel { (false, input) } else { (input, false) }
}

pub fn decoder_2to4(a: bool, b: bool) -> [bool; 4] {
    let mut out = [false; 4];
    let idx = (b as usize) * 2 + a as usize;
    out[idx] = true;
    out
}

pub fn encoder_4to2(inputs: &[bool; 4]) -> (bool, bool) {
    for i in (0..4).rev() {
        if inputs[i] {
            return (i & 1 != 0, i & 2 != 0);
        }
    }
    (false, false)
}

pub fn sr_latch(s: bool, r: bool, q_prev: bool) -> bool {
    if s && !r {
        true
    } else if !s && r {
        false
    } else {
        q_prev
    }
}

pub fn d_flip_flop(d: bool, _: bool) -> bool {
    d
}

pub fn jk_flip_flop(j: bool, k: bool, q_prev: bool) -> bool {
    match (j, k) {
        (false, false) => q_prev,
        (false, true) => false,
        (true, false) => true,
        (true, true) => !q_prev,
    }
}

pub fn binary_to_gray(binary: u32) -> u32 {
    binary ^ (binary >> 1)
}

pub fn gray_to_binary(gray: u32) -> u32 {
    let mut binary = gray;
    let mut mask = gray >> 1;
    while mask != 0 {
        binary ^= mask;
        mask >>= 1;
    }
    binary
}

pub fn ones_complement(val: u32, bits: u32) -> u32 {
    val ^ ((1 << bits) - 1)
}

pub fn twos_complement(val: u32, bits: u32) -> u32 {
    (ones_complement(val, bits)).wrapping_add(1) & ((1 << bits) - 1)
}
