use crate::ephemeris::vsop87a_full_authoritative as vsop;
use crate::math::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Planet {
    Sun,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
}

#[inline]
fn vsop_t(jd_tt: f64) -> f64 {
    (jd_tt - 2451545.0) / 365250.0
}

/// Returns HELIOCENTRIC rectangular XYZ (AU)
pub fn heliocentric_xyz(planet: Planet, jd_tt: f64) -> Vec3 {
    let t = vsop_t(jd_tt);

    match planet {
        Planet::Mercury => {
            let v = vsop::get_mercury(t);
            Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        }
        Planet::Venus => {
            let v = vsop::get_venus(t);
            Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        }
        Planet::Mars => {
            let v = vsop::get_mars(t);
            Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        }
        Planet::Jupiter => {
            let v = vsop::get_jupiter(t);
            Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        }
        Planet::Saturn => {
            let v = vsop::get_saturn(t);
            Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        }

        // Sun at heliocentric origin
        Planet::Sun => Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    }
}
