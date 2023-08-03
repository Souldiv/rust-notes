use std::ops::{Add, Sub, Mul, Div};
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::marker::Copy;

pub trait EuclideanDist {
    fn calculate(&self) -> f64;
}

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

pub trait Sqrt {
    fn sqrt(&self) -> f64;
}

impl<T: Mul + Add + Sub + Div> Sqrt for T {
    fn sqrt(&self) -> f64{
        (*self).sqrt()
    }
}


impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn get_data_mut(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }
}

pub fn euclidean_distance<T>(point: &Point<T>) -> f64
where
    T: Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Clone
        + PartialOrd
        + Copy
        + Sqrt
{
    let diff_x = point.x.clone();
    let diff_y = point.y.clone();
    let distance = (diff_x * diff_x + diff_y * diff_y).sqrt();
    distance
}