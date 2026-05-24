#[derive(Debug, Clone, Copy)]
pub struct AntoineCoeffs {
    pub formula: &'static str,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub t_min_k: f64,
    pub t_max_k: f64,
}

pub const TABLE: &[AntoineCoeffs] = &[
    AntoineCoeffs { formula: "H2O",    a: 8.07131, b: 1730.630, c: 233.426, t_min_k: 274.0, t_max_k: 373.0 },
    AntoineCoeffs { formula: "CH3OH",  a: 8.08097, b: 1582.271, c: 239.726, t_min_k: 257.0, t_max_k: 364.0 },
    AntoineCoeffs { formula: "C2H5OH", a: 8.20417, b: 1642.890, c: 230.300, t_min_k: 270.0, t_max_k: 369.0 },
    AntoineCoeffs { formula: "C3H6O",  a: 7.11714, b: 1210.595, c: 229.664, t_min_k: 259.0, t_max_k: 380.0 },
    AntoineCoeffs { formula: "C6H6",   a: 6.90565, b: 1211.033, c: 220.790, t_min_k: 281.0, t_max_k: 377.0 },
    AntoineCoeffs { formula: "C7H8",   a: 6.95464, b: 1344.800, c: 219.482, t_min_k: 286.0, t_max_k: 410.0 },
    AntoineCoeffs { formula: "C8H10",  a: 6.99052, b: 1453.430, c: 215.310, t_min_k: 300.0, t_max_k: 440.0 },
    AntoineCoeffs { formula: "C5H12",  a: 6.85221, b: 1064.630, c: 232.000, t_min_k: 220.0, t_max_k: 330.0 },
    AntoineCoeffs { formula: "C6H14",  a: 6.87601, b: 1171.170, c: 224.410, t_min_k: 245.0, t_max_k: 370.0 },
    AntoineCoeffs { formula: "C7H16",  a: 6.89386, b: 1264.370, c: 216.640, t_min_k: 270.0, t_max_k: 400.0 },
    AntoineCoeffs { formula: "C8H18",  a: 6.91874, b: 1351.990, c: 209.150, t_min_k: 290.0, t_max_k: 425.0 },
    AntoineCoeffs { formula: "CH4",    a: 6.61184, b: 389.930,  c: 266.000, t_min_k: 90.0,  t_max_k: 190.0 },
    AntoineCoeffs { formula: "C2H6",   a: 6.95335, b: 699.106,  c: 260.264, t_min_k: 130.0, t_max_k: 199.0 },
    AntoineCoeffs { formula: "C3H8",   a: 6.80398, b: 803.810,  c: 246.990, t_min_k: 164.0, t_max_k: 250.0 },
    AntoineCoeffs { formula: "C4H10",  a: 6.80896, b: 935.860,  c: 238.730, t_min_k: 196.0, t_max_k: 290.0 },
    AntoineCoeffs { formula: "C2H4",   a: 6.74756, b: 585.000,  c: 255.000, t_min_k: 120.0, t_max_k: 182.0 },
    AntoineCoeffs { formula: "C2H2",   a: 7.09990, b: 711.000,  c: 253.400, t_min_k: 192.0, t_max_k: 208.0 },
    AntoineCoeffs { formula: "NH3",    a: 7.55466, b: 1002.711, c: 247.885, t_min_k: 196.0, t_max_k: 320.0 },
    AntoineCoeffs { formula: "CO2",    a: 7.81024, b: 995.705,  c: 293.475, t_min_k: 154.0, t_max_k: 204.0 },
    AntoineCoeffs { formula: "SO2",    a: 7.32776, b: 999.900,  c: 237.190, t_min_k: 195.0, t_max_k: 280.0 },
    AntoineCoeffs { formula: "H2S",    a: 7.11962, b: 802.227,  c: 249.610, t_min_k: 190.0, t_max_k: 230.0 },
    AntoineCoeffs { formula: "Cl2",    a: 6.93790, b: 861.340,  c: 246.330, t_min_k: 195.0, t_max_k: 263.0 },
    AntoineCoeffs { formula: "HCl",    a: 7.17570, b: 745.800,  c: 258.880, t_min_k: 137.0, t_max_k: 230.0 },
    AntoineCoeffs { formula: "CHCl3",  a: 6.90328, b: 1163.030, c: 227.400, t_min_k: 260.0, t_max_k: 370.0 },
    AntoineCoeffs { formula: "CCl4",   a: 6.87926, b: 1212.021, c: 226.410, t_min_k: 280.0, t_max_k: 350.0 },
    AntoineCoeffs { formula: "CH2Cl2", a: 7.40916, b: 1325.938, c: 252.616, t_min_k: 233.0, t_max_k: 313.0 },
    AntoineCoeffs { formula: "Hg",     a: 7.81470, b: 3216.700, c: 0.000,   t_min_k: 400.0, t_max_k: 800.0 },
    AntoineCoeffs { formula: "Br2",    a: 6.87780, b: 1119.680, c: 221.380, t_min_k: 224.0, t_max_k: 332.0 },
    AntoineCoeffs { formula: "I2",     a: 7.01840, b: 1610.900, c: 205.000, t_min_k: 322.0, t_max_k: 458.0 },
    AntoineCoeffs { formula: "CS2",    a: 6.94279, b: 1169.110, c: 241.590, t_min_k: 277.0, t_max_k: 353.0 },
    AntoineCoeffs { formula: "C2H4O2", a: 7.55716, b: 1642.540, c: 233.385, t_min_k: 290.0, t_max_k: 392.0 },
    AntoineCoeffs { formula: "HCN",    a: 7.52800, b: 1329.500, c: 260.400, t_min_k: 256.0, t_max_k: 299.0 },
    AntoineCoeffs { formula: "C3H8O3", a: 6.16500, b: 1036.300, c: 28.000,  t_min_k: 415.0, t_max_k: 563.0 },
];

impl AntoineCoeffs {
    pub fn vapor_pressure_pa(&self, temperature_k: f64) -> f64 {
        let t_c = temperature_k - 273.15;
        let log10_p_mmhg = self.a - self.b / (t_c + self.c);
        let p_mmhg = 10.0_f64.powf(log10_p_mmhg);
        p_mmhg * 133.322
    }
}

pub fn by_formula(formula: &str) -> Option<&'static AntoineCoeffs> {
    TABLE.iter().find(|a| a.formula == formula)
}
