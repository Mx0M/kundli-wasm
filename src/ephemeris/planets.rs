use std::f64::consts::TAU;

use crate::ephemeris::vsop87a_full_authoritative::{
    get_earth, get_jupiter, get_mars, get_mercury, get_saturn, get_venus,
};

/// Speed of light in AU / day
const C_AU_PER_DAY: f64 = 173.144632674240;

/// Classical planets
#[derive(Debug, Clone, Copy)]
pub enum Planet {
    Sun,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
}

/// VSOP time argument
#[inline]
fn vsop_t(jd_tt: f64) -> f64 {
    (jd_tt - 2451545.0) / 365250.0
}

/// Heliocentric XYZ
#[inline]
fn heliocentric_xyz(planet: Planet, t: f64) -> (f64, f64, f64) {
    match planet {
        Planet::Sun => (0.0, 0.0, 0.0),
        Planet::Mercury => {
            let p = get_mercury(t);
            (p[0], p[1], p[2])
        }
        Planet::Venus => {
            let p = get_venus(t);
            (p[0], p[1], p[2])
        }
        Planet::Mars => {
            let p = get_mars(t);
            (p[0], p[1], p[2])
        }
        Planet::Jupiter => {
            let p = get_jupiter(t);
            (p[0], p[1], p[2])
        }
        Planet::Saturn => {
            let p = get_saturn(t);
            (p[0], p[1], p[2])
        }
    }
}

/// Geocentric tropical longitude with light-time correction
pub fn planet_tropical_lon(planet: Planet, jd_tt: f64) -> f64 {
    let t = vsop_t(jd_tt);

    // Earth heliocentric position
    let e = get_earth(t);
    let (xe, ye, ze) = (e[0], e[1], e[2]);

    // Initial heliocentric planet position
    let (xp, yp, zp) = heliocentric_xyz(planet, t);

    // Initial geocentric vector
    let (xg0, yg0, zg0) = if let Planet::Sun = planet {
        (-xe, -ye, -ze)
    } else {
        (xp - xe, yp - ye, zp - ze)
    };

    // Light-time correction
    let r = (xg0 * xg0 + yg0 * yg0 + zg0 * zg0).sqrt();
    let delta_days = r / C_AU_PER_DAY;
    let t_corr = t - delta_days / 365250.0;

    // Recompute planet position
    let (xp2, yp2, zp2) = heliocentric_xyz(planet, t_corr);

    let (xg, yg, _) = if let Planet::Sun = planet {
        (-xe, -ye, -ze)
    } else {
        (xp2 - xe, yp2 - ye, zp2 - ze)
    };

    normalize(yg.atan2(xg))
}

/// Convenience wrappers
pub fn sun_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Sun, jd_tt)
}
pub fn mercury_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Mercury, jd_tt)
}
pub fn venus_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Venus, jd_tt)
}
pub fn mars_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Mars, jd_tt)
}
pub fn jupiter_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Jupiter, jd_tt)
}
pub fn saturn_lon(jd_tt: f64) -> f64 {
    planet_tropical_lon(Planet::Saturn, jd_tt)
}

/// Normalize angle
#[inline]
fn normalize(mut a: f64) -> f64 {
    a %= TAU;
    if a < 0.0 {
        a += TAU;
    }
    a
}
