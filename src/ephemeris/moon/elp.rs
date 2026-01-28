// src/ephemeris/moon/elp.rs

use crate::ephemeris::moon::elp_terms_main::MAIN_TERMS;
use crate::ephemeris::moon::elp_terms_plan::PLAN_TERMS;
use crate::ephemeris::moon::elp_terms_sec::SEC_TERMS;
use crate::ephemeris::moon::elp_types::ElpTerm;
use std::f64::consts::PI;

pub fn moon_longitude_mean(jd_tt: f64) -> f64 {
    let t = (jd_tt - 2451545.0) / 36525.0;

    // Fundamental arguments (Meeus / ELP)
    let d = deg(297.8501921 + 445267.1114034 * t);
    let m = deg(357.5291092 + 35999.0502909 * t);
    let mp = deg(134.9633964 + 477198.8675055 * t);
    let f = deg(93.2720950 + 483202.0175233 * t);

    let l0 = deg(218.3164477 + 481267.88123421 * t - 0.0015786 * t * t);

    let e = 1.0 - 0.002516 * t - 0.0000074 * t * t;

    let mut sum = 0.0;

    accumulate(&MAIN_TERMS, d, m, mp, f, e, &mut sum);
    accumulate(&PLAN_TERMS, d, m, mp, f, e, &mut sum);
    accumulate(&SEC_TERMS, d, m, mp, f, e, &mut sum);

    normalize(l0 + arcsec(sum))
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
