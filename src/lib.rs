pub mod astrology;
pub mod dasha;
pub mod divisional;
pub mod ephemeris;
pub mod frames;
pub mod math;
pub mod time;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use std::f64::consts::TAU;

// use crate::astrology::nakshatra_calc::compute_nakshatra;
use crate::dasha::DashaMode;
// use crate::ephemeris::moon_mean::moon_mean_longitude_tropical;
// ===================== TIME =====================
use crate::time::calendar::jd_to_calendar;
use crate::time::datetime::{DateTimeInput, jd_tt_from_datetime};
use crate::time::julian::jd_ut_from_tt;

// ===================== EPHEMERIS =====================
use crate::ephemeris::moon::node_true::true_lunar_node;
use crate::ephemeris::moon_elp::moon_longitude_elp;
use crate::ephemeris::planets::{
    jupiter_lon, mars_lon, mercury_lon, saturn_lon, sun_lon, venus_lon,
};

// ===================== FRAME =====================
use crate::frames::ayanamsa::lahiri_ayanamsa;

// ===================== ASTROLOGY =====================
use crate::astrology::houses::{ascendant_sidereal, whole_sign_houses};
use crate::astrology::nakshatra::nakshatra_from_sidereal_lon;

// ===================== DASHAS =====================
use crate::dasha::vimshottari::{
    DashaLord, antardasha_timeline, mahadasha_timeline, pratyantardasha_timeline,
};

// ===================== DIVISIONAL =====================
use crate::divisional::divisional_chart;

// =====================================================
// =================== JS STRUCTS =======================
// =====================================================

#[derive(Serialize)]
pub struct PlanetJS {
    pub name: String,
    pub tropical_deg: f64,
    pub sidereal_deg: f64,
}

#[derive(Serialize)]
pub struct HouseJS {
    pub number: u8,
    pub sign: String,
}

#[derive(Serialize)]
pub struct DashaPeriodJS {
    pub maha: String,
    pub antara: Option<String>,
    pub praty: Option<String>,
    pub start_jd: f64,
    pub end_jd: f64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct DivisionalPlanetJS {
    pub planet: String,
    pub sign: String,
    pub degree: f64,
}

#[derive(Serialize)]
pub struct DivisionalChartJS {
    pub division: u8,
    pub planets: Vec<DivisionalPlanetJS>,
}

#[derive(Serialize)]
pub struct KundliJS {
    pub jd_tt: f64,
    pub jd_ut: f64,

    pub ascendant_sidereal_deg: f64,

    pub moon_sidereal_deg: f64,
    pub nakshatra: String,
    pub pada: u8,
    pub nakshatra_lord: String,

    pub planets: Vec<PlanetJS>,
    pub houses: Vec<HouseJS>,

    pub mahadashas: Vec<DashaPeriodJS>,
    pub antardashas: Vec<DashaPeriodJS>,
    pub pratyantardashas: Vec<DashaPeriodJS>,

