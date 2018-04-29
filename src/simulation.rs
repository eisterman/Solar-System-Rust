extern crate num;
extern crate ggez;
extern crate alga;
extern crate nalgebra;

use std::fmt::Debug;
use nalgebra::Scalar;
use nalgebra::geometry::Point2;
use nalgebra::Vector2;
use alga::general::Real;
use alga::general::RingCommutative;
use num::ToPrimitive;
use sim_elements::Simulable;

//pub const DEFAULT_G: f64 = 1000.;

pub struct Simulation<T,U> where T: Scalar, U: RingCommutative + Copy {
    delta_t: U,
    total_rel_t: U,
    univ_g: T,
    sim_bodies: Vec<Box<Simulable<T,U>>>,
    sim_datas: Vec<SimData<T>>, // Cache
}

#[derive(Clone,Copy)]
pub struct SimData<T> where T: Scalar {
    pub pos: Point2<T>,
    pub vel: Vector2<T>,
    pub mass: T,
}

impl<T,U> Simulation<T,U> where T: Scalar + Real, U: Copy + ToPrimitive + RingCommutative + Debug {
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

    pub fn get_sim_data(&self) -> Vec<SimData<T>> {
        self.sim_datas.clone()
    }

    pub fn get_total_rel_t(&self) -> U {
        self.total_rel_t
    }

    pub fn calculate_cdm(&self) -> Point2<T> {
        let x: T = num::zero();
        let y: T = num::zero();
        let numerator = Point2::from_coordinates(self.sim_datas.iter().map(|x| -> Vector2<T> {x.pos * x.mass - Point2::<T>::origin()}).sum());
        let denominator: T = self.sim_datas.iter().map(|x| x.mass).fold(num::zero(), |acc,x| {acc + x});
        numerator / denominator
    }

    pub fn calculate_angular_momentum_cdm(&self) -> T {
        let cdm = self.calculate_cdm();
        // Lx = Ly = 0; Lz = PosX * VelY - PosY * VelX con posizioni relative al CDM
        self.sim_datas.iter().map(|obj| (obj.pos.x - cdm.x)* obj.vel.y - (obj.pos.y - cdm.y) * obj.vel.x).fold(num::zero(), |acc,x| {acc + x})
    }
}

//TODO: Find a solution! 0.5 break the generics structure
impl Simulation<f32,i32> {
    pub fn calculate_kinetic_energy(&self) -> f32 {
        self.sim_datas.iter().map(|obj| { 0.5 * obj.mass * (obj.vel.x * obj.vel.x + obj.vel.y * obj.vel.y) } ).sum()
    }

    pub fn calculate_potential_energy(&self) -> f32 {
        let mut cumulative = 0_f32;
        let n_body = self.sim_datas.len();
        for i in 0 .. n_body {
            for j in (i+1) .. n_body {
                let numerator = self.univ_g * self.sim_datas[i].mass * self.sim_datas[j].mass;
                let r = self.sim_datas[j].pos - self.sim_datas[i].pos;
                let denominator = r.x.hypot(r.y);
                cumulative -= numerator/denominator;
            }
        }
        cumulative
    }
}

use std::fmt;

impl<T> fmt::Display for SimData<T> where T: Real + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos = {} , Vel= {}", self.pos, self.vel)
    }
}

impl<T,U> fmt::Display for Simulation<T,U> where T: Real + fmt::Display, U: RingCommutative + Copy + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "T={}", self.total_rel_t * self.delta_t)?;
        for body in self.sim_bodies.iter() {
            writeln!(f, "{} - {}", body.get_name(), body.get_data())?;
        }
        Ok(())
    }
}