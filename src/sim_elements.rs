extern crate num;

use vector::Vector2;
use num::Float;
use num::Integer;
use num::NumCast;
use simulation::SimData;
use std::slice::Iter;
use std::iter::Skip;

pub trait Simulable<T,U> where T: Float, U: Integer {
    fn get_name(&self) -> String;
    fn get_data(&self) -> SimData<T>;
    fn simulate_step(&mut self, other_data: Skip<Iter<SimData<T>>>, delta_t: U, univ_g: T); 
}

#[derive(Clone)]
pub struct Planet<T> where T: Float {
    name: String,
    data: SimData<T>,
}

impl<T> Planet<T> where T: Float {
    pub fn new(name: &str, pos: Vector2<T>, vel: Vector2<T>, mass: T) -> Planet<T> {
        assert!( mass > num::zero() );
        Planet { name: String::from(name), data: SimData{ pos, vel, mass }}
    }
}

impl<T,U> Simulable<T,U> for Planet<T> where T: Float, U: Integer + NumCast {
    fn get_name(&self) -> String { self.name.clone() }
    fn get_data(&self) -> SimData<T> { self.data }

    fn simulate_step(&mut self, other_data: Skip<Iter<SimData<T>>>, delta_t: U, univ_g: T) {
        let mut res_force: Vector2<T> = Vector2::new_zero();
        let my_data = &mut self.data;
        for body in other_data {
            let distance_vec = body.pos - my_data.pos;
            let single_force = distance_vec * ( univ_g * my_data.mass * body.mass / distance_vec.norm().powi(3) );
            res_force = res_force + single_force;
        }
        let inst_accel = res_force / my_data.mass;
        let dt = T::from(delta_t).unwrap();
        my_data.vel = my_data.vel + inst_accel * dt;
        my_data.pos = my_data.pos + my_data.vel * dt;
    }
}