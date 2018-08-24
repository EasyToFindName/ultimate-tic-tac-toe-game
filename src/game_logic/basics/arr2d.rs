use std::fmt;

#[derive(Debug, Clone)]
pub struct Arr2D<T: Clone> {
    internal_repr: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Clone> Arr2D<T> {
    pub fn new(rows: usize, columns: usize, default_value: T) -> Self {
        Arr2D {internal_repr: vec![default_value.clone(); rows * columns],
            rows, columns}
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) -> Result<(), String> {
        if row < self.rows && column < self.columns {
            self.internal_repr[column + row * self.columns] = value;
            Ok(())
        }
            else {
                Err(format!("set: Out of bounds\n\
                          Requested row: {} |  rows: {}\n \
                          Requested column: {} | columns: {}", row, self.rows, column, self.columns))
            }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<&T, String> {
        if row < self.rows && column < self.columns {
            Ok(&self.internal_repr[column + row * self.columns])
        }
            else {
                Err(format!("get: Out of bounds\n\
                          Requested row: {} |  rows: {}\n \
                          Requested column: {} | columns: {}", row, self.rows, column, self.columns))
            }
    }

    pub fn rows(&self) -> usize { self.rows }
    pub fn columns(&self) -> usize { self.columns }
}

// displays
impl<T> fmt::Display for Arr2D<T>
    where T: fmt::Display + Clone
{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());

        for row in 0..self.rows {
            for column in 0..self.columns {
                let _ = write!(f, "{} ", self.internal_repr[column + row * self.columns]);
            }
            result = write!(f, "\n");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arr2d_test() {
        let mut arr = Arr2D::new(5, 5, 1);
        assert_eq!(arr.rows(), 5);
        assert_eq!(arr.columns(), 5);

        arr.set(3, 4, 100).ok();
        assert_eq!(arr.get(3, 4), Ok(&100));
    }
}