use std::f64::consts::PI;

/// General precession in longitude (arcseconds)
/// IAU 2006 model, valid for several centuries
///
/// Input:
/// - jd_tt : Julian Day (TT)
///
/// Output:
/// - precession in longitude (radians)
pub fn general_precession_lon(jd_tt: f64) -> f64 {
    let t = (jd_tt - 2451545.0) / 36525.0;

    // IAU 2006 precession in longitude (arcsec)
    let psi = 5028.796195 * t + 1.1054348 * t * t + 0.00007964 * t * t * t;

    arcsec_to_rad(psi)
}

#[inline]
fn arcsec_to_rad(x: f64) -> f64 {
    x * PI / (180.0 * 3600.0)
}
