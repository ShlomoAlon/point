//! a point library for working with 2d arrays or vectors.
//!
//! it has all the cardinal direction for point addition as well as scalar multiplication.
//! # Examples
//! here are a couple of examples of array accessing
//! ```
//! use point_index::*;
//! let mut vec = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
//! let mut point1 = Point { x: 1, y: 1 };
//! assert_eq!(vec[point1], 4);
//! // Up is the same as Point {x: 0, y: -1}
//! let point2 = point1 + UP;
//! assert_eq!(vec[point2], 1);
//! // Down is the same as Point {x: 0, y: 1}
//! let point3 = point2 + DOWN * 2;
//! assert_eq!(vec[point3], 7);
//! // Up left is the same as Point {x: -1, y: -1}
//! let point4 = point1 + UP_LEFT;
//! assert_eq!(vec[point4], 0)
//! ```
//! here is a sample of examples of array mutation
//! ```
//! use point_index::*;
//! let mut arr = [[0; 10]; 10];
//! // down right is the same as Point {x: 1, y: 1}
//! for x in 0 .. 10{
//!     arr[DOWN_RIGHT * x] = x
//! }
//! assert_eq!(
//!     arr,
//!    [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//!     [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
//!     [0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 3, 0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 5, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0, 6, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0, 0, 7, 0, 0],
//!     [0, 0, 0, 0, 0, 0, 0, 0, 8, 0],
//!     [0, 0, 0, 0, 0, 0, 0, 0, 0, 9]]
//!
//! )
//! ```
//! quick warning, this library relies on const generics for arrays.
use std::mem;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const UP_LEFT: Point = Point { x: -1, y: -1 };
pub const UP_RIGHT: Point = Point { x: 1, y: -1 };
pub const DOWN_LEFT: Point = Point { x: -1, y: 1 };
pub const DOWN_RIGHT: Point = Point { x: 1, y: 1 };


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// creates a new point from two usizes, if you want to create a point from two isize,
    /// use Point{x: x, y: y}
    /// or Point::new_isize(x, y)
    pub fn new(x: usize, y: usize) -> Point {
        Point {x: x as isize, y: y as isize}
    }
    /// creates a new point from two isize, if you want to create a point from two usize,
    /// use Point::new(x, y)
    pub fn new_isize(x: isize, y: isize) -> Point {
        Point {x, y}
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new_isize(self.x - other.x, self.y - other.y)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, other: isize) -> Point {
        Point::new_isize(self.x * other, self.y * other)
    }
}

impl<A> Index<Point> for Vec<Vec<A>> {
    type Output = A;

    fn index(&self, index: Point) -> &A {
        let x: usize = index.x.try_into().unwrap();
        let y: usize = index.y.try_into().unwrap();
        &self[y][x]
    }
}

impl<A, const SIZE_OUTER: usize, const SIZE_INNER: usize> Index<Point> for [[A; SIZE_INNER]; SIZE_OUTER] {
    type Output = A;

    fn index(&self, index: Point) -> &A {
        let x: usize = index.x.try_into().unwrap();
        let y: usize = index.y.try_into().unwrap();
        &self[y][x]
    }
}

impl <A> IndexMut<Point> for Vec<Vec<A>> {
    fn index_mut(&mut self, index: Point) -> &mut A {
        let x: usize = index.x.try_into().unwrap();
        let y: usize = index.y.try_into().unwrap();
        &mut self[y][x]
    }
}

impl <A, const SIZE_OUTER: usize, const SIZE_INNER: usize> IndexMut<Point> for [[A; SIZE_INNER]; SIZE_OUTER] {
    fn index_mut(&mut self, index: Point) -> &mut A {
        let x: usize = index.x.try_into().unwrap();
        let y: usize = index.y.try_into().unwrap();
        &mut self[y][x]
    }
}


/// This trait is used to get a value from a 2d array. If the operation fails, either because the
/// point is out of bounds or because the conversion from isize to usize fails, None is returned.
pub trait Get{
    type Output;
    fn get_option(&self, point: Point) -> Option<&Self::Output>;
    fn get_mut_option(&mut self, point: Point) -> Option<&mut Self::Output>;
}

impl<A> Get for Vec<Vec<A>> {
    type Output = A;

