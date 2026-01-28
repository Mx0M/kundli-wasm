use crate::PlanetJS;
use crate::astrology::houses::House;
use crate::astrology::nakshatra::NakshatraInfo;
use crate::dasha::vimshottari::{AntarPeriod, DashaPeriod, PratyPeriod};
use crate::divisional::DivisionalChart;

// =====================================================
// Core Chart Model (Domain-Level)
// =====================================================

/// Natal chart (D1) + timing systems
pub struct Chart {
    // ---------------- TIME ----------------
    pub jd_tt: f64,
    pub jd_ut: f64,

    // ---------------- LAGNA ----------------
    pub ascendant_sidereal_deg: f64,
    pub houses: Vec<House>,

    // ---------------- PLANETS ----------------
    pub planets: Vec<PlanetJS>,

    // ---------------- MOON ----------------
    pub moon_sidereal_deg: f64,
    pub nakshatra: NakshatraInfo,

    // ---------------- DASHAS ----------------
    pub mahadashas: Vec<DashaPeriod>,
    pub antardashas: Vec<AntarPeriod>,
    pub pratyantardashas: Vec<PratyPeriod>,

    // ---------------- DIVISIONAL ----------------
    pub divisional_charts: Vec<DivisionalChart>,
}

// =====================================================
// Helpers
// =====================================================

impl Chart {
    /// Return current Mahadasha at a given JD
    pub fn current_mahadasha(&self, jd: f64) -> Option<&DashaPeriod> {
        self.mahadashas
            .iter()
            .find(|d| jd >= d.start_jd && jd < d.end_jd)
    }

    /// Return current Antardasha at a given JD
    pub fn current_antardasha(&self, jd: f64) -> Option<&AntarPeriod> {
        self.antardashas
            .iter()
            .find(|d| jd >= d.start_jd && jd < d.end_jd)
    }

    /// Return current Pratyantardasha at a given JD
    pub fn current_pratyantardasha(&self, jd: f64) -> Option<&PratyPeriod> {
        self.pratyantardashas
            .iter()
            .find(|d| jd >= d.start_jd && jd < d.end_jd)
    }

    /// Planet → house mapping (Whole Sign)
    pub fn planet_house(&self, planet_name: &str) -> Option<u8> {
        let p = self.planets.iter().find(|p| p.name == planet_name)?;

        let sign_index = (p.sidereal_deg / 30.0).floor() as u8;

        let asc_sign = (self.ascendant_sidereal_deg / 30.0).floor() as u8;

        Some(((sign_index + 12 - asc_sign) % 12) + 1)
    }

    /// Get divisional chart by division number (D1–D30)
    pub fn divisional(&self, division: u8) -> Option<&DivisionalChart> {
        self.divisional_charts
            .iter()
            .find(|c| c.division == division)
    }
}
