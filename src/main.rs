use nalgebra::Vector3;

pub fn main() {}

pub trait BasisFunction {
    fn evaluate(&self, x: f64) -> f64;

    fn gradient(&self, x: f64) -> f64;
}

fn BSpline1D(x: f64, h: f64) -> f64 {
    if -2.0 * h <= x && x <= -h {
        return (1. / 6.) * (x / h).powi(3) + (x / h).powi(2) + 2. * (x / h) + (4. / 3.);
    }

    if -h <= x && x <= 0. {
        return (-1. / 2.) * (x / h).powi(3) - (x / h).powi(2) + (4. / 3.);
    }

    if 0. <= x && x <= h {
        return (1. / 2.) * (x / h).powi(3) - (x / h).powi(2) + (4. / 3.);
    }

    if h <= x && x <= 2. * h {
        return (-1. / 6.) * (x / h).powi(3) + (x / h).powi(2) - 2. * (x / h) + (4. / 3.);
    }

    return 0.;
}

pub struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    mass: f64,
    volume: f64,
    stress: Vector3<f64>,
}
