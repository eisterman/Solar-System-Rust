extern crate num;

use std::fmt;
use num::{Float,Integer};
use vector::Vector2;
use sim_elements::{Simulable,Planet};

const default_G: f64 = 5.6;

struct Simulation<T,U> where T: Float, U: Integer{
    delta_t: U,
    total_rel_t: U,
    G: T,
    sim_bodies: Vec<Box<Simulable<T,U>>>,
}

impl<T,U> Simulation<T,U> where T: Float, U: Integer {
    fn new(time_granularity: U) -> Simulation<T,U> {
        assert!(time_granularity != num::zero());
        Simulation{ delta_t: time_granularity, total_rel_t: num::zero(), sim_bodies: Vec::new(), G: default_G as T }
    }

    fn add_body(&mut self body: Box<Simulable<T,U>>) {
        body.sim_bodies.push(body);
    }

    fn evolve_single_delta_t(&mut self) {
        for body_box in self.sim_bodies.iter_mut() {
            body_box.simulate_step(&sim_bodies, self.delta_t, self.G);
        }
        self.total_rel_t += 1
    }

    fn evolve(&mut self, num_of_step: U) {
        for i in 0..num_of_step {
            self.evolve_single_delta_t();
        }
    }
}

impl<T,U> fmt::Display for Simulation<T,U> where T: Float, U: Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.delta_t, self.total_rel_t)
    }
}