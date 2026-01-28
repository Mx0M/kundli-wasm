// src/ephemeris/moon/elp_types.rs

#[derive(Clone, Copy)]
pub struct ElpTerm {
    pub d: i8,
    pub m: i8,
    pub mp: i8,
    pub f: i8,
    pub a0: f64, // arcseconds
    pub a1: f64,
    // arcseconds per T
}
