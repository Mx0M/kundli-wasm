use crate::time::delta_t::delta_t;

/// Convert Gregorian calendar date to Julian Day (UT)
///
/// Inputs:
/// - year  : i32
/// - month : u8 (1–12)
/// - day   : u8 (1–31)
/// - hour  : f64 (fractional hours, UTC)
///
/// Output:
/// - Julian Day (UT)
///
/// Valid for all Gregorian dates (post 1582)
pub fn calendar_to_jd(year: i32, month: u8, day: u8, hour: f64) -> f64 {
    let mut y = year as i32;
    let mut m = month as i32;

    if m <= 2 {
        y -= 1;
        m += 12;
    }

    let a = (y / 100) as i32;
    let b = 2 - a + (a / 4);

    let jd_day = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * ((m + 1) as f64)).floor()
        + day as f64
        + b as f64
        - 1524.5;

    jd_day + hour / 24.0
}
pub fn jd_ut_from_tt(jd_tt: f64) -> f64 {
    let delta_seconds = delta_t(jd_tt);
    jd_tt - delta_seconds / 86400.0
}
