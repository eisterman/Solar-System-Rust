extern crate num;

use num::Float;
use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vector2<T> where T: Float {
    x: T,
    y: T,
}

impl<T: Add<Output=T>> Add for Vector2<T> where T: Float {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T> { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T: Sub<Output=T>> Sub for Vector2<T> where T: Float {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T> { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<T> Mul<T> for Vector2<T> where T: Mul<Output=T> + Float {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2::<T> { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T> Div<T> for Vector2<T> where T: Div<Output=T> + Float {
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Vector2<T> {
        assert!(rhs != num::zero()); //TODO: How to manipulate rhs = num::zero()?
        Vector2::<T> { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T> Vector2<T> where T: Float {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2::<T> { x, y }
    }
    
    pub fn new_zero() -> Vector2<T> {
        Vector2::<T> { x: num::zero(), y: num::zero() }
    }

    pub fn get_x(&self) -> T { self.x }
    pub fn get_y(&self) -> T { self.y }

    pub fn norm(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

//TODO: How to make 5 * Vec2? I really need it?

#[cfg(test)]
mod tests{
    use vector::*;

    #[test]
    fn new_zero_test() {
        assert_eq!(Vector2::new_zero(), Vector2::<f64> {x:0., y:0.} );
    }

    #[test]
    fn new_test() {
        assert_eq!(Vector2::<f64> { x: 4., y: 1.}, Vector2::new(4., 1.));
    }

    #[test]
    fn add_trait() {
        assert_eq!(Vector2::<f64> { x: 4., y: 1.}, Vector2::<f64> {x:3., y:1.5} + Vector2::<f64> {x:1., y:-0.5} );
    }

    #[test]
    fn sub_trait() {
        assert_eq!(Vector2::<f64> { x: 2., y: 2.}, Vector2::<f64> {x:3., y:1.5} - Vector2::<f64> {x:1., y:-0.5} );
    }

    #[test]
    fn mul_trait() {
        assert_eq!(Vector2::<f64> { x: 6., y: 3.}, Vector2::<f64> {x:3., y:1.5} * 2. ); 
    }

    #[test]
    fn div_trait() {
        assert_eq!(Vector2::<f64> { x: 6., y: 3.} / 2. , Vector2::<f64> {x:3., y:1.5});
    }

    #[test]
    fn norm_test() {
        assert_eq!(Vector2::<f64> { x: 4., y: -3.}.norm(), 5_f64);
    }
}