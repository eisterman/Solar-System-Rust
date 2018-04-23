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
use ggez::graphics::Color;
use ggez::graphics::Rect;

const BEZIER_TOLERANCE: f32 = 2.0;

#[derive(Clone,Copy)]
pub enum BodyShape {
    Circle { radius: f32 },
    Rectangle { w: f32, h:f32 },
    Square { l: f32 }
} // add special shape POINT (single pixel at every scale)

#[derive(Clone,Copy)]
pub struct BodyGraphProperty {
    shape: BodyShape,
    color: Color, 
    draw_mode: DrawMode,
}

impl BodyGraphProperty {
    pub fn new(shape: BodyShape, color: Color, draw_mode: DrawMode) -> BodyGraphProperty {
        BodyGraphProperty { shape, color, draw_mode } 
    }
}

pub struct GraphicSimulation {
    engine: Simulation<f32,i32>,
    graph_property: Vec<BodyGraphProperty>,
}

impl GraphicSimulation {
    pub fn new(_ctx: &mut Context) -> GameResult<GraphicSimulation> {
        let s = GraphicSimulation { engine: Simulation::new(64000, 6.67_e-14), graph_property: Vec::<BodyGraphProperty>::new() };
        Ok(s)
    }

    pub fn create_planet(&mut self, name: &str, pos: Point2<f32>, vel: Vector2<f32>, mass: f32, graph_prop: BodyGraphProperty) { //TODO: GraphicProperty input!
        let body = Box::new(Planet::new(name, pos, vel, mass));
        
        use self::BodyShape::*;
        match graph_prop.shape {
            Circle { radius }       => if radius <= 0. { panic!("Circle radius need to be > 0.") }
            Rectangle { w, h }      => if w <= 0. || h <= 0. { panic!("Rectangle x and y need to be > 0.") },
            Square { l }            => if l <= 0. { panic!("Square l need to be > 0.") },
        }

        self.engine.add_body(body);
        self.graph_property.push(graph_prop);
    }
}

// Esprimo in sistema di misura Pos: 10^6 km, Tempo: s, massa: 10^24 kg
// G = 6.67 * 10^-14 (10^6 km)^3/((10^24 kg) s^2)
// G = 6.67 * 10^4 km^3/((10^24 kg) s^2)
impl event::EventHandler for GraphicSimulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.engine.evolve(1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let data = self.engine.get_sim_data();        

        graphics::clear(ctx);

        for (body, graph_prop) in data.iter().zip(self.graph_property.iter()) {
            graphics::set_color(ctx, graph_prop.color)?;
            use self::BodyShape::*;
            match graph_prop.shape {
                Circle { radius }       => {
                    graphics::circle(ctx,
                        graph_prop.draw_mode,
                        body.pos,
                        radius,
                        BEZIER_TOLERANCE)?;
                }
                Rectangle { w, h }      => {
                    graphics::rectangle(ctx,
                        graph_prop.draw_mode,
                        Rect::new(body.pos.x, body.pos.y, w, h))?; //x e y indicano il centro o lo spigolo inferiore sinistro?
                }
                Square { l }            => {
                    graphics::rectangle(ctx,
                        graph_prop.draw_mode,
                        Rect::new(body.pos.x, body.pos.y, l, l))?; //x e y indicano il centro o lo spigolo inferiore sinistro?
                }
            }
        }
        graphics::present(ctx);

        Ok(())
    }
}
