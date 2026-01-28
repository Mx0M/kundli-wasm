use std::f64::consts::PI;

/// True lunar node longitude (Rahu), radians, tropical
/// Based on Meeus + IAU formulation (JHora-compatible)
pub fn true_lunar_node(jd_tt: f64) -> f64 {
    let t = (jd_tt - 2451545.0) / 36525.0;

    // Mean longitude of ascending node (deg)
    let omega = 125.0445550 - 1934.1361849 * t + 0.0020762 * t * t + t * t * t / 467410.0
        - t * t * t * t / 60616000.0;

    // Fundamental arguments (deg)
    let d = 297.8501921 + 445267.1114034 * t;
    let m = 357.5291092 + 35999.0502909 * t;
    let mp = 134.9633964 + 477198.8675055 * t;
    let f = 93.2720950 + 483202.0175233 * t;

    // Convert to radians
    let omega = deg(omega);
    let d = deg(d);
    let m = deg(m);
    let mp = deg(mp);
    let f = deg(f);

    // True node correction (arcseconds)
    let delta = -1.4979 * (2.0 * d).sin() - 0.1500 * (m).sin() - 0.1226 * (2.0 * f).sin()
        + 0.1176 * (2.0 * mp).sin()
        - 0.0801 * (2.0 * d + m).sin();

    normalize(omega + arcsec(delta))
}

#[inline]
fn deg(x: f64) -> f64 {
    x * PI / 180.0
}

#[inline]
fn arcsec(x: f64) -> f64 {
    x * PI / (180.0 * 3600.0)
}

#[inline]
fn normalize(mut a: f64) -> f64 {
    a %= 2.0 * PI;
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}