    pub divisional_charts: Vec<DivisionalChartJS>,
}

// =====================================================
// =================== HELPERS ==========================
// =====================================================

fn normalize(mut a: f64) -> f64 {
    a %= TAU;
    if a < 0.0 {
        a += TAU;
    }
    a
}

fn lord_name(l: DashaLord) -> String {
    format!("{:?}", l)
}

fn fmt_date(jd: f64) -> String {
    let d = jd_to_calendar(jd);
    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
}

// =====================================================
// =================== WASM API =========================
// =====================================================

#[wasm_bindgen]
pub fn generate_kundli(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: f64,
    tz_offset_hours: f64,
    latitude_deg: f64,
    longitude_deg: f64,
) -> JsValue {
    // ---------- TIME ----------
    let jd_tt = jd_tt_from_datetime(DateTimeInput {
        year,
        month,
        day,
        hour,
        minute,
        second,
        tz_offset_hours,
    });

    let jd_ut = jd_ut_from_tt(jd_tt);

    // ---------- AYANAMSA ----------
    let ayan = lahiri_ayanamsa(jd_tt);

    // ---------- MOON ----------
    let moon_trop = moon_longitude_elp(jd_tt);
    let moon_sid = normalize(moon_trop - ayan);
    // use for mean mmoon calculation
    // let moon_mean_tropical = moon_mean_longitude_tropical(jd_tt);
    // let moon_mean_sidereal = normalize(moon_mean_tropical - ayan);

    let nak = nakshatra_from_sidereal_lon(moon_sid);

    // ---------- PLANETS ----------
    let raw_planets = vec![
        ("Sun", sun_lon(jd_tt)),
        ("Moon", moon_trop),
        ("Mercury", mercury_lon(jd_tt)),
        ("Venus", venus_lon(jd_tt)),
        ("Mars", mars_lon(jd_tt)),
        ("Jupiter", jupiter_lon(jd_tt)),
        ("Saturn", saturn_lon(jd_tt)),
        ("Rahu", true_lunar_node(jd_tt)),
        ("Ketu", normalize(true_lunar_node(jd_tt) + TAU / 2.0)),
    ];

    let planets: Vec<PlanetJS> = raw_planets
        .into_iter()
        .map(|(name, trop)| {
            let sid = normalize(trop - ayan);
            PlanetJS {
                name: name.to_string(),
                tropical_deg: trop.to_degrees(),
                sidereal_deg: sid.to_degrees(),
            }
        })
        .collect();

    // ---------- HOUSES ----------
    let lat = latitude_deg.to_radians();
    let lon = longitude_deg.to_radians();

    let asc_sid = ascendant_sidereal(jd_ut, lat, lon);

    let houses: Vec<HouseJS> = whole_sign_houses(asc_sid)
        .into_iter()
        .map(|h| HouseJS {
            number: h.number,
            sign: h.sign.name().to_string(),
        })
        .collect();

    // ---------- DASHAS ----------
    let mahadashas = mahadasha_timeline(jd_tt, moon_sid, DashaMode::Astronomical)
        .into_iter()
        .map(|d| DashaPeriodJS {
            maha: lord_name(d.lord),
            antara: None,
            praty: None,
            start_jd: d.start_jd,
            end_jd: d.end_jd,
            start_date: fmt_date(d.start_jd),
            end_date: fmt_date(d.end_jd),
        })
        .collect();

    let antardashas = antardasha_timeline(jd_tt, moon_sid, DashaMode::Astronomical)
        .into_iter()
        .map(|d| DashaPeriodJS {
            maha: lord_name(d.maha),
            antara: Some(lord_name(d.antara)),
            praty: None,
            start_jd: d.start_jd,
            end_jd: d.end_jd,
            start_date: fmt_date(d.start_jd),
            end_date: fmt_date(d.end_jd),
        })
        .collect();

    let pratyantardashas = pratyantardasha_timeline(jd_tt, moon_sid, DashaMode::Astronomical)
        .into_iter()
        .map(|d| DashaPeriodJS {
            maha: lord_name(d.maha),
            antara: Some(lord_name(d.antara)),
            praty: Some(lord_name(d.praty)),
            start_jd: d.start_jd,
            end_jd: d.end_jd,
            start_date: fmt_date(d.start_jd),
            end_date: fmt_date(d.end_jd),
        })
        .collect();

    // ---------- DIVISIONAL D1–D30 ----------
    let mut divisional_charts = Vec::new();

    for div in 1..=30 {
        let chart = divisional_chart(div, &planets);

        let planets_js = chart
            .planets
            .into_iter()
            .map(|p| DivisionalPlanetJS {
                planet: p.planet,
                sign: p.sign,
                degree: p.degree,
            })
            .collect();

        divisional_charts.push(DivisionalChartJS {
            division: div,
            planets: planets_js,
        });
    }

    // ---------- OUTPUT ----------
    let result = KundliJS {
        jd_tt,
        jd_ut,

        ascendant_sidereal_deg: asc_sid.to_degrees(),

        moon_sidereal_deg: moon_sid.to_degrees(),
        nakshatra: nak.name.to_string(),
        pada: nak.pada,
        nakshatra_lord: lord_name(nak.lord),

        planets,
        houses,

        mahadashas,
        antardashas,
        pratyantardashas,

        divisional_charts,
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}
