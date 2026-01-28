// src/frames/ayanamsa.rs
//
// Lahiri (Chitra Paksha) Ayanamsa
//
// Reference:
// - Lahiri ayanamsa ≈ 23°51′ at J2000
// - Based on IAU precession in longitude
//
// All angles are in RADIANS.

use std::f64::consts::PI;

/// Compute Lahiri ayanamsa (radians) for given Julian Day (TT)
pub fn lahiri_ayanamsa(jd_tt: f64) -> f64 {
    // Julian centuries from J2000
    let t = (jd_tt - 2451545.0) / 36525.0;

    // Lahiri ayanamsa at J2000 (arcseconds)
    // 23° 51′ 00.00″ = 23.85°
    // = 85860 arcseconds
    let ayan_0 = 85860.0;

    // Annual precession in longitude (arcseconds)
    // IAU 1976 / Meeus
    let ayan_arcsec = ayan_0 + 5029.0966 * t + 1.11113 * t * t - 0.000006 * t * t * t;

    arcsec_to_rad(ayan_arcsec)
}

/// Apply Lahiri ayanamsa to a tropical longitude
///
/// λ_sidereal = λ_tropical − ayanamsa
pub fn to_sidereal(lambda_tropical: f64, jd_tt: f64) -> f64 {
    let ayan = lahiri_ayanamsa(jd_tt);
    normalize_angle(lambda_tropical - ayan)
}

/// Normalize angle to [0, 2π)
#[inline]
fn normalize_angle(mut a: f64) -> f64 {
    a %= 2.0 * PI;
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}

/// Arcseconds → radians
#[inline]
fn arcsec_to_rad(a: f64) -> f64 {
    a * PI / (180.0 * 3600.0)
}