    fn get_option(&self, point: Point) -> Option<&Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        self.get(y)?.get(x)
    }

    fn get_mut_option(&mut self, point: Point) -> Option<&mut Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        self.get_mut(y)?.get_mut(x)
    }
}

impl<A, const SIZE_INNER: usize, const SIZE_OUTER: usize> Get for [[A; SIZE_INNER]; SIZE_OUTER] {
    type Output = A;

    fn get_option(&self, point: Point) -> Option<&Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        self.get(y)?.get(x)
    }

    fn get_mut_option(&mut self, point: Point) -> Option<&mut Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        self.get_mut(y)?.get_mut(x)
    }
}


/// This trait is used to set a value in a 2d array if it succeeds then the item that was at that
/// index is returned. If the operation fails, either because the point is out of bounds or because
/// the conversion from isize to usize fails, None is returned.
pub trait Set{
    type Output;
    fn set(&mut self, point: Point, value: Self::Output) -> Option<Self::Output>;
}

impl<A> Set for Vec<Vec<A>> {
    type Output = A;

    fn set(&mut self, point: Point, value: Self::Output) -> Option<Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        let inner = self.get_mut(y)?;
        let location = inner.get_mut(x)?;
        Some(mem::replace(location, value))
    }
}

impl<A, const SIZE_INNER: usize, const SIZE_OUTER: usize> Set for [[A; SIZE_INNER]; SIZE_OUTER] {
    type Output = A;

    fn set(&mut self, point: Point, value: Self::Output) -> Option<Self::Output> {
        let x: usize = point.x.try_into().ok()?;
        let y: usize = point.y.try_into().ok()?;
        Some(mem::replace(self.get_mut(y)?.get_mut(x)?, value))
    }
}

/// turns a 2d vec into a flat iterator that returns the point and the value at that point
/// it goes from left to right, top to bottom
/// eventually I will find a way to implement this as a trait without using box to get a decend speed up
/// and better notation
pub fn enumerate_iter_vec<'a, A: 'a>(vec: Vec<Vec<A>>) -> Box<dyn Iterator<Item=(Point,A)> + 'a>{
    Box::new(vec.into_iter().enumerate().flat_map(|(y, row)| {
        row.into_iter().enumerate().map(move |(x, item)| {
            (Point::new(x, y), item)
        })
    }))
}

/// turns a 2d array into a flat iterator that returns the point and the value at that point
/// it goes from left to right, top to bottom
/// eventually I will find a way to implement this as a trait without using box to get a decend speed up
/// and better notation
pub fn enumerate_iter_arr<'a, A: 'a, const INNER: usize, const OUTER: usize>(arr: [[A; INNER]; OUTER]) -> Box<dyn Iterator<Item=(Point,A)> + 'a>{
    Box::new(arr.into_iter().enumerate().flat_map(|(y, row)| {
        row.into_iter().enumerate().map(move |(x, item)| {
            (Point::new(x, y), item)
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![vec![0; 3]; 3];

        assert_eq!(v[Point::new(1, 1)], 0);
    }
    #[test]
    fn it_works_array() {
        let v = [[0; 3]; 3];

        assert_eq!(v[Point::new(1, 1)], 0);
    }
    #[test]
    fn basic_mutation() {
        let mut v = vec![vec![0; 3]; 3];
        let point = Point::new(1, 1);
        v[point] = 1;

        assert_eq!(v[point], 1);
    }
    #[test]
    fn basic_mutation_array() {
        let mut v = [[0; 3]; 3];
        let point = Point::new(1, 1);
        v[point] = 1;

        assert_eq!(v[point], 1);
    }
    #[test]
    fn basic_point_addition() {
        let v = [[1, 1, 1, 2]; 3];
        let point = Point::new(1, 1) + Point::new(2, 0);
        assert_eq!(v[point], 2);
    }
    #[test]
    fn basic_multiplication() {
        let v = [[1, 1, 1, 2]; 3];
        let point = Point::new(1, 1) + RIGHT * 2;
        assert_eq!(v[point], 2);
    }
    #[test]
    fn large_array() {
        let v = [[0; 500]; 1000];
        let point = Point::new(1, 1) + RIGHT * 2;
        assert_eq!(v[point], 0);
    }
}
