/// Compute Delta-T = TT − UT (seconds)
///
/// Input:
/// - jd_ut : Julian Day (UT)
///
/// Output:
/// - ΔT in seconds
///
/// Source:
/// - NASA / Espenak & Meeus polynomial fits
pub fn delta_t_seconds(jd_ut: f64) -> f64 {
    let y = jd_to_decimal_year(jd_ut);

    if y < -500.0 {
        delta_t_before_500bc(y)
    } else if y < 500.0 {
        delta_t_500bc_to_500ad(y)
    } else if y < 1600.0 {
        delta_t_500ad_to_1600(y)
    } else if y < 1700.0 {
        delta_t_1600_to_1700(y)
    } else if y < 1800.0 {
        delta_t_1700_to_1800(y)
    } else if y < 1860.0 {
        delta_t_1800_to_1860(y)
    } else if y < 1900.0 {
        delta_t_1860_to_1900(y)
    } else if y < 1920.0 {
        delta_t_1900_to_1920(y)
    } else if y < 1941.0 {
        delta_t_1920_to_1941(y)
    } else if y < 1961.0 {
        delta_t_1941_to_1961(y)
    } else if y < 1986.0 {
        delta_t_1961_to_1986(y)
    } else if y < 2005.0 {
        delta_t_1986_to_2005(y)
    } else if y < 2050.0 {
        delta_t_2005_to_2050(y)
    } else if y < 2150.0 {
        delta_t_2050_to_2150(y)
    } else {
        delta_t_after_2150(y)
    }
}

fn jd_to_decimal_year(jd: f64) -> f64 {
    2000.0 + (jd - 2451545.0) / 365.25
}

// ---- polynomial segments ----

fn delta_t_2005_to_2050(y: f64) -> f64 {
    let t = y - 2000.0;
    62.92 + 0.32217 * t + 0.005589 * t * t
}

fn delta_t_1986_to_2005(y: f64) -> f64 {
    let t = y - 2000.0;
    63.86 + 0.3345 * t - 0.060374 * t * t
        + 0.0017275 * t * t * t
        + 0.000651814 * t * t * t * t
        + 0.00002373599 * t * t * t * t * t
}

fn delta_t_1961_to_1986(y: f64) -> f64 {
    let t = y - 1975.0;
    45.45 + 1.067 * t - t * t / 260.0
}

fn delta_t_1941_to_1961(y: f64) -> f64 {
    let t = y - 1950.0;
    29.07 + 0.407 * t - t * t / 233.0 + t * t * t / 2547.0
}

fn delta_t_1920_to_1941(y: f64) -> f64 {
    let t = y - 1920.0;
    21.20 + 0.84493 * t - 0.076100 * t * t + 0.0020936 * t * t * t
}

fn delta_t_1900_to_1920(y: f64) -> f64 {
    let t = y - 1900.0;
    -2.79 + 1.494119 * t - 0.0598939 * t * t + 0.0061966 * t * t * t
}

fn delta_t_1860_to_1900(y: f64) -> f64 {
    let t = y - 1860.0;
    7.62 + 0.5737 * t - 0.251754 * t * t + 0.01680668 * t * t * t - 0.0004473624 * t * t * t * t
}

fn delta_t_1800_to_1860(y: f64) -> f64 {
    let t = y - 1800.0;
    13.72 - 0.332447 * t + 0.0068612 * t * t + 0.0041116 * t * t * t - 0.00037436 * t * t * t * t
}

fn delta_t_1700_to_1800(y: f64) -> f64 {
    let t = y - 1700.0;
    8.83 + 0.1603 * t - 0.0059285 * t * t + 0.00013336 * t * t * t - t * t * t * t / 1174000.0
}

fn delta_t_1600_to_1700(y: f64) -> f64 {
    let t = y - 1600.0;
    120.0 - 0.9808 * t - 0.01532 * t * t + t * t * t / 7129.0
}

fn delta_t_500ad_to_1600(y: f64) -> f64 {
    let t = (y - 1000.0) / 100.0;
    1574.2 - 556.01 * t + 71.23472 * t * t + 0.319781 * t * t * t
}

fn delta_t_500bc_to_500ad(y: f64) -> f64 {
    let t = y / 100.0;
    10583.6 - 1014.41 * t + 33.78311 * t * t
}

fn delta_t_before_500bc(y: f64) -> f64 {
    let t = (y - 1820.0) / 100.0;
    -20.0 + 32.0 * t * t
}

fn delta_t_after_2150(y: f64) -> f64 {
    let t = (y - 1820.0) / 100.0;
    -20.0 + 32.0 * t * t
}
fn delta_t_2050_to_2150(y: f64) -> f64 {
    let t = (y - 1820.0) / 100.0;
    -20.0 + 32.0 * t * t - 0.5628 * (2150.0 - y)
}

/// ΔT = TT − UT (seconds)
///
/// Valid approximately for years 1800–2150
/// Accuracy: ~1–2 seconds (more than sufficient for astrology)
pub fn delta_t(jd_tt: f64) -> f64 {
    // Convert JD to Julian year
    let y = 2000.0 + (jd_tt - 2451545.0) / 365.25;

    if y < 1800.0 {
        // rough fallback
        return 13.0;
    }

    if y <= 1860.0 {
        let t = y - 1800.0;
        return 13.72 - 0.332447 * t + 0.0068612 * t * t + 0.0041116 * t * t * t
            - 0.00037436 * t.powi(4)
            + 0.0000121272 * t.powi(5)
            - 0.0000001699 * t.powi(6)
            + 0.000000000875 * t.powi(7);
    }

    if y <= 1900.0 {
        let t = y - 1860.0;
        return 7.62 + 0.5737 * t - 0.251754 * t * t + 0.01680668 * t * t * t
            - 0.0004473624 * t.powi(4)
            + t.powi(5) / 233174.0;
    }

    if y <= 1920.0 {
        let t = y - 1900.0;
        return -2.79 + 1.494119 * t - 0.0598939 * t * t + 0.0061966 * t * t * t
            - 0.000197 * t.powi(4);
    }

    if y <= 1941.0 {
        let t = y - 1920.0;
        return 21.20 + 0.84493 * t - 0.076100 * t * t + 0.0020936 * t * t * t;
    }

    if y <= 1961.0 {
        let t = y - 1950.0;
        return 29.07 + 0.407 * t - t * t / 233.0 + t * t * t / 2547.0;
    }

    if y <= 1986.0 {
        let t = y - 1975.0;
        return 45.45 + 1.067 * t - t * t / 260.0 - t * t * t / 718.0;
    }

    if y <= 2005.0 {
        let t = y - 2000.0;
        return 63.86 + 0.3345 * t - 0.060374 * t * t
            + 0.0017275 * t * t * t
            + 0.000651814 * t.powi(4)
            + 0.00002373599 * t.powi(5);
    }

    if y <= 2050.0 {
        let t = y - 2000.0;
        return 62.92 + 0.32217 * t + 0.005589 * t * t;
    }

    if y <= 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return -20.0 + 32.0 * u * u - 0.5628 * (2150.0 - y);
    }

    // far future fallback
    let u = (y - 1820.0) / 100.0;
    -20.0 + 32.0 * u * u
}
