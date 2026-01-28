use crate::astrology::nakshatra::NAKSHATRA_LORDS;
use crate::dasha::DashaMode;

const NAK_LEN: f64 = 13.333333333333334;

pub struct NakshatraResult {
    pub index: usize,
    pub lord: crate::dasha::vimshottari::DashaLord,
    pub fraction_elapsed: f64,
}

/// Compute nakshatra according to selected mode
pub fn compute_nakshatra(moon_sid_deg: f64, mode: DashaMode) -> NakshatraResult {
    let corrected_lon = match mode {
        DashaMode::Astronomical => moon_sid_deg,

        // JHora-compatible adjustment
        DashaMode::JHoraCompatible => {
            // Empirically consistent correction:
            // shifts boundary behavior, not Moon physics
            moon_sid_deg - 0.25
        }
    };

    let mut lon = corrected_lon % 360.0;
    if lon < 0.0 {
        lon += 360.0;
    }

    let index = (lon / NAK_LEN).floor() as usize;
    let offset = lon % NAK_LEN;
    let fraction_elapsed = offset / NAK_LEN;

    NakshatraResult {
        index,
        lord: NAKSHATRA_LORDS[index],
        fraction_elapsed,
    }
}
