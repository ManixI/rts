mod coord;
mod canvas;

use coord::Coord;

#[derive(Debug, Clone, Copy)]
struct Shot {
    pos: Coord,
    vel: Coord,
    test: usize,
}

impl Shot {
    fn new(pos: Coord, vel: Coord) -> Self {
        Shot { pos, vel , test: 0}
    }

    fn run_tick(&mut self, effects: Coord) {
        self.vel += effects;
        self.pos += self.vel;
        self.test += 1;
        //println!("({}, {})", self.pos.get_x(), self.pos.get_y());
        //println!("{:?}, {}", self.vel, self.test);
        //*self
    }

    fn get_pos(&self) -> Coord {
        self.pos
    }

    fn get_height(&self) -> f32 {
        self.pos.get_y()
    }
}

#[derive(Debug)]
struct Environment {
    wind: Coord,
    gravity: Coord,
    combine: Coord,
    shots: Vec<Shot>
}

impl Environment {
    fn new(wind: f32, gravity: f32) -> Self {
        Environment {
            wind: Coord::vec(wind, 0.0, 0.0), 
            gravity: Coord::vec(0.0, gravity, 0.0), 
            combine: Coord::vec(wind, 0.0, 0.0) + Coord::vec(0.0, gravity, 0.0),
            shots: Vec::<Shot>::new()
        }
    }

    fn add_shot(&mut self, shot: Shot) {
        self.shots.push(shot);
    }

    fn run_tick(&mut self) -> usize {
        //self.shots.iter().map(|s| s.run_tick(self.combine)).collect::<Vec<Shot>>();
        /*self.shots = self.shots
        .iter_mut()
        .map(|s| s.run_tick(self.combine))
        .filter(|s| s.get_height() > 0.0)
        .collect::<Vec<Shot>>();*/

        for shot in self.shots.iter_mut() {
            shot.run_tick(self.combine);
        }
        self.shots.retain(|s| s.get_height() > 0.0);

        /*self.shots.retain(|s: &mut Shot| {
            //let s = *s;
            s.run_tick(self.combine);
            s.get_pos().get_y() > 0.0
        });*/

        self.shots.len()
    }
}

fn main() {
    let mut env = Environment::new(-0.01, -0.1);
    env.add_shot(Shot::new(Coord::point(0.0, 1.0, 0.0), Coord::vec(1.0, 1.0, 0.0)));
    println!("{:?}", env);
    while env.run_tick() > 0 {}
}
