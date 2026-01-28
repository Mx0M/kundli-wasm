pub mod vimshottari;
#[derive(Clone, Copy)]
pub enum DashaMode {
    Astronomical,    // pure Moon longitude
    JHoraCompatible, // nakshatra-adjusted
}
