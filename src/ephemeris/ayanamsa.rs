use crate::ephemeris::precession::general_precession_lon;
use std::f64::consts::PI;

/// Lahiri (Chitra Paksha) Ayanamsa
///
/// Input:
/// - jd_tt : Julian Day (TT)
///
/// Output:
/// - ayanamsa in radians (subtract from tropical longitude)
pub fn lahiri_ayanamsa(jd_tt: f64) -> f64 {
    // Lahiri ayanamsa at J2000.0 (arcseconds)
    // Value used by JHora / Swiss Ephemeris
    const LAHIRI_J2000_ARCSEC: f64 = 858.0 * 60.0 + 45.0; // 23° 51′ 45″

    let ayan_j2000 = arcsec_to_rad(LAHIRI_J2000_ARCSEC);

    // Precession since J2000
    let precession = general_precession_lon(jd_tt);

    ayan_j2000 + precession
}

#[inline]
fn arcsec_to_rad(x: f64) -> f64 {
    x * PI / (180.0 * 3600.0)
}
