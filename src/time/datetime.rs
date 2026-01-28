use crate::time::delta_t::delta_t_seconds;
use crate::time::julian::calendar_to_jd;

/// Civil date-time input (local time)
#[derive(Debug, Clone, Copy)]
pub struct DateTimeInput {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: f64,

    /// Timezone offset from UTC (e.g. +5.5 for IST)
    pub tz_offset_hours: f64,
}

/// Convert local civil time to Julian Day (TT)
///
/// This is the ONLY function that should be used
/// to obtain JD_TT for ephemeris calculations.
pub fn jd_tt_from_datetime(input: DateTimeInput) -> f64 {
    // 1️⃣ Convert local time → UTC
    let utc_hours = input.hour as f64 + input.minute as f64 / 60.0 + input.second / 3600.0
        - input.tz_offset_hours;

    // Normalize UTC hours into [0, 24)
    let (day_adjust, utc_hours) = normalize_hours(utc_hours);

    // 2️⃣ Julian Day in UT
    let jd_ut = calendar_to_jd(
        input.year,
        input.month,
        (input.day as i32 + day_adjust) as u8,
        utc_hours,
    );

    // 3️⃣ ΔT (seconds)
    let delta_t = delta_t_seconds(jd_ut);

    // 4️⃣ Convert UT → TT
    jd_ut + delta_t / 86400.0
}

/// Normalize hours into 0–24 range, return day adjustment
fn normalize_hours(mut hours: f64) -> (i32, f64) {
    let mut day_adjust = 0;

    while hours < 0.0 {
        hours += 24.0;
        day_adjust -= 1;
    }

    while hours >= 24.0 {
        hours -= 24.0;
        day_adjust += 1;
    }

    (day_adjust, hours)
}
