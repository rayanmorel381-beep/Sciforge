#[derive(Debug, Clone, Copy)]
pub struct Hardness {
    pub material: &'static str,
    pub vickers_hv: Option<f64>,
    pub brinell_hb: Option<f64>,
    pub mohs: Option<f64>,
}

pub const TABLE: &[Hardness] = &[
    Hardness { material: "Talc",        vickers_hv: None,         brinell_hb: None,        mohs: Some( 1.0) },
    Hardness { material: "Gypsum",      vickers_hv: None,         brinell_hb: None,        mohs: Some( 2.0) },
    Hardness { material: "Calcite",     vickers_hv: None,         brinell_hb: None,        mohs: Some( 3.0) },
    Hardness { material: "Fluorite",    vickers_hv: None,         brinell_hb: None,        mohs: Some( 4.0) },
    Hardness { material: "Apatite",     vickers_hv: None,         brinell_hb: None,        mohs: Some( 5.0) },
    Hardness { material: "Orthoclase",  vickers_hv: None,         brinell_hb: None,        mohs: Some( 6.0) },
    Hardness { material: "Quartz",      vickers_hv: Some(1100.0), brinell_hb: None,        mohs: Some( 7.0) },
    Hardness { material: "Topaz",       vickers_hv: Some(1648.0), brinell_hb: None,        mohs: Some( 8.0) },
    Hardness { material: "Corundum",    vickers_hv: Some(2000.0), brinell_hb: None,        mohs: Some( 9.0) },
    Hardness { material: "Diamond",     vickers_hv: Some(10000.0),brinell_hb: None,        mohs: Some(10.0) },
    Hardness { material: "Pb",          vickers_hv: Some(   5.0), brinell_hb: Some(  4.0), mohs: Some(1.5) },
    Hardness { material: "Sn",          vickers_hv: Some(   5.0), brinell_hb: Some(  3.9), mohs: Some(1.5) },
    Hardness { material: "Au",          vickers_hv: Some(  25.0), brinell_hb: Some( 25.0), mohs: Some(2.5) },
    Hardness { material: "Ag",          vickers_hv: Some(  25.0), brinell_hb: Some( 24.5), mohs: Some(2.5) },
    Hardness { material: "Al",          vickers_hv: Some(  25.0), brinell_hb: Some( 15.0), mohs: Some(2.75) },
    Hardness { material: "Cu",          vickers_hv: Some(  50.0), brinell_hb: Some( 35.0), mohs: Some(3.0) },
    Hardness { material: "Mg",          vickers_hv: Some(  44.0), brinell_hb: Some( 35.0), mohs: Some(2.5) },
    Hardness { material: "Zn",          vickers_hv: Some(  35.0), brinell_hb: Some( 30.0), mohs: Some(2.5) },
    Hardness { material: "Fe",          vickers_hv: Some(  60.0), brinell_hb: Some( 70.0), mohs: Some(4.0) },
    Hardness { material: "Ni",          vickers_hv: Some( 638.0), brinell_hb: Some(700.0), mohs: Some(4.0) },
    Hardness { material: "Pt",          vickers_hv: Some(  56.0), brinell_hb: Some( 39.0), mohs: Some(3.5) },
    Hardness { material: "Ti",          vickers_hv: Some( 970.0), brinell_hb: Some(716.0), mohs: Some(6.0) },
    Hardness { material: "Cr",          vickers_hv: Some(1060.0), brinell_hb: Some(687.0), mohs: Some(8.5) },
    Hardness { material: "W",           vickers_hv: Some(3430.0), brinell_hb: Some(2570.0),mohs: Some(7.5) },
    Hardness { material: "Mo",          vickers_hv: Some(1530.0), brinell_hb: Some(1500.0),mohs: Some(5.5) },
    Hardness { material: "AISI_1045",   vickers_hv: Some( 200.0), brinell_hb: Some(200.0), mohs: None },
    Hardness { material: "AISI_4140",   vickers_hv: Some( 302.0), brinell_hb: Some(285.0), mohs: None },
    Hardness { material: "AISI_316",    vickers_hv: Some( 217.0), brinell_hb: Some(180.0), mohs: None },
    Hardness { material: "Al_6061_T6",  vickers_hv: Some( 107.0), brinell_hb: Some( 95.0), mohs: None },
    Hardness { material: "Al_7075_T6",  vickers_hv: Some( 175.0), brinell_hb: Some(150.0), mohs: None },
    Hardness { material: "Ti_6Al_4V",   vickers_hv: Some( 349.0), brinell_hb: Some(334.0), mohs: None },
    Hardness { material: "Inconel_718", vickers_hv: Some( 380.0), brinell_hb: Some(365.0), mohs: None },
    Hardness { material: "WC",          vickers_hv: Some(2600.0), brinell_hb: None,        mohs: Some(9.0) },
    Hardness { material: "SiC",         vickers_hv: Some(2800.0), brinell_hb: None,        mohs: Some(9.5) },
    Hardness { material: "Al2O3",       vickers_hv: Some(2000.0), brinell_hb: None,        mohs: Some(9.0) },
    Hardness { material: "Si3N4",       vickers_hv: Some(1700.0), brinell_hb: None,        mohs: Some(9.0) },
    Hardness { material: "BN_cubic",    vickers_hv: Some(4500.0), brinell_hb: None,        mohs: Some(9.5) },
];

pub fn by_material(material: &str) -> Option<&'static Hardness> {
    TABLE.iter().find(|h| h.material == material)
}
