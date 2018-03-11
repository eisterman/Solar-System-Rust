extern crate num;

use vector::Vector2;
use num::Float;
use num::Integer;
use num::NumCast;

pub trait Simulable<T,U> where T: Float, U: Integer {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Vector2<T>;
    fn get_velocity(&self) -> Vector2<T>;
    fn get_mass(&self) -> T;
    fn simulate_step(&mut self, other_sim: &Vec<Box<Simulable<T,U>>>, delta_t: U, G: T);
}

pub struct Planet<T> where T: Float {
    name: String,
    pos: Vector2<T>,
    vel: Vector2<T>,
    mass: T,
}

impl<T> Planet<T> where T: Float {
    pub fn new(name: &str, pos: Vector2<T>, vel: Vector2<T>, mass: T) -> Planet<T> {
        assert!( mass > num::zero() );
        Planet { name: String::from(name), pos, vel, mass }
    }
}

impl<T,U> Simulable<T,U> for Planet<T> where T: Float, U: Integer + NumCast {
    fn get_name(&self) -> String { self.name.clone() }
    fn get_position(&self) -> Vector2<T> { self.pos }
    fn get_velocity(&self) -> Vector2<T> { self.vel }
    fn get_mass(&self) -> T { self.mass }

    fn simulate_step(&mut self, other_sim: &Vec<Box<Simulable<T,U>>>, delta_t: U, G: T) {
        let mut res_force: Vector2<T> = Vector2::new_zero();
        for body_box in other_sim.iter() {
            if body_box.get_name() == self.name { continue; }
            let distance_vec = body_box.get_position() - self.pos;
            let single_force = distance_vec * ( G * self.mass * body_box.get_mass() / distance_vec.norm().powi(3) );
            res_force = res_force + single_force;
        }
        let inst_accel = res_force / self.mass;
        let dt = T::from(delta_t).unwrap();
        self.vel = self.vel + inst_accel * dt;
        self.pos = self.pos + self.vel * dt;
    }
}