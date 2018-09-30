use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

pub trait Inverse {
    fn inversed(self) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T: Copy + Clone> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Clone> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point{x,y}
    }
}

//operators overloading
impl<T> Add for Point<T>
    where T: Copy + Clone + Add<T, Output=T>
{
    type Output = Self;

    fn add(self, other: Point<T>) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Point<T>
    where T: Copy + Clone + Sub<T, Output=T>
{
    type Output = Self;

    fn sub(self, other: Point<T>) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> Mul for Point<T>
    where T: Copy + Clone + Mul<T, Output=T>
{
    type Output = Self;

    fn mul(self, other: Point<T>) -> Self {
        Point::new(self.x * other.x, self.y * other.y)
    }
}

impl<T> Mul<T> for Point<T>
    where T: Copy + Clone + Mul<T, Output=T>
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

impl<T> Div for Point<T>
    where T: Copy + Clone + Div<T, Output=T>
{
    type Output = Self;

    fn div(self, other: Point<T>) -> Self {
        Point::new(self.x / other.x, self.y / other.y)
    }
}

impl<T> Div<T> for Point<T>
    where T: Copy + Clone + Div<T, Output=T>
{
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Point::new(self.x / scalar, self.y / scalar)
    }
}

impl Inverse for Point<i8> {
    fn inversed(self) -> Self {
        self * -1
    }
}

impl Inverse for Point<i16> {
    fn inversed(self) -> Self {
        self * -1
    }
}

impl Inverse for Point<i32> {
    fn inversed(self) -> Self {
        self * -1
    }
}

impl Inverse for Point<i64> {
    fn inversed(self) -> Self {
        self * -1
    }
}

impl Inverse for Point<isize> {
    fn inversed(self) -> Self {
        self * -1
    }
}

impl Inverse for Point<f32> {
    fn inversed(self) -> Self {
        self * -1.0
    }
}

impl Inverse for Point<f64> {
    fn inversed(self) -> Self {
        self * -1.0
    }
}

impl<T> fmt::Display for Point<T>
    where T: fmt::Display + Clone + Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_test() {
        let p1 = Point::new(2, 3);
        let p2 = Point::new(4, 6);
        assert_eq!(p1, p2 / 2);

        let calc_result = ((((p1 + p2) / p1) * ((p2 - p1) / p1)) * 2).inversed();
        assert_eq!(calc_result, Point::new(-6, -6));
    }
}