extern crate num;

use std::fmt;
use num::{Float,Integer};
use vector::Vector2;
use sim_elements::{Simulable,Planet};

struct Simulation<T,U> where T: Float, U: Integer{
    delta_t: U,
    total_t: U,
    sim_bodies: Vec<Box<Simulable<T,U>>>,
}

impl<T,U> Simulation<T,U> where T: Float, U: Integer {
    fn new(time_granularity: U) -> Simulation<T,U> {
        assert!(time_granularity != num::zero());
        Simulation{ delta_t: time_granularity, total_t: num::zero(), sim_bodies: Vec::new() }
    }

    fn add_body(&mut self body: Box<Simulable<T,U>>) {
        body.sim_bodies.push(body);
    }

    fn evolve_single_delta_t(&mut self) {
        unimplemented!()
    }

    fn evolve(&mut self, num_of_step: U) {
        unimplemented!()
    }
}

impl<T,U> fmt::Display for Simulation<T,U> where T: Float, U: Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.delta_t, self.total_t)
    }
}