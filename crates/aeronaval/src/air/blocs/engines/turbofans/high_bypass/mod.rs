pub mod sn {
    pub struct D { pub thrust_kn: f64, pub bypass_ratio: f64, pub fan_diameter_m: f64 }
    pub mod cfm56_5b4 { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 5.9, fan_diameter_m: 1.735 }; }
    pub mod cfm56_5b7 { pub const D: super::D = super::D { thrust_kn: 133.4, bypass_ratio: 5.7, fan_diameter_m: 1.735 }; }
    pub mod cfm56_7b24 { pub const D: super::D = super::D { thrust_kn: 107.0, bypass_ratio: 5.1, fan_diameter_m: 1.549 }; }
    pub mod cfm56_7b27 { pub const D: super::D = super::D { thrust_kn: 121.4, bypass_ratio: 5.1, fan_diameter_m: 1.549 }; }
    pub mod v2527_a5 { pub const D: super::D = super::D { thrust_kn: 120.1, bypass_ratio: 4.6, fan_diameter_m: 1.613 }; }
    pub mod v2533_a5 { pub const D: super::D = super::D { thrust_kn: 146.8, bypass_ratio: 4.6, fan_diameter_m: 1.613 }; }
    pub mod ge90_76b { pub const D: super::D = super::D { thrust_kn: 339.3, bypass_ratio: 8.4, fan_diameter_m: 3.124 }; }
    pub mod ge90_115b { pub const D: super::D = super::D { thrust_kn: 513.0, bypass_ratio: 9.0, fan_diameter_m: 3.251 }; }
    pub mod trent_700 { pub const D: super::D = super::D { thrust_kn: 300.3, bypass_ratio: 5.0, fan_diameter_m: 2.474 }; }
    pub mod trent_800 { pub const D: super::D = super::D { thrust_kn: 413.7, bypass_ratio: 6.5, fan_diameter_m: 2.794 }; }
    pub mod trent_1000 { pub const D: super::D = super::D { thrust_kn: 320.0, bypass_ratio: 10.8, fan_diameter_m: 2.845 }; }
    pub mod pw4056 { pub const D: super::D = super::D { thrust_kn: 252.4, bypass_ratio: 5.0, fan_diameter_m: 2.438 }; }
    pub mod pw4090 { pub const D: super::D = super::D { thrust_kn: 408.2, bypass_ratio: 6.4, fan_diameter_m: 2.845 }; }
    pub mod cf6_80c2 { pub const D: super::D = super::D { thrust_kn: 276.8, bypass_ratio: 5.3, fan_diameter_m: 2.362 }; }
    pub mod cf6_80e1 { pub const D: super::D = super::D { thrust_kn: 300.3, bypass_ratio: 5.8, fan_diameter_m: 2.438 }; }
    pub mod genx_1b76 { pub const D: super::D = super::D { thrust_kn: 339.3, bypass_ratio: 9.3, fan_diameter_m: 2.819 }; }
    pub mod genx_2b67 { pub const D: super::D = super::D { thrust_kn: 296.0, bypass_ratio: 8.0, fan_diameter_m: 2.616 }; }
    pub mod trent_xwb_84 { pub const D: super::D = super::D { thrust_kn: 374.0, bypass_ratio: 9.3, fan_diameter_m: 3.0 }; }
    pub mod trent_xwb_97 { pub const D: super::D = super::D { thrust_kn: 430.0, bypass_ratio: 9.6, fan_diameter_m: 3.0 }; }
}

pub mod standard;
pub mod long_range;

use crate::components::powertrain::engines::turbines::assemblies::fan::turbofan::Turbofan;

pub fn all() -> Vec<Turbofan> {
    let mut v = Vec::new();
    v.extend(standard::all());
    v.extend(long_range::all());
    v
}
