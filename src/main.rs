use std::{
    fmt::Display,
    hint::black_box,
    ops::{Add, AddAssign, Index, Mul, Sub, SubAssign},
};

use rand::Rng;

#[derive(Clone, Copy, Default)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
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

#[derive(Default, Clone)]
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

impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        let data: Vec<T> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
}

impl<T> Sub for &Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.rows, rhs.rows,
            "The number of rows in the left must equal the number of rows in the right."
        );
        assert_eq!(
            self.cols, rhs.cols,
            "The number of cols in the left must equal the number of cols in the right."
        );

        let data: Vec<T> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| *a - *b)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
}

impl<T> Mul for &Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.cols, rhs.rows,
            "The number of columns in the left matrix must equal the number of rows in the right matrix."
        );

        let mut data = vec![T::default(); self.rows * rhs.cols];

        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut sum = T::default();

                for k in 0..self.cols {
                    sum = sum + self.data[r * self.cols + k] * rhs.data[k * rhs.cols + c];
                }

                data[r * rhs.cols + c] = sum;
            }
        }

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data,
        }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.cols, rhs.rows,
            "The number of columns in the left matrix must equal the number of rows in the right matrix."
        );

        let mut data = vec![T::default(); self.rows * rhs.cols];

        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut sum = T::default();

                for k in 0..self.cols {
                    sum = sum + self.data[r * self.cols + k] * rhs.data[k * rhs.cols + c];
                }

                data[r * rhs.cols + c] = sum;
            }
        }

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row * self.cols + col]
    }
}

fn generate_random_matrix(rows: usize, cols: usize, range: f64) -> Matrix<Complex> {
    let mut rng = rand::rng();

    let data = (0..rows * cols)
        .map(|_| {
            Complex::new(
                rng.random_range(-range..=range),
                rng.random_range(-range..=range),
            )
        })
        .collect();

    Matrix { rows, cols, data }
}

fn run_matrix_benchmark() {
    const MATRIX_SIZE: usize = 512;
    const NUM_RUNS: usize = 1000;
    const RANGE: f64 = 10.0;

    println!("Generating random data...");

    let left_matrices: Vec<_> = (0..NUM_RUNS)
        .map(|_| generate_random_matrix(MATRIX_SIZE, MATRIX_SIZE, RANGE))
        .collect();

    let right_matrices: Vec<_> = (0..NUM_RUNS)
        .map(|_| generate_random_matrix(MATRIX_SIZE, MATRIX_SIZE, RANGE))
        .collect();

    println!("Running {NUM_RUNS} multiplications...");

    let mut checksum = 0.0;

    for i in 0..NUM_RUNS {
        let result = black_box(left_matrices[i].clone()) * black_box(right_matrices[i].clone());

        checksum += black_box(result[(0, 0)].real);
    }

    println!("Checksum: {checksum}");
}

fn main() {
    run_matrix_benchmark();
}
