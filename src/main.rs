extern crate num;

mod vector;
mod sim_elements;
mod simulation;

use vector::Vector2;
use sim_elements::Planet;
use simulation::{Simulation,DEFAULT_G};

fn main() {
    println!("Start...");
    let a = Box::new(Planet::new("Sole", Vector2::new(-1.,0.), Vector2::new_zero(), 5.));
    let b = Box::new(Planet::new("Terra", Vector2::new(5.,0.), Vector2::new(0.,1.), 1.));
    let mut engine = Simulation::new(1, DEFAULT_G);
    println!("{}", engine);
    engine.add_body(a);
    println!("{}", engine);
    engine.add_body(b);
    println!("{}", engine);
    engine.evolve(1);
    println!("{}", engine);
    engine.evolve(5);
    println!("{}", engine);
    println!("Completed!");
}
