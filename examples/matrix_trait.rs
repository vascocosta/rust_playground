use std::{
    error::Error,
    ops::{Add, Div, Mul, Sub},
};

trait Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn rows(&self) -> Option<usize>;
    fn cols(&self) -> Option<usize>;
    fn get(&self, row: usize, col: usize) -> Option<&T>;
    fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Matrix2<T> {
    elements: Vec<Vec<T>>,
}

impl<T> Matrix2<T>
where
    T: Clone + Default + Add<Output = T>,
{
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            elements: vec![vec![T::default(); cols]; rows],
        }
    }

    fn from_slice(elements: &[&[T]]) -> Self {
        let mut new_elements = Vec::new();

        for row in elements {
            new_elements.push(row.to_vec());
        }

        Self {
            elements: new_elements,
        }
    }
}

impl<T> Matrix<T> for Matrix2<T>
where
    T: Clone + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn rows(&self) -> Option<usize> {
        Some(self.elements.len())
    }

    fn cols(&self) -> Option<usize> {
        if self.elements.is_empty() {
            Some(0)
        } else {
            Some(self.elements.first()?.len())
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.elements.get(row)?.get(col)
    }

    fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), Box<dyn Error>> {
        let row = self.elements.get_mut(row).ok_or("row does not exit")?;
        let col = row.get_mut(col).ok_or("column does not exist")?;
        *col = value;

        Ok(())
    }
}

fn main() {
    let mut m: Matrix2<f32> = Matrix2::new(3, 3);
    println!("{:?}", m);

    let row1 = [1, 2, 5, 7];
    let row2 = [3, 4, 8, 5];
    let row3 = [3, 4, 8, 5];
    let elements = vec![&row1[..], &row2[..], &row3[..]];
    let m2 = Matrix2::from_slice(&elements);
}
