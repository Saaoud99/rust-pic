pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) / 1.8;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    return c * 1.8 + 32.0;
}