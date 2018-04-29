extern crate num;
extern crate ggez;
extern crate alga;
extern crate nalgebra;
extern crate solar_system_rust;

use nalgebra::Vector2;
use nalgebra::geometry::Point2;
use solar_system_rust::{GraphicSimulation,BodyGraphProperty,BodyShape};
use ggez::*;
use ggez::graphics::DrawMode;
use ggez::graphics::Color;

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut GraphicSimulation::new(ctx).unwrap(); //&mut MainState::new(ctx).unwrap();

    // Sole: Centrato su 400,300, Velocità nulla, Massa: 1.9891 * 10^30 kg = (1.9891 * 10^6 )* 10^24 kg
    let sun_graph = BodyGraphProperty::new(BodyShape::Circle{ radius: 10.0 }, Color::new(1., 1., 1., 1.), DrawMode::Fill);
    state.create_planet("Sole", Point2::new(400.,300.), Vector2::new(0., 0.), 1.9891_e6, sun_graph);
    // Terra: Semiasse: 149 597 887.5 km = 149.597 * 10^6 km, Vel tang = 29.783 km/s = 29.783 * 10^-6 (10*6 km)/s, Massa: 5.9726 * 10^24 kg
    let earth_graph = BodyGraphProperty::new(BodyShape::Circle{ radius: 3.0 }, Color::new(1., 1., 1., 1.), DrawMode::Fill);
    state.create_planet("Terra", Point2::new(400. + 149.597, 300.), Vector2::new(0.,0.5*29.783_e-6), 5.9726, earth_graph);
    
    //state.create_planet("Terra2", Point2::new(400. - 149.597, 300.), Vector2::new(0.,-0.5*29.783_e-6), 5.9726, earth_graph);

    event::run(ctx, state).unwrap();
}
