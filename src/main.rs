mod coord;
mod canvas;
mod matrix;

use canvas::Canvas;
use coord::Coord;
use canvas::color::Color;

#[derive(Debug, Clone, Copy)]
struct Shot {
    pos: Coord,
    vel: Coord,
}

impl Shot {
    fn new(pos: Coord, vel: Coord) -> Self {
        Shot { pos, vel }
    }

    fn run_tick(&mut self, effects: Coord) {
        self.vel += effects;
        self.pos += self.vel.normalized();
    }

    fn get_pos(&self) -> Coord {
        self.pos
    }

    fn get_height(&self) -> f32 {
        self.pos.get_y()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Environment {
    wind: Coord,
    gravity: Coord,
    combine: Coord,
    shots: Vec<Shot>,
    canvas: Canvas
}

#[allow(dead_code)]
impl Environment {
    fn new(wind: f32, gravity: f32, height: usize, width: usize) -> Self {
        Environment {
            wind: Coord::vec(wind, 0.0, 0.0), 
            gravity: Coord::vec(0.0, gravity, 0.0), 
            combine: Coord::vec(wind, 0.0, 0.0) + Coord::vec(0.0, gravity, 0.0),
            shots: Vec::<Shot>::new(),
            canvas: Canvas::new(height, width),
        }
    }

    fn add_shot(&mut self, shot: Shot) {
        self.shots.push(shot);
    }

    fn run_tick(&mut self) -> usize {
        self.shots.retain(|s| s.get_height() > 0.0);
        for shot in self.shots.iter_mut() {
            shot.run_tick(self.combine);
            let s = shot.get_pos();
            self.canvas.set_pixel(s.get_x() as usize, self.canvas.get_height() - s.get_y() as usize, Color::new(1.0, 0.0, 0.0, 0.0));
        }
        self.shots.len()
    }

    fn get_shots(&self) -> &Vec<Shot> {
        &self.shots
    }

    fn draw_canvas(&self, filename: &str) -> std::io::Result<()> {
        self.canvas.to_file(filename)
    }
}

fn main() {
    let mut env = Environment::new(-0.01, -0.1, 900, 550);
    env.add_shot(Shot::new(Coord::point(0.0, 1.0, 0.0), Coord::vec(5.0, 8.2, 0.0) * 11.25));
    //println!("{:?}", env);
    while env.run_tick() > 0 {
        //println!("dist: {}", env.get_shots()[0].get_pos().get_x());
    }
    let _ = env.draw_canvas("out.ppm");
}
