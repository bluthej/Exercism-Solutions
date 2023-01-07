// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PRODUCTION_RATE: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed32 = u32::from(speed);

    match speed {
        0 => 0.,
        1..=4 => (speed32 * PRODUCTION_RATE) as f64,
        5..=8 => ((speed32 * PRODUCTION_RATE) as f64) * 0.9,
        9..=10 => ((speed32 * PRODUCTION_RATE) as f64) * 0.77,
        _ => panic!("speed out of range"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
