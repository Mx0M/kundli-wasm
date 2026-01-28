use crate::ephemeris::vsop_helio::{Planet, heliocentric_xyz};
use crate::ephemeris::vsop87a_full_authoritative as vsop;
use crate::math::vec3::Vec3;

#[inline]
fn vsop_t(jd_tt: f64) -> f64 {
    (jd_tt - 2451545.0) / 365250.0
}

/// GEOCENTRIC rectangular XYZ (AU)
pub fn geocentric_xyz(planet: Planet, jd_tt: f64) -> Vec3 {
    let t = vsop_t(jd_tt);

    // Earth heliocentric position
    let e = vsop::get_emb(t);

    match planet {
        // ☀️ SPECIAL CASE — Sun
        Planet::Sun => Vec3 {
            x: -e[0],
            y: -e[1],
            z: -e[2],
        },

        // All other planets
        _ => {
            let p = heliocentric_xyz(planet, jd_tt);
            Vec3 {
                x: p.x - e[0],
                y: p.y - e[1],
                z: p.z - e[2],
            }
        }
    }
}
