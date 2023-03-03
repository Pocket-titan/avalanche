use std::ops::Index;

struct Vector<T: Sized, const N: usize>([T; N]);

impl<T: Sized + Copy, const N: usize> Vector<T, N> {
    fn x(&self) -> T
    where
        Self: SizeAtLeast<2>,
    {
        self.0[0]
    }

    fn y(&self) -> T
    where
        Self: SizeAtLeast<2>,
    {
        self.0[1]
    }

    fn z(&self) -> T
    where
        Self: SizeAtLeast<3>,
    {
        self.0[2]
    }
}

trait SizeAtLeast<const N: usize> {}

impl<T: Sized> SizeAtLeast<2> for Vector<T, 2> {}
impl<T: Sized> SizeAtLeast<2> for Vector<T, 3> {}

impl<T: Sized> SizeAtLeast<3> for Vector<T, 3> {}

impl<I, const N: usize> Index<usize> for Vector<I, N> {
    type Output = I;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub struct Basis<F: BasisFunction, const D: usize> {
    // Grid, h
    dimension: usize,
    f: F,
}

pub trait BasisFunction {
    fn evaluate(&self, x: &f64) -> f64;
    fn derivative(&self, x: &f64) -> f64;
}

impl<F: BasisFunction, const D: usize> Basis<F, D> {
    fn new(dimension: usize, f: F) -> Self {
        Self { dimension, f }
    }
}

impl<F: BasisFunction> Basis<F, 1> {
    fn evaluate(&self, x: &f64) -> f64 {
        self.f.evaluate(x)
    }

    fn derivative(&self, x: &f64) -> f64 {
        self.f.derivative(x)
    }
}

impl<F: BasisFunction> Basis<F, 2> {
    fn evaluate(&self, x: &Vector<f64, 2>) -> f64 {
        self.f.evaluate(&x.x()) * self.f.evaluate(&x.y())
    }

    fn derivative(&self, x: &Vector<f64, 2>) -> Vector<f64, 2> {
        let dx = self.f.derivative(&x.x());
        let dy = self.f.derivative(&x.y());

        Vector([dy * self.f.evaluate(&x.x()), dx * self.f.evaluate(&x.y())])
    }
}

impl<F: BasisFunction> Basis<F, 3> {
    fn evaluate(&self, x: &Vector<f64, 3>) -> f64 {
        self.f.evaluate(&x.x()) * self.f.evaluate(&x.y()) * self.f.evaluate(&x.z())
    }

    fn derivative(&self, x: &Vector<f64, 3>) -> Vector<f64, 3> {
        let dx = self.f.derivative(&x.x());
        let dy = self.f.derivative(&x.y());
        let dz = self.f.derivative(&x.z());

        Vector([
            dx * self.f.evaluate(&x.y()) * self.f.evaluate(&x.z()),
            dy * self.f.evaluate(&x.x()) * self.f.evaluate(&x.z()),
            dz * self.f.evaluate(&x.x()) * self.f.evaluate(&x.y()),
        ])
    }
}

#[derive(Default)]
pub struct PiecewiseLinear;

impl BasisFunction for PiecewiseLinear {
    fn evaluate(&self, x: &f64) -> f64 {
        let x_abs = x.abs();

        if 0. <= x_abs && x_abs < 1. {
            return 1. - x_abs;
        }

        0.
    }

    fn derivative(&self, x: &f64) -> f64 {
        let mut res = 0.;

        if &-1. <= x && x < &0. {
            res = 1.;
        } else if &0. <= x && x <= &1. {
            res = -1.;
        }

        res
    }
}

#[derive(Default)]
pub struct CubicSpline;

impl BasisFunction for CubicSpline {
    fn evaluate(&self, x: &f64) -> f64 {
        let x_abs = x.abs();

        if 0. <= x_abs && x_abs < 1. {
            return (1. / 2.) * x_abs.powi(3) - x_abs.powi(2) + (2. / 3.);
        }

        if 1. <= x_abs && x_abs <= 2. {
            return (-1. / 6.) * x_abs.powi(3) + x_abs.powi(2) - 2. * x_abs + (4. / 3.);
        }

        0.
    }

    fn derivative(&self, x: &f64) -> f64 {
        let abs_x = x.abs();
        let mut res = 0.;

        if 0. <= abs_x && abs_x < 1. {
            res = x * ((3. / 2.) * abs_x - 2.);
        } else if 1. <= abs_x && abs_x <= 2. {
            res = x * (2. - (1. / 2.) * abs_x - 2. / abs_x);
        }

        res
    }
}
