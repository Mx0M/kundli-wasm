// src/frames/nutation.rs
//
// Nutation according to IAU 1980 theory (Meeus).
// Accuracy: ~1 arcsecond (sufficient for astrology).
//
// All angles are in RADIANS unless stated otherwise.

use std::f64::consts::PI;

/// Nutation result
#[derive(Clone, Copy, Debug)]
pub struct Nutation {
    pub delta_psi: f64, // nutation in longitude (radians)
    pub delta_eps: f64, // nutation in obliquity (radians)
    pub eps_mean: f64,  // mean obliquity (radians)
    pub eps_true: f64,  // true obliquity (radians)
}

/// Compute nutation quantities for given Julian Day (TT)
pub fn nutation(jd_tt: f64) -> Nutation {
    let t = (jd_tt - 2451545.0) / 36525.0;

    // Fundamental arguments (Meeus, in degrees)
    let d = deg_to_rad(297.85036 + 445267.111480 * t - 0.0019142 * t * t + t * t * t / 189474.0);

    let m = deg_to_rad(357.52772 + 35999.050340 * t - 0.0001603 * t * t - t * t * t / 300000.0);

    let mp = deg_to_rad(134.96298 + 477198.867398 * t + 0.0086972 * t * t + t * t * t / 56250.0);

    let f = deg_to_rad(93.27191 + 483202.017538 * t - 0.0036825 * t * t + t * t * t / 327270.0);

    let omega = deg_to_rad(125.04452 - 1934.136261 * t + 0.0020708 * t * t + t * t * t / 450000.0);

    // Nutation series (truncated, Meeus Table 22.A)
    // Coefficients are in 0.0001 arcseconds
    let terms = [
        (0, 0, 0, 0, 1, -171996.0, -174.2, 92025.0, 8.9),
        (0, 0, 2, -2, 2, -13187.0, -1.6, 5736.0, -3.1),
        (0, 0, 2, 0, 2, -2274.0, -0.2, 977.0, -0.5),
        (0, 0, 0, 0, 2, 2062.0, 0.2, -895.0, 0.5),
        (0, 1, 0, 0, 0, 1426.0, -3.4, 54.0, -0.1),
        (1, 0, 0, 0, 0, 712.0, 0.1, -7.0, 0.0),
        (0, 1, 2, -2, 2, -517.0, 1.2, 224.0, -0.6),
        (0, 0, 2, 0, 1, -386.0, -0.4, 200.0, 0.0),
        (1, 0, 2, 0, 2, -301.0, 0.0, 129.0, -0.1),
        (0, -1, 2, -2, 2, 217.0, -0.5, -95.0, 0.3),
    ];

    let mut delta_psi = 0.0;
    let mut delta_eps = 0.0;

    for (d_m, m_m, mp_m, f_m, om_m, psi0, psi1, eps0, eps1) in terms {
        let arg = d_m as f64 * d
            + m_m as f64 * m
            + mp_m as f64 * mp
            + f_m as f64 * f
            + om_m as f64 * omega;

        delta_psi += (psi0 + psi1 * t) * arg.sin();
        delta_eps += (eps0 + eps1 * t) * arg.cos();
    }

    // Convert from 0.0001 arcsec → radians
    delta_psi *= 1.0e-4 * arcsec_to_rad(1.0);
    delta_eps *= 1.0e-4 * arcsec_to_rad(1.0);

    // Mean obliquity (arcseconds → radians)
    let eps_mean = arcsec_to_rad(84381.448 - 46.8150 * t - 0.00059 * t * t + 0.001813 * t * t * t);

    let eps_true = eps_mean + delta_eps;

    Nutation {
        delta_psi,
        delta_eps,
        eps_mean,
        eps_true,
    }
}

/// Degrees → radians
#[inline]
fn deg_to_rad(d: f64) -> f64 {
    d * PI / 180.0
}

/// Arcseconds → radians
#[inline]
fn arcsec_to_rad(a: f64) -> f64 {
    a * PI / (180.0 * 3600.0)
}
