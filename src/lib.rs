extern crate num;
extern crate ggez;
extern crate alga;
extern crate nalgebra;
extern crate gnuplot;

mod sim_elements;
mod simulation;

use nalgebra::Vector2;
use nalgebra::geometry::Point2;
use sim_elements::Planet;
use simulation::Simulation;
use ggez::*;
use ggez::event::Keycode;
use ggez::event::Mod;
use ggez::graphics::DrawMode;
use ggez::graphics::Color;
use ggez::graphics::Rect;

const BEZIER_TOLERANCE: f32 = 2.0;

#[derive(Debug,Clone)]
struct PhysicalCollectedData {
    total_angular_momentum: Vec<f64>,
    system_kinetic_energy: Vec<f64>,
    system_potential_energy: Vec<f64>,
    center_of_mass: Vec<Point2<f64>>,
}

impl PhysicalCollectedData {
    fn new() -> PhysicalCollectedData {
        PhysicalCollectedData{  total_angular_momentum: Vec::new(), 
                                system_kinetic_energy: Vec::new(), 
                                system_potential_energy: Vec::new(),
                                center_of_mass: Vec::new() }
    }

    fn add_data_tuple(&mut self, angular_momentum: f64, kinetic_energy: f64, potential_energy: f64, center_of_mass: Point2<f64>) {
        self.total_angular_momentum.push(angular_momentum);
        self.system_kinetic_energy.push(kinetic_energy);
        self.system_potential_energy.push(potential_energy);
        self.center_of_mass.push(center_of_mass);
    }
}

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
    engine: Simulation<f64,i64>,
    graph_property: Vec<BodyGraphProperty>,
    stored_data: PhysicalCollectedData,
}

impl GraphicSimulation {
    pub fn new(_ctx: &mut Context) -> GameResult<GraphicSimulation> { //64000 default gran
        let s = GraphicSimulation { engine: Simulation::new(64000, 6.67_e-14), graph_property: Vec::<BodyGraphProperty>::new(), stored_data: PhysicalCollectedData::new() };
        Ok(s)
    }

    pub fn create_planet(&mut self, name: &str, pos: Point2<f64>, vel: Vector2<f64>, mass: f64, graph_prop: BodyGraphProperty) { //TODO: GraphicProperty input!
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

#[inline]
fn point_downscale(input: Point2<f64>) -> Point2<f32> {
    Point2::<f32>::new(input.x as f32, input.y as f32)
}

// Esprimo in sistema di misura Pos: 10^6 km, Tempo: s, massa: 10^24 kg
// G = 6.67 * 10^-14 (10^6 km)^3/((10^24 kg) s^2)
// G = 6.67 * 10^4 km^3/((10^24 kg) s^2)
impl event::EventHandler for GraphicSimulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.engine.evolve(1);
        //Calculate L, Ek, Ep and insert in self.stored_data
        let l = self.engine.calculate_angular_momentum_cdm();
        let ek = self.engine.calculate_kinetic_energy();
        let ep = self.engine.calculate_potential_energy();
        let cdm = self.engine.calculate_cdm();
        self.stored_data.add_data_tuple(l, ek, ep, cdm);
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
                        point_downscale(body.pos),
                        radius,
                        BEZIER_TOLERANCE)?;
                }
                Rectangle { w, h }      => {
                    graphics::rectangle(ctx,
                        graph_prop.draw_mode,
                        Rect::new(body.pos.x as f32, body.pos.y as f32, w, h))?; //x e y indicano il centro o lo spigolo inferiore sinistro?
                }
                Square { l }            => {
                    graphics::rectangle(ctx,
                        graph_prop.draw_mode,
                        Rect::new(body.pos.x as f32, body.pos.y as f32, l, l))?; //x e y indicano il centro o lo spigolo inferiore sinistro?
                }
            }
        }
        graphics::present(ctx);

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match _keycode {
            Keycode::Return => {
                println!("Elaborazione dati in corso...");
                // Elaborazione dati
                use gnuplot::{Figure, Caption, Color};
                // Angular Momentum
                let y1 = &self.stored_data.total_angular_momentum;
                let x1: Vec<f64> = (0..y1.len()).map(|x| {x as f64}).collect();
                let mut fg1 = Figure::new();
                fg1.axes2d().lines(&x1, y1, &[Caption("Total Angular Momentum"), Color("black")]);
                fg1.show();
                // Total Mechanical Energy
                let y2: Vec<f64> = self.stored_data.system_kinetic_energy.iter().zip(self.stored_data.system_potential_energy.iter()).map(|(x,y)| { x + y }).collect();
                let x2: Vec<f64> = (0..y2.len()).map(|x| {x as f64}).collect();
                let mut fg2 = Figure::new();
                fg2.axes2d().lines(&x2, &y2, &[Caption("Total Mechanical Energy"), Color("red")]);
                fg2.show();
                // Center of Mass scatter plot
                let (x3, y3): (std::vec::Vec<f64>, std::vec::Vec<f64>) = self.stored_data.center_of_mass.iter().fold( (Vec::<f64>::new(), Vec::<f64>::new()), |(mut x,mut y), p| {x.push(p.x); y.push(p.y); (x,y)});
                let mut fg3 = Figure::new();
                fg3.axes2d().points(x3, y3, &[Caption("Center of Mass progression"), Color("blue")]);
                fg3.show();
                // Center of Mass (t,x) and (t,y) plots
                /*let z4 = (0..y3.len()).map(|x| {x as f64}).collect();
                let mut fg4 = Figure::new();
                fg4.axes3d().surface(mat: X, num_rows: usize, num_cols: usize, dimensions: Option<(f64, f64, f64, f64)>, options: &[Caption("Center of Mass time progression"), Color("blue")]);
                fg4.show();*/
                // Quit Event
                _ctx.quit().unwrap();
            }
            _ => {}
        }
    }
}
