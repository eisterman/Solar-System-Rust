extern crate num;

use vector::Vector2;

#[derive(Debug)]
struct CorpoCeleste {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    mass: f64,
    name: String,
}

impl CorpoCeleste {
    fn new(name: &str, mass: f64, pos: Vector2<f64>, vel: Vector2<f64>) -> CorpoCeleste {
        CorpoCeleste { position: pos, velocity: vel, mass: mass, name: String::from(name) }
    }

    fn pos(self) -> Vector2<f64> {
        self.position
    }

    fn vel(self) -> Vector2<f64> {
        self.velocity
    }

    fn mass(self) -> f64 {
        self.mass
    }

    fn name(self) -> String {
        self.name
    }
}
