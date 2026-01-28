// src/ephemeris/coordinates.rs
//
// Converts GEOCENTRIC rectangular coordinates (XYZ)
// into ecliptic longitude, latitude, and distance.
//
// Frame: mean ecliptic & equinox of date
// Units:
//   - longitude, latitude: radians
//   - distance: AU

use crate::math::vec3::Vec3;

/// Spherical ecliptic coordinates
#[derive(Clone, Copy, Debug)]
pub struct EclipticCoord {
    pub lon: f64, // radians, 0..2π
    pub lat: f64, // radians, −π/2..+π/2
    pub r: f64,   // AU
}

/// Convert rectangular XYZ → ecliptic lon/lat
pub fn xyz_to_ecliptic(v: Vec3) -> EclipticCoord {
    let r_xy = (v.x * v.x + v.y * v.y).sqrt();
    let r = (r_xy * r_xy + v.z * v.z).sqrt();

    // Longitude λ = atan2(y, x)
    let mut lon = v.y.atan2(v.x);
    if lon < 0.0 {
        lon += std::f64::consts::TAU;
    }

    // Latitude β = atan2(z, sqrt(x² + y²))
    let lat = v.z.atan2(r_xy);

    EclipticCoord { lon, lat, r }
}
