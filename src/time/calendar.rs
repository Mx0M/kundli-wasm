#[derive(Clone)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

pub fn jd_to_calendar(jd: f64) -> CalendarDate {
    // Meeus algorithm
    let z = (jd + 0.5).floor();
    let f = (jd + 0.5) - z;

    let mut a = z;
    if z >= 2299161.0 {
        let alpha = ((z - 1867216.25) / 36524.25).floor();
        a += 1.0 + alpha - (alpha / 4.0).floor();
    }

    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();

    let day = (b - d - (30.6001 * e).floor() + f) as u8;
    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 } as u8;
    let year = if month > 2 { c - 4716.0 } else { c - 4715.0 } as i32;

    CalendarDate { year, month, day }
}
