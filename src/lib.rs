
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
    fn new(x: isize, y: isize) -> Point {
        Point {x, y}
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, other: isize) -> Point {
        Point::new(self.x * other, self.y * other)
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

}
