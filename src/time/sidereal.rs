use std::f64::consts::TAU;

/// Greenwich Mean Sidereal Time (radians)
///
/// Input:
/// - jd_ut : Julian Day (UT)
///
/// Output:
/// - GMST in radians (0 .. 2π)
pub fn greenwich_sidereal_time(jd_ut: f64) -> f64 {
    let t = (jd_ut - 2451545.0) / 36525.0;

    // GMST in seconds (Meeus)
    let gmst_sec = 67310.54841 + (876600.0 * 3600.0 + 8640184.812866) * t + 0.093104 * t * t
        - 6.2e-6 * t * t * t;

    // Convert seconds → radians
    let gmst_rad = (gmst_sec / 86400.0) * TAU;

    normalize(gmst_rad)
}

/// Local Sidereal Time (radians)
///
/// Input:
/// - jd_ut        : Julian Day (UT)
/// - longitude_rad : observer longitude (east positive)
pub fn local_sidereal_time(jd_ut: f64, longitude_rad: f64) -> f64 {
    normalize(greenwich_sidereal_time(jd_ut) + longitude_rad)
}

#[inline]
fn normalize(mut a: f64) -> f64 {
    a %= TAU;
    if a < 0.0 {
        a += TAU;
    }
    a
}
