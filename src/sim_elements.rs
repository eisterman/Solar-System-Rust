extern crate num;
extern crate ggez;
extern crate alga;
extern crate nalgebra;

use alga::general::Real;
use alga::general::RingCommutative;
use nalgebra::Vector2;
use num::{ToPrimitive,NumCast};
use simulation::SimData;

pub trait Simulable<T,U> where T: Real, U: RingCommutative {
    fn get_name(&self) -> String;
    fn get_data(&self) -> SimData<T>;
    fn simulate_step(&mut self, other_data: Vec<&SimData<T>>, delta_t: U, univ_g: T); 
}

#[derive(Clone)]
pub struct Planet<T> where T: Real {
    name: String,
    data: SimData<T>,
}

impl<T> Planet<T> where T: Real {
    pub fn new(name: &str, pos: Vector2<T>, vel: Vector2<T>, mass: T) -> Planet<T> {
        assert!( mass > num::zero() );
        Planet { name: String::from(name), data: SimData{ pos, vel, mass }}
    }
}

impl<T,U> Simulable<T,U> for Planet<T> where T: Real + NumCast, U: RingCommutative + ToPrimitive{
    fn get_name(&self) -> String { self.name.clone() }
    fn get_data(&self) -> SimData<T> { self.data }

    fn simulate_step(&mut self, other_data: Vec<&SimData<T>>, delta_t: U, univ_g: T) {
        let mut res_force: Vector2<T> = Vector2::new(num::zero(), num::zero());
        let my_data = &mut self.data;
        for body in other_data {
            let distance_vec = body.pos - my_data.pos;
            let single_force = distance_vec * ( univ_g * my_data.mass * body.mass / Real::powi(distance_vec.norm(),3) );
            res_force = res_force + single_force;
        }
        let inst_accel = res_force / my_data.mass;
        let dt = T::from(delta_t).unwrap();
        my_data.vel = my_data.vel + inst_accel * dt;
        my_data.pos = my_data.pos + my_data.vel * dt;
    }
}