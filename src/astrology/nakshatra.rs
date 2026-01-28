use crate::dasha::vimshottari::DashaLord;
use std::f64::consts::PI;

/// Information about a Nakshatra
#[derive(Debug, Clone)]
pub struct NakshatraInfo {
    pub index: usize, // 0..26
    pub name: &'static str,
    pub pada: u8, // 1..4
    pub lord: DashaLord,
}

/// Nakshatra names in order (sidereal)
pub const NAKSHATRA_NAMES: [&str; 27] = [
    "Ashwini",
    "Bharani",
    "Krittika",
    "Rohini",
    "Mrigashira",
    "Ardra",
    "Punarvasu",
    "Pushya",
    "Ashlesha",
    "Magha",
    "Purva Phalguni",
    "Uttara Phalguni",
    "Hasta",
    "Chitra",
    "Swati",
    "Vishakha",
    "Anuradha",
    "Jyeshtha",
    "Mula",
    "Purva Ashadha",
    "Uttara Ashadha",
    "Shravana",
    "Dhanishta",
    "Shatabhisha",
    "Purva Bhadrapada",
    "Uttara Bhadrapada",
    "Revati",
];

/// Nakshatra lords (same order as names)
pub const NAKSHATRA_LORDS: [DashaLord; 27] = [
    DashaLord::Ketu,    // Ashwini
    DashaLord::Venus,   // Bharani
    DashaLord::Sun,     // Krittika
    DashaLord::Moon,    // Rohini
    DashaLord::Mars,    // Mrigashira
    DashaLord::Rahu,    // Ardra
    DashaLord::Jupiter, // Punarvasu
    DashaLord::Saturn,  // Pushya
    DashaLord::Mercury, // Ashlesha
    DashaLord::Ketu,    // Magha
    DashaLord::Venus,   // Purva Phalguni
    DashaLord::Sun,     // Uttara Phalguni
    DashaLord::Moon,    // Hasta
    DashaLord::Mars,    // Chitra
    DashaLord::Rahu,    // Swati
    DashaLord::Jupiter, // Vishakha
    DashaLord::Saturn,  // Anuradha
    DashaLord::Mercury, // Jyeshtha
    DashaLord::Ketu,    // Mula
    DashaLord::Venus,   // Purva Ashadha
    DashaLord::Sun,     // Uttara Ashadha
    DashaLord::Moon,    // Shravana
    DashaLord::Mars,    // Dhanishta
    DashaLord::Rahu,    // Shatabhisha
    DashaLord::Jupiter, // Purva Bhadrapada
    DashaLord::Saturn,  // Uttara Bhadrapada
    DashaLord::Mercury, // Revati
];

/// Compute Nakshatra + Pada from sidereal longitude
///
/// Input:
/// - lon_rad : sidereal longitude in radians (0..2π)
///
/// Output:
/// - NakshatraInfo (index, name, pada, lord)
pub fn nakshatra_from_sidereal_lon(lon_rad: f64) -> NakshatraInfo {
    let lon = normalize(lon_rad);

    // One nakshatra = 360° / 27
    let nak_size = 2.0 * PI / 27.0;
    let pada_size = nak_size / 4.0;

    let index = (lon / nak_size).floor() as usize;
    let index = index.min(26);

    let nak_start = index as f64 * nak_size;
    let pada = ((lon - nak_start) / pada_size).floor() as u8 + 1;

    NakshatraInfo {
        index,
        name: NAKSHATRA_NAMES[index],
        pada: pada.min(4),
        lord: NAKSHATRA_LORDS[index],
    }
}

/// Normalize angle to 0..2π
#[inline]
fn normalize(mut a: f64) -> f64 {
    a %= 2.0 * PI;
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}
