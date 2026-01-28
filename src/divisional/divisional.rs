use std::f64::consts::TAU;

/// Output planet in a divisional chart
#[derive(Clone)]
pub struct DivisionalPlanet {
    pub planet: String,
    pub sign: String,
    pub degree: f64,
}

/// Output divisional chart
pub struct DivisionalChart {
    pub division: u8,
    pub planets: Vec<DivisionalPlanet>,
}

static SIGNS: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

#[inline]
fn sign_name(index: u8) -> &'static str {
    SIGNS[(index % 12) as usize]
}

/// Generic Parāśari divisional mapping (D1–D30)
pub fn divisional_chart(division: u8, planets: &[crate::PlanetJS]) -> DivisionalChart {
    assert!(division >= 1 && division <= 30);

    let mut out = Vec::new();
    let part = TAU / (12.0 * division as f64);

    for p in planets {
        let lon = p.sidereal_deg.to_radians();

        let index = (lon / part).floor() as u32;
        let sign_index = (index % 12) as u8;

        let deg_in_sign = ((lon / part).fract()) * (30.0 / division as f64);

        out.push(DivisionalPlanet {
            planet: p.name.clone(),
            sign: sign_name(sign_index).to_string(),
            degree: deg_in_sign,
        });
    }

    DivisionalChart {
        division,
        planets: out,
    }
}
