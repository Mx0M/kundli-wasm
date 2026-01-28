use crate::ephemeris::moon::elp_terms_main::MAIN_TERMS;
use crate::ephemeris::moon::elp_terms_plan::PLAN_TERMS;
use crate::ephemeris::moon::elp_types::ElpTerm;
use std::f64::consts::PI;

/// Returns geocentric ecliptic longitude of Moon (radians, mean equinox of date)
pub fn moon_longitude_elp(jd_tt: f64) -> f64 {
    let t = (jd_tt - 2451545.0) / 36525.0;

    // ELP82B fundamental arguments
    let d = deg(297.85036 + 445267.111480 * t - 0.0019142 * t * t + t * t * t / 189474.0);
    let m = deg(357.52772 + 35999.050340 * t - 0.0001603 * t * t - t * t * t / 300000.0);
    let mp = deg(134.96298 + 477198.867398 * t + 0.0086972 * t * t + t * t * t / 56250.0);
    let f = deg(93.27191 + 483202.017538 * t - 0.0036825 * t * t + t * t * t / 327270.0);

    // Mean longitude of the Moon
    let l0 = deg(218.3164477 + 481267.88123421 * t - 0.0015786 * t * t);

    let mut sum_arcsec = 0.0;
    accumulate(&MAIN_TERMS, d, m, mp, f, t, &mut sum_arcsec);
    accumulate(&PLAN_TERMS, d, m, mp, f, t, &mut sum_arcsec);

    normalize(l0 + arcsec(sum_arcsec))
}

fn accumulate(terms: &[ElpTerm], d: f64, m: f64, mp: f64, f: f64, t: f64, acc: &mut f64) {
    for term in terms {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;

        let coeff = term.a0 + term.a1 * t;

        *acc += coeff * arg.sin();
    }
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
