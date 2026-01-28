// Mean Moon longitude (Meeus / JHora-compatible)
// Tropical longitude, degrees

pub fn moon_mean_longitude_tropical(jd_tt: f64) -> f64 {
    let t = (jd_tt - 2451545.0) / 36525.0;

    let l0 = 218.3164477 + 481267.88123421 * t - 0.0015786 * t * t + t * t * t / 538841.0
        - t * t * t * t / 65194000.0;

    normalize(l0)
}

#[inline]
fn normalize(mut x: f64) -> f64 {
    x %= 360.0;
    if x < 0.0 {
        x += 360.0;
    }
    x
}
