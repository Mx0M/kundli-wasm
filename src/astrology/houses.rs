use std::f64::consts::TAU;

use crate::frames::ayanamsa::lahiri_ayanamsa;
use crate::time::sidereal::local_sidereal_time;

/// Zodiac signs
#[derive(Debug, Clone)]
pub enum Sign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl Sign {
    pub fn from_index(i: u8) -> Self {
        match i % 12 {
            0 => Sign::Aries,
            1 => Sign::Taurus,
            2 => Sign::Gemini,
            3 => Sign::Cancer,
            4 => Sign::Leo,
            5 => Sign::Virgo,
            6 => Sign::Libra,
            7 => Sign::Scorpio,
            8 => Sign::Sagittarius,
            9 => Sign::Capricorn,
            10 => Sign::Aquarius,
            _ => Sign::Pisces,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Sign::Aries => "Aries",
            Sign::Taurus => "Taurus",
            Sign::Gemini => "Gemini",
            Sign::Cancer => "Cancer",
            Sign::Leo => "Leo",
            Sign::Virgo => "Virgo",
            Sign::Libra => "Libra",
            Sign::Scorpio => "Scorpio",
            Sign::Sagittarius => "Sagittarius",
            Sign::Capricorn => "Capricorn",
            Sign::Aquarius => "Aquarius",
            Sign::Pisces => "Pisces",
        }
    }
}

/// House info
pub struct House {
    pub number: u8,
    pub sign: Sign,
}

/// Compute sidereal Ascendant (radians)
pub fn ascendant_sidereal(jd_tt: f64, latitude_rad: f64, longitude_rad: f64) -> f64 {
    let lst = local_sidereal_time(jd_tt, longitude_rad);
    let obliq = 23.43929111_f64.to_radians(); // mean obliquity

    let tan_a = 1.0 / (lst.cos()) * (-lst.sin());

    let asc = tan_a.atan2(latitude_rad.cos() - tan_a * latitude_rad.sin() * obliq.tan());

    let mut asc = asc;
    if asc < 0.0 {
        asc += TAU;
    }

    // convert to sidereal
    let ayan = lahiri_ayanamsa(jd_tt);
    let mut sid = asc - ayan;
    if sid < 0.0 {
        sid += TAU;
    }

    sid
}

/// Whole Sign Houses (D1)
pub fn whole_sign_houses(asc_sidereal: f64) -> Vec<House> {
    let asc_sign = (asc_sidereal / (TAU / 12.0)).floor() as u8;

    (0..12)
        .map(|i| House {
            number: i + 1,
            sign: Sign::from_index(asc_sign + i),
        })
        .collect()
}
