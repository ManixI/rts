mod coord;
mod canvas;
mod matrix;
mod ray;
mod sphere;
mod light;
mod material;

use std::f32;

use canvas::Canvas;
use coord::Coord;
use canvas::color::Color;
use light::{lighting, Light};
use material::Material;
use matrix::Matrix;
use ray::{intersection::Intersect, Ray};
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

#[allow(dead_code)]
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

fn draw_multiple_spheres(filename: &str, spheres: &[Sphere], per_row: usize, resolution: usize) {
    let mut canvas = Canvas::new(resolution * per_row, (spheres.len() / per_row) * resolution);
    
    for (idx, orb) in spheres.iter().enumerate() {
        let row = idx / per_row;
        let col = idx % per_row;
        println!("drawing sphere {} at ({}, {})", idx, row, col);
        let drawn = outline_sphere(
            filename, 
            resolution, 
            orb.clone(),
            Light::new(Coord::point(10.0, 10.0, -10.0), Color::white())    
        );
        for x in 0..resolution {
            for y in 0..resolution {
                canvas.set_pixel(
                    x + col * resolution, 
                    y + row * resolution, 
                    drawn.get_pixel(x, y));
            }
        }
    }
    let _ = canvas.to_file(filename);
}


fn outline_sphere(filename: &str, resolution: usize, orb: Sphere, light: Light) -> Canvas {
    let size = resolution;
    let mut canvas = Canvas::new(size, size);

    //let mut orb = Sphere::default();
    //let mut mat = Material::default();

    //mat.set_shininess(100.0);
    //mat.set_specular(-0.9);

    //mat.set_color(Color::red());
    //orb.set_material(mat);
    //orb.apply_transformation(Matrix::scaling(1.0, 0.5, 1.0));

    let camera_pos = Coord::point(0.0, 0.0, -5.0);
    let wall_pos = Coord::point(0.0, 0.0, 10.0);
    let wall_size = 7.0;

    let light = light;

    //let background_color = orb.get_material().get_color().inverse();
    let background_color = Color::black();
    let pixel_size = wall_size / size as f32;

    for y in 0..size {
        let world_y = wall_size/2.0 - pixel_size * y as f32;
        for x in 0..size {
            let world_x = -(wall_size/2.0) + pixel_size * x as f32;
            let pos = Coord::point(world_x, world_y, wall_pos.get_z());
            let ray = Ray::new(camera_pos, (pos - camera_pos).normalized());
            let xs = ray.intersect(&orb);
            if xs.is_some() {
                let xs = xs.unwrap();
                let point = ray.position(xs[0].get_time());
                let normal = orb.normal_at(point);
                let cam_v = -ray.get_direction();
                canvas.set_pixel(
                    x, 
                    y, 
                    lighting(xs[0].get_object().get_material(), light, point, cam_v, normal)
                );
            }
            else {
                canvas.set_pixel(x, y, background_color);
            }
        }
    }

    let _ = canvas.to_file(filename);
    canvas
}

// TODO: make this a test case for lighting func
fn draw_test_spheres() {
    let orbs = vec![Sphere::default(); 6];
    let small_vals = vec![0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
    let shiny_vals = vec![0.0, 5.0, 10.0, 50.0, 100.0, 500.0];

    // shininess
    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_shininess(shiny_vals[i]);
        mat.set_specular(small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("spec-pos.ppm", &t_orbs[0..6], 2, 400);


    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_shininess(50.0);
        mat.set_specular(-small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("spec-neg.ppm", &t_orbs[0..6], 2, 400);


    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_diffuse(small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("diff-pos.ppm", &t_orbs[0..6], 2, 400);


    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_diffuse(-small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("diff-neg.ppm", &t_orbs[0..6], 2, 400);


    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_ambient(small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("amb-pos.ppm", &t_orbs[0..6], 2, 400);
    
    let mut t_orbs = orbs.clone();
    for i in 0..6 {
        let mut mat = Material::default();
        mat.set_ambient(-small_vals[i]);
        mat.set_color(Color::red());
        t_orbs[i].set_material(mat);
    }
    draw_multiple_spheres("amb-neg.ppm", &t_orbs[0..6], 2, 400);


    let mut orb = Sphere::default();
    let mut mat = Material::default();
    mat.set_color(Color::red());
    orb.set_material(mat);
    let light = Light::new(Coord::point(10.0, 10.0, -10.0), Color::blue());
    outline_sphere("color-add.ppm", 400, orb.clone(), light);

    mat.set_color(Color::purple());
    mat.set_diffuse(-0.5);
    outline_sphere("color-sub.ppm", 400, orb.clone(), light);

    mat.set_color(Color::red());
    mat.set_diffuse(0.5);
    mat.set_ambient(10.0);
    outline_sphere("color-add-amb.ppm", 400, orb, light);
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
    //let _ = outline_sphere("sphere.ppm", 400, Sphere::default());

    let mut orbs = vec![Sphere::default(); 8];
    //let mat_values = vec![0.0, 5.0, 10.0, 50.0, 100.0, 500.0];
    let mat_values = vec![0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
    /*let mat_values = vec![
        Color::red(),
        Color::blue(),
        Color::green(),
        Color::yellow(),
        Color::purple(),
        Color::turquoise(),
    ];*/
    for i in 0..mat_values.len() {
        let mut mat = Material::default();
        mat.set_color(Color::red());
        //mat.set_ambient(0.5);s
        //mat.set_diffuse(mat_values[i]);
        //mat.set_shininess(50.0);
        //mat.set_specular(shiny_values[i]);
        orbs[i].set_material(mat);
    }
    //draw_multiple_spheres("composite.ppm", &orbs[5..6], 1, 400);
    draw_test_spheres();
}
