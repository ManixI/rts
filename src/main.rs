mod coord;
mod canvas;
mod matrix;
mod ray;
mod sphere;

use std::{f32, ops::Mul};

use canvas::Canvas;
use coord::Coord;
use canvas::color::Color;
use matrix::Matrix;
use ray::Ray;
use sphere::Sphere;

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

fn draw_clock(filename: &str) {
    let mut clockface = Canvas::new(100, 100);
    let white = Color::new(1.0, 1.0, 1.0, 0.0);
    let step = f32::consts::PI/6.0;
    for spot in 0..12 {
        let point = Matrix::translation(50.0, 50.0, 0.0) * Matrix::rotate_z(step * spot as f32) * Matrix::translation(25.0, 0.0, 0.0) * Coord::point(0.0, 0.0, 0.0);
        clockface.set_pixel(point.get_x() as usize, point.get_y() as usize, white);
        println!("{:?}", point);
    }
    let _ = clockface.to_file(filename);
}

fn outline_sphere(filename: &str) {
    let mut canvas = Canvas::new(100, 100);
    let camera_pos = Coord::point(0.0, 0.0, -100.0);
    let mut orb = Sphere::default();
    let red = Color::red();
    orb.apply_transformation(Matrix::translation(0.5, 0.5, 15.0) * Matrix::scaling(10.0, 10.0, 1.0));
    //orb.apply_transformation(Matrix::scaling(5.0, 5.0, 5.0));
    println!("{:?}", orb.get_origin());
    for x in 0..101 {
        for y in 0..101 {
            let ray = Ray::new(camera_pos, Coord::vec((x as f32) / 2.0, (y as f32) / 2.0, 100.0/2.0));
            if ray.intersect(&orb).is_some() {
                canvas.set_pixel(x, y, red);
            }
        }
    }
    let _ = canvas.to_file(filename);
}

fn main() {
    let mut env = Environment::new(-0.01, -0.1, 900, 550);
    env.add_shot(Shot::new(Coord::point(0.0, 1.0, 0.0), Coord::vec(5.0, 8.2, 0.0) * 11.25));
    //println!("{:?}", env);
    while env.run_tick() > 0 {
        //println!("dist: {}", env.get_shots()[0].get_pos().get_x());
    }
    let _ = env.draw_canvas("out.ppm");

    //draw_clock("clock.ppm");
    outline_sphere("sphere.ppm");
}
