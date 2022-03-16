/// returns 5x^4 - 2x^3 + x^2 + 2x + 3
/// uses a Horner's method
pub fn polynomial(x: f64, coefs: [f64; 5]) -> f64 {
    let mut out: f64 = coefs[0];
    for i in 1..coefs.len() {
        out = out*x+coefs[i];
    }
    out
}

pub fn polynomial_der(x: f64, coefs: [f64; 5]) -> f64 {
    let new_coefs: [f64; 4] = [coefs[0]*4.0, coefs[1]*3.0, coefs[2]*2.0, coefs[3]];
    let mut out: f64 = new_coefs[0];
    for i in 1..new_coefs.len() {
        out = out*x+new_coefs[i];
    }
    out
}

///returns 2sin(x) * 5cos(x/2)
pub fn trigonometric(x: f64) -> f64 {
    2.0 * x.sin() * 5.0 * (x / 2.0).cos()
}

pub fn trigonometric_der(x: f64) -> f64 {
    10.0 * (x/2.0).cos() * x.cos() - 5.0 * (x/2.0).sin() * x.sin()
}

///returns 5^x - 2
pub fn exponential(x: f64) -> f64 {
    5.0_f64.powf(x) - 2.0
}

pub fn exponential_der(x: f64) -> f64 {
    5.0_f64.powf(x) * 5.0_f64.log10()
}

///returns sin(x^2) + 5x^3 + 1.05^x
pub fn mixed_functions(x: f64) -> f64 {
    x.powf(2.0).sin() + 5.0 * x.powf(3.0) + 1.05_f64.powf(x)
}

pub fn mixed_der(x: f64) -> f64 {
    0.0487902 * 1.05_f64.powf(x) + 15.0*x.powf(2.0) + 2.0*x*x.powf(2.0).cos()
}