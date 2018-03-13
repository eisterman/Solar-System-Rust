extern crate num;

mod vector;
mod sim_elements;
mod simulation;

use vector::Vector2;
use sim_elements::Planet;
use simulation::{Simulation,DEFAULT_G};

fn main() {
    println!("Start...");
    let a = Box::new(Planet::new("Sole", Vector2::new_zero(), Vector2::new_zero(), 5.));
    let b = Box::new(Planet::new("Terra", Vector2::new(5.,0.), Vector2::new(0.,1.), 1.));
    let mut engine = Simulation::new(1, DEFAULT_G);
    engine.add_body(a);
    engine.add_body(b);
    engine.evolve(5);
    println!("Completed!");
}
