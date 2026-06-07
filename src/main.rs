use std::{f32, sync::Arc};
use core::f32::consts::PI;

use rtc::{
    canvas::Canvas,
    coord::Coord,
    tex::color::Color,
    light::{lighting, Light},
    material::Material,
    matrix::Matrix,
    ray::Ray,
    renderable::{Renderable, RenderableBase},
    sphere::Sphere,
    camera::Camera,
    plane::Plane,
    tex::pattern::Pattern,
    world::World,
};

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
                    lighting(xs[0].get_object(), light, point, cam_v, normal, false)
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
    orb.set_material(mat.clone());
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

fn draw_scene() {
    let mut floor = Plane::default();
    //floor.apply_transformation(Matrix::translation(0.0, 0.0, 0.0));
    let mut mat = Material::default();
    //mat.set_color(Color::new(1.0, 0.9, 0.9, 0.0));
    mat.set_specular(0.0);
    mat.set_texture(
        Arc::new(
            Pattern::new_checker(
                Arc::new(Color::white()), 
                Arc::new(Color::black()), 
                Matrix::translation(-10.0, -10.0, 0.0) // BUG: this translation makes the checker significantly clearer then the identity
                //Matrix::identity(4)
            )
        )
    );
    mat.set_reflection(0.5);
    floor.set_material(mat);

    let mut left_wall = Plane::default();
    left_wall.set_transformation(
        Matrix::translation(0.0, 0.0, 5.0) *
        Matrix::rotate_y(-PI/4.0) *
        Matrix::rotate_x(PI/2.0)
    );
    left_wall.get_material().set_texture(Arc::new(Pattern::new_stripe(Arc::new(Color::new(1.0, 0.8, 0.1, 0.0)), Arc::new(Color::white()), Matrix::rotate_z(PI/4.0))));

    let mut right_wall = Plane::default();
    right_wall.set_transformation(
        Matrix::translation(0.0, 0.0, 5.0) *
        Matrix::rotate_y(PI/4.0) *
        Matrix::rotate_x(PI/2.0) 
    );


    let mut middle = Sphere::default();
    middle.set_transformation(Matrix::translation(-0.5, 1.0, 0.5));
    let mut mat = Material::default();
    mat.set_diffuse(0.7);
    mat.set_specular(0.3);
    mat.set_texture(
        Arc::new(
            Pattern::new_stripe(
                Arc::new(Color::red()),
                Arc::new(Color::green()),
                Matrix::scaling(0.05, 1.0, 1.0) * Matrix::rotate_y(f32::consts::PI / 4.0)
            )
        )
    );
    middle.set_material(mat);

    let mut right = Sphere::default();
    right.set_transformation(
        Matrix::translation(1.5, 0.5, -0.5) *
        Matrix::scaling(0.5, 0.5, 0.5)
    );
    let mut mat = Material::default();
    //mat.set_color(Color::new(0.5, 1.0, 0.1, 0.0));
    mat.set_diffuse(0.7);
    mat.set_specular(0.3);
    mat.set_texture(Arc::new(Pattern::new_gradient(Arc::new(Color::blue()), Arc::new(Color::red()), Matrix::scaling(0.25, 0.25, 0.25))));
    right.set_material(mat);

    let mut left = Sphere::default();
    left.set_transformation(
        Matrix::translation(-1.5, 0.33, -0.75) *
        Matrix::scaling(0.33, 0.33, 0.33)
    );
    let mut mat = Material::default();
    mat.set_color(Color::new(1.0, 0.8, 0.1, 0.0));
    mat.set_diffuse(0.7);
    mat.set_specular(0.3);
    left.set_material(mat);

    let mut world = World::new();
    let light = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
    world.add_light(light);

    world.add_obj(Arc::new(floor));
    world.add_obj(Arc::new(left_wall));
    world.add_obj(Arc::new(right_wall));
    world.add_obj(Arc::new(middle));
    world.add_obj(Arc::new(left));
    world.add_obj(Arc::new(right));

    let mut cam = Camera::new(1000, 1000, PI/3.0);
    cam.transform(Matrix::view_transformation(
        Coord::point(0.0, 1.5, -5.0), 
        Coord::point(0.0, 1.0, 0.0), 
    Coord::vec(0.0, 1.0, 0.0)));
    let canvas = world.render_world_multi(&cam);
    let _ = canvas.to_file("out.ppm");
    
}

fn main() {
    //let mut env = Environment::new(-0.01, -0.1, 900, 550);
    //env.add_shot(Shot::new(Coord::point(0.0, 1.0, 0.0), Coord::vec(5.0, 8.2, 0.0) * 11.25));
    //println!("{:?}", env);
    //while env.run_tick() > 0 {
        //println!("dist: {}", env.get_shots()[0].get_pos().get_x());
    //}
    //let _ = env.draw_canvas("out.ppm");

    //draw_clock("clock.ppm");
    //let _ = outline_sphere("sphere.ppm", 400, Sphere::default());

    //draw_test_spheres();
    draw_scene();
}
