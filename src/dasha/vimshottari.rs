// ======================================================
// Vimshottari Dasha — JHora Correct Implementation
// ======================================================

use crate::{astrology::nakshatra_calc::compute_nakshatra, dasha::DashaMode};

// ------------------ CONSTANTS ------------------

// Sidereal year (days) — JHora compatible
const SIDEREAL_YEAR: f64 = 365.256363004;

// Full Vimshottari cycle = 120 sidereal years
const TOTAL_DAYS: f64 = 120.0 * SIDEREAL_YEAR;

// Nakshatra length (degrees)
const _NAK_LEN: f64 = 13.333333333333334;

// ------------------ DASHĀ LORDS ------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashaLord {
    Ketu,
    Venus,
    Sun,
    Moon,
    Mars,
    Rahu,
    Jupiter,
    Saturn,
    Mercury,
}

impl DashaLord {
    pub fn next(self) -> Self {
        use DashaLord::*;
        match self {
            Ketu => Venus,
            Venus => Sun,
            Sun => Moon,
            Moon => Mars,
            Mars => Rahu,
            Rahu => Jupiter,
            Jupiter => Saturn,
            Saturn => Mercury,
            Mercury => Ketu,
        }
    }
}

// ------------------ MAHĀDASHĀ YEARS ------------------

fn dasha_years(l: DashaLord) -> f64 {
    use DashaLord::*;
    match l {
        Ketu => 7.0,
        Venus => 20.0,
        Sun => 6.0,
        Moon => 10.0,
        Mars => 7.0,
        Rahu => 18.0,
        Jupiter => 16.0,
        Saturn => 19.0,
        Mercury => 17.0,
    }
}

// ------------------ PERIOD STRUCTS ------------------

#[derive(Clone)]
pub struct DashaPeriod {
    pub lord: DashaLord,
    pub start_jd: f64,
    pub end_jd: f64,
    pub mode: DashaMode,
}

#[derive(Clone)]
pub struct AntarPeriod {
    pub maha: DashaLord,
    pub antara: DashaLord,
    pub start_jd: f64,
    pub end_jd: f64,
    pub mode: DashaMode,
}

#[derive(Clone)]
pub struct PratyPeriod {
    pub maha: DashaLord,
    pub antara: DashaLord,
    pub praty: DashaLord,
    pub start_jd: f64,
    pub end_jd: f64,
    pub mode: DashaMode,
}

// ======================================================
// 1️⃣ MAHĀDASHĀ — JHora BACKWARD-SHIFT METHOD
// ======================================================

pub fn mahadasha_timeline(birth_jd: f64, moon_sid_deg: f64, mode: DashaMode) -> Vec<DashaPeriod> {
    let nak = compute_nakshatra(moon_sid_deg, mode);

    let start_lord = nak.lord;
    let frac_elapsed = nak.fraction_elapsed;

    let elapsed_years = frac_elapsed * dasha_years(start_lord);
    let elapsed_days = elapsed_years * SIDEREAL_YEAR;

    let mut current_jd = birth_jd - elapsed_days;
    let mut lord = start_lord;

    let mut out = Vec::new();

    for _ in 0..9 {
        let days = dasha_years(lord) * SIDEREAL_YEAR;

        out.push(DashaPeriod {
            lord,
            start_jd: current_jd,
            end_jd: current_jd + days,
            mode,
        });

        current_jd += days;
        lord = lord.next();
    }

    out
}

// pub fn mahadasha_timeline(birth_jd: f64, moon_sid_deg: f64) -> Vec<DashaPeriod> {
//     // Nakshatra index & lord
//     let nak_index = (moon_sid_deg / NAK_LEN).floor() as usize;
//     let start_lord = NAKSHATRA_LORDS[nak_index];

//     // Fraction elapsed in nakshatra
//     let offset = moon_sid_deg % NAK_LEN;
//     let frac_elapsed = offset / NAK_LEN;

//     // Elapsed part of Mahadasha (sidereal years)
//     let elapsed_years = frac_elapsed * dasha_years(start_lord);
//     let elapsed_days = elapsed_years * SIDEREAL_YEAR;

//     // True Mahadasha start (before birth)
//     let mut current_jd = birth_jd - elapsed_days;
//     let mut lord = start_lord;

//     let mut out = Vec::new();

//     // Full 120-year cycle (9 Mahadashas)
//     for _ in 0..9 {
//         let days = dasha_years(lord) * SIDEREAL_YEAR;

//         out.push(DashaPeriod {
//             lord,
//             start_jd: current_jd,
//             end_jd: current_jd + days,
//         });

//         current_jd += days;
//         lord = lord.next();
//     }

//     out
// }

// ======================================================
// 2️⃣ ANTARDASHĀ — SINGLE BASE CYCLE (CRITICAL FIX)
// ======================================================

pub fn antardasha_timeline(birth_jd: f64, moon_sid_deg: f64, mode: DashaMode) -> Vec<AntarPeriod> {
    let maha = mahadasha_timeline(birth_jd, moon_sid_deg, mode);
    let mut out = Vec::new();

    for m in maha {
        let mut cur = m.start_jd;
        let mut antara = m.lord;

        for _ in 0..9 {
            let days = TOTAL_DAYS * (dasha_years(m.lord) / 120.0) * (dasha_years(antara) / 120.0);

            out.push(AntarPeriod {
                maha: m.lord,
                antara,
                start_jd: cur,
                end_jd: cur + days,
                mode,
            });

            cur += days;
            antara = antara.next();
        }
    }

    out
}

// ======================================================
// 3️⃣ PRATYANTARDASHĀ — SINGLE BASE CYCLE (CRITICAL FIX)
// ======================================================

pub fn pratyantardasha_timeline(
    birth_jd: f64,
    moon_sid_deg: f64,
    mode: DashaMode,
) -> Vec<PratyPeriod> {
    let antara = antardasha_timeline(birth_jd, moon_sid_deg, mode);
    let mut out = Vec::new();

    for a in antara {
        let mut cur = a.start_jd;
        let mut praty = a.antara;

        for _ in 0..9 {
            let days = TOTAL_DAYS
                * (dasha_years(a.maha) / 120.0)
                * (dasha_years(a.antara) / 120.0)
                * (dasha_years(praty) / 120.0);

            out.push(PratyPeriod {
                maha: a.maha,
                antara: a.antara,
                praty,
                start_jd: cur,
                end_jd: cur + days,
                mode,
            });

            cur += days;
            praty = praty.next();
        }
    }

    out
}
