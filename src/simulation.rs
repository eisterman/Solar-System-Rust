extern crate num;

use std::fmt;
use num::{Float,Integer,NumCast};
use vector::Vector2;
use sim_elements::{Simulable,Planet};

pub const default_G: f64 = 5.6;

pub struct Simulation<T,U> where T: Float, U: Integer + Copy {
    delta_t: U,
    total_rel_t: U,
    G: T,
    sim_bodies: Vec<Box<Simulable<T,U>>>,
    sim_datas: Vec<Sim_Data<T>>, // Cache
}

#[derive(Clone,Copy)]
pub struct Sim_Data<T> where T: Float {
    pub id: usize,
    pub pos: Vector2<T>,
    pub vel: Vector2<T>,
    pub mass: T,
}

impl<T,U> Simulation<T,U> where T: Float, U: Integer + Copy + NumCast {
    pub fn new(time_granularity: U, G: T) -> Simulation<T,U> {
        assert!(time_granularity != num::zero());
        Simulation{
            delta_t: time_granularity,
            total_rel_t: num::zero(),
            sim_bodies: Vec::new(),
            G: G,
            sim_datas: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Box<Simulable<T,U>>) {
        let data = Sim_Data { 
            id: self.sim_bodies.len(),
            pos: body.get_position(),
            vel: body.get_velocity(),
            mass: body.get_mass(),
        };
        self.sim_bodies.push(body);
        self.sim_datas.push(data);
    }

    fn evolve_single_delta_t(&mut self) {
        for (i,body) in self.sim_bodies.iter_mut().enumerate() {
            body.simulate_step(i, &self.sim_datas, self.delta_t, self.G);
        }
        for (data,body) in self.sim_datas.iter_mut().zip(&self.sim_bodies) {
            data.pos = body.get_position();
            data.vel = body.get_velocity();
            data.mass = body.get_mass();
        }
        self.total_rel_t = self.total_rel_t + num::one();
    }

    pub fn evolve(&mut self, num_of_step: U) {
        for _ in 0..num_of_step.to_usize().unwrap() {
            self.evolve_single_delta_t();
        }
    }
}

impl<T,U> fmt::Display for Simulation<T,U> where T: Float, U: Integer + Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", 5, 6)
    }
}