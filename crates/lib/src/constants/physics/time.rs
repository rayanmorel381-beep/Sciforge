pub const NANOSECOND: f64 = 1.0e-9;
pub const MICROSECOND: f64 = 1.0e-6;
pub const MILLISECOND: f64 = 1.0e-3;
pub const SECOND: f64 = 1.0;
pub const MINUTE: f64 = 60.0;
pub const HOUR: f64 = 3_600.0;
pub const DAY: f64 = 86_400.0;
pub const WEEK: f64 = 7.0 * DAY;

pub const SIDEREAL_DAY: f64 = 86_164.090_530_833;
pub const STELLAR_DAY: f64 = 86_164.098_903_691;

pub const JULIAN_YEAR: f64 = 365.25 * DAY;
pub const GREGORIAN_YEAR: f64 = 365.2425 * DAY;
pub const TROPICAL_YEAR: f64 = 3.155_693e7;
pub const SIDEREAL_YEAR: f64 = 3.155_815e7;
pub const ANOMALISTIC_YEAR: f64 = 3.155_843_404e7;

pub const SYNODIC_MONTH: f64 = 29.530_588_861 * DAY;
pub const SIDEREAL_MONTH: f64 = 27.321_661_55 * DAY;
pub const ANOMALISTIC_MONTH: f64 = 27.554_549_878 * DAY;
pub const DRACONIC_MONTH: f64 = 27.212_220_817 * DAY;

pub const JULIAN_CENTURY: f64 = 36_525.0 * DAY;
pub const JULIAN_MILLENNIUM: f64 = 365_250.0 * DAY;
pub const JULIAN_GIGAYEAR: f64 = JULIAN_YEAR * 1.0e9;

pub const SECONDS_PER_NANOSECOND: f64 = 1.0e-9;
pub const SECONDS_PER_MICROSECOND: f64 = 1.0e-6;
pub const SECONDS_PER_MILLISECOND: f64 = 1.0e-3;
pub const SECONDS_PER_MINUTE: f64 = 60.0;
pub const SECONDS_PER_HOUR: f64 = 3_600.0;
pub const SECONDS_PER_DAY: f64 = 86_400.0;
pub const SECONDS_PER_WEEK: f64 = 7.0 * SECONDS_PER_DAY;
pub const SECONDS_PER_JULIAN_YEAR: f64 = JULIAN_YEAR;

pub const PER_NANOSECOND: f64 = 1.0e9;
pub const PER_MICROSECOND: f64 = 1.0e6;
pub const PER_MILLISECOND: f64 = 1.0e3;
pub const PER_MINUTE: f64 = 1.0 / MINUTE;
pub const PER_HOUR: f64 = 1.0 / HOUR;
pub const PER_DAY: f64 = 1.0 / DAY;
pub const PER_JULIAN_YEAR: f64 = 1.0 / JULIAN_YEAR;

pub const JULIAN_CENTURY_DAYS: f64 = 36_525.0;
pub const JULIAN_MILLENNIUM_DAYS: f64 = 365_250.0;
pub const JULIAN_YEAR_DAYS: f64 = 365.25;

pub const J2000_EPOCH_JD: f64 = 2_451_545.0;
pub const J1900_EPOCH_JD: f64 = 2_415_020.0;
pub const B1950_EPOCH_JD: f64 = 2_433_282.423_5;
pub const UNIX_EPOCH_JD: f64 = 2_440_587.5;
pub const MJD_OFFSET: f64 = 2_400_000.5;
