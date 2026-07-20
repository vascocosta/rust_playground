use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self {
            real: 0.0,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self {
            real: value,
            imag: 0.0,
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: (self.real * rhs.real) - (self.imag * rhs.imag),
            imag: (self.real * rhs.imag) + (self.imag * rhs.real),
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}i", self.real, self.imag)
    }
}

struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

fn main() {
    let mut z1 = Complex::new(1.0, 2.0);

    let z2 = Complex {
        real: 3.0,
        imag: 4.0,
    };

    let sum = z1 + z2;

    z1 += z2;

    println!("{}", sum);
    println!("{}", z1);
}
