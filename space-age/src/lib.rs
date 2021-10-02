// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const CONVERSION_FACTOR: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.0 / 31557600.0) / Self::CONVERSION_FACTOR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const CONVERSION_FACTOR: f64 = 0.2408467;
}
impl Planet for Venus {
    const CONVERSION_FACTOR: f64 = 0.61519726;
}
impl Planet for Earth {
    const CONVERSION_FACTOR: f64 = 1.0;
}
impl Planet for Mars {
    const CONVERSION_FACTOR: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const CONVERSION_FACTOR: f64 = 11.862615;
}
impl Planet for Saturn {
    const CONVERSION_FACTOR: f64 = 29.447498;
}
impl Planet for Uranus {
    const CONVERSION_FACTOR: f64 = 84.016846;
}
impl Planet for Neptune {
    const CONVERSION_FACTOR: f64 = 164.79132;
}

