extern crate num;

use vector::Vector2;
use num::Float;
use num::Integer;

pub trait Simulable<T,U> where T: Float, U: Integer {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Vector2<T>;
    fn get_velocity(&self) -> Vector2<T>;
    fn get_mass(&self) -> T;
    fn simulate_step(&mut self, other_sim: Vec<Box<Simulable<T,U>>>, delta_t: U);
}

pub struct Planet<T> where T: Float {
    name: String,
    pos: Vector2<T>,
    vel: Vector2<T>,
    mass: T,
}

impl<T> Planet<T> where T: Float {
    fn new(name: &str, pos: Vector2<T>, vel: Vector2<T>, mass: T) -> Planet<T> {
        assert!( mass > num::zero() );
        Planet { name: String::from(name), pos, vel, mass }
    }
}

impl<T,U> Simulable<T,U> for Planet<T> where T: Float, U: Integer {
    fn get_name(&self) -> String { self.name.clone() }
    fn get_position(&self) -> Vector2<T> { self.pos }
    fn get_velocity(&self) -> Vector2<T> { self.vel }
    fn get_mass(&self) -> T { self.mass }
    fn simulate_step(&mut self, other_sim: Vec<Box<Simulable<T,U>>>, delta_t: U) {
        unimplemented!()
    }
}