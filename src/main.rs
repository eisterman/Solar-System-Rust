extern crate num;
extern crate ggez;
extern crate alga;
extern crate nalgebra;

mod sim_elements;
mod simulation;

use nalgebra::Vector2;
use nalgebra::geometry::Point2;
use sim_elements::Planet;
use simulation::Simulation;
use ggez::*;
use ggez::graphics::DrawMode;

struct MainState {
    engine: Simulation<f32,i32>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Sole: Centrato su 400,300, Velocità nulla, Massa: 1.9891 * 10^30 kg = (1.9891 * 10^6 )* 10^24 kg
        let a = Box::new(Planet::new("Sole", Point2::new(400.,300.), Vector2::new(0., 0.), 1.9891_e6));
        // Terra: Semiasse: 149 597 887.5 km = 149.597 * 10^6 km, Vel tang = 29.783 km/s = 29.783 * 10^-6 (10*6 km)/s, Massa: 5.9726 * 10^24 kg
        let b = Box::new(Planet::new("Terra", Point2::new(400. + 149.597, 300.), Vector2::new(0.,29.783_e-6), 5.9726));
        let mut engine = Simulation::new(64000, 6.67_e-14);
        engine.add_body(a);
        engine.add_body(b);
        let s = MainState { engine };
        Ok(s)
    }
}

// Esprimo in sistema di misura Pos: 10^6 km, Tempo: s, massa: 10^24 kg
// G = 6.67 * 10^-14 (10^6 km)^3/((10^24 kg) s^2)
// G = 6.67 * 10^4 km^3/((10^24 kg) s^2)
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.engine.evolve(1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let data = self.engine.get_sim_data();
        let sun = data[0];
        let earth = data[1];
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         sun.pos,
                         10.0,
                         2.0)?;
        graphics::circle(ctx,
                         DrawMode::Fill,
                         earth.pos,
                         3.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

/*fn main() {
    println!("Start...");
    let a = Box::new(Planet::new("Sole", Point2::new(-1.,0.), Vector2::new(0., 0.), 5.));
    let b = Box::new(Planet::new("Terra", Point2::new(5.,0.), Vector2::new(0.,1.), 1.));
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
}*/
