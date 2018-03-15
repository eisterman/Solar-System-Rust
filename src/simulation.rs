extern crate num;

use std::fmt;
use num::{Float,Integer,NumCast};
use vector::Vector2;
use sim_elements::Simulable;

pub const DEFAULT_G: f64 = 0.00001;

pub struct Simulation<T,U> where T: Float, U: Integer + Copy {
    delta_t: U,
    total_rel_t: U,
    univ_g: T,
    sim_bodies: Vec<Box<Simulable<T,U>>>,
    sim_datas: Vec<SimData<T>>, // Cache
}

#[derive(Clone,Copy)]
pub struct SimData<T> where T: Float {
    pub pos: Vector2<T>,
    pub vel: Vector2<T>,
    pub mass: T,
}

impl<T,U> Simulation<T,U> where T: Float, U: Integer + Copy + NumCast {
    pub fn new(time_granularity: U, univ_g: T) -> Simulation<T,U> {
        assert_ne!(time_granularity, num::zero());
        Simulation{
            delta_t: time_granularity,
            total_rel_t: num::zero(),
            sim_bodies: Vec::new(),
            univ_g: univ_g,
            sim_datas: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Box<Simulable<T,U>>) {
        let data = body.get_data();
        self.sim_bodies.push(body);
        self.sim_datas.push(data);
    }

    fn evolve_single_delta_t(&mut self) {
        for (i,body) in self.sim_bodies.iter_mut().enumerate() {
            let iter_data: Vec<&SimData<T>> = self.sim_datas.iter().enumerate().filter(|&(n, _)| n != i).map(|(_, v)| v).collect();
            body.simulate_step( iter_data, self.delta_t, self.univ_g);
        }
        for (data,body) in self.sim_datas.iter_mut().zip(&self.sim_bodies) {
            *data = body.get_data();
        }
        self.total_rel_t = self.total_rel_t + num::one();
    }

    pub fn evolve(&mut self, num_of_step: U) {
        for _ in 0..num_of_step.to_usize().unwrap() {
            self.evolve_single_delta_t();
        }
    }
}

impl<T> fmt::Display for SimData<T> where T: Float + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos = {} , Vel= {}", self.pos, self.vel)
    }
}

impl<T,U> fmt::Display for Simulation<T,U> where T: Float + fmt::Display, U: Integer + Copy + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "T={}", self.total_rel_t * self.delta_t)?;
        for body in self.sim_bodies.iter() {
            writeln!(f, "{} - {}", body.get_name(), body.get_data())?;
        }
        Ok(())
    }
}