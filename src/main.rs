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
    primitives::sphere::Sphere,
    camera::Camera,
    primitives::plane::Plane,
    tex::pattern::Pattern,
    world::World,
    primitives::cube::Cube,
    primitives::cylinder::Cylinder,
    primitives::cone::Cone,
};

#[derive(Debug, Clone, Copy)]
struct Shot {
    pos: Coord,
    vel: Coord,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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
    mat.set_reflection(0.125);
    floor.set_material(mat);

    let mut left_wall = Plane::default();
    left_wall.set_transformation(
        Matrix::translation(0.0, 0.0, 5.0) *
        Matrix::rotate_y(-PI/4.0) *
        Matrix::rotate_x(PI/2.0)
    );
    let mut mat = left_wall.get_material();
    mat.set_texture(Arc::new(Pattern::new_stripe(Arc::new(Color::new(1.0, 0.8, 0.1, 0.0)), Arc::new(Color::white()), Matrix::rotate_y(PI/2.0))));
    left_wall.set_material(mat.clone());

    let mut right_wall = Plane::default();
    right_wall.set_transformation(
        Matrix::translation(0.0, 0.0, 5.0) *
            Matrix::rotate_y_degrees(45.0) *
            Matrix::rotate_x(PI/2.0) *
            Matrix::rotate_z_degrees(180.0)
    );
    let mut mirror_mat = Material::default();
    mirror_mat.set_color(Color::white());
    mirror_mat.set_reflection(1.0);
    mirror_mat.set_diffuse(0.1);
    mirror_mat.set_shininess(0.1);
    mirror_mat.set_specular(0.1);
    //right_wall.set_material(mirror_mat.clone());
    //left_wall.set_material(mirror_mat);


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
                Matrix::scaling(0.05, 0.5, 0.5) 
                    * Matrix::rotate_z(f32::consts::PI / 2.0)
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
    mat.set_texture(
        Arc::new(
            Pattern::new_gradient(
                Arc::new(Color::blue()), 
                Arc::new(Color::red()), 
                Matrix::scaling(2.0, 2.0, 2.0)
                    * Matrix::translation(1.5, 1.5, 1.5)
            )
        )
    );
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
    //world.add_obj(Arc::new(right_wall));
    world.add_obj(Arc::new(middle));
    world.add_obj(Arc::new(left));
    world.add_obj(Arc::new(right));

    let mut cam = Camera::new(800, 800, PI/3.0);
    cam.transform(Matrix::view_transformation(
        Coord::point(0.0, 1.5, -5.0), 
        Coord::point(0.0, 1.0, 0.0), 
    Coord::vec(0.0, 1.0, 0.0)));
    world.set_max_depth(5);
    let canvas = world.render_world_multi(&cam);
    let _ = canvas.to_file("out.ppm");
    
}

#[allow(dead_code)]
fn draw_bubble_sphere() {
    let half = 5.0;

    let mut checker_mat = Material::default();
    checker_mat.set_specular(0.0);
    checker_mat.set_texture(Arc::new(Pattern::new_checker(
        Arc::new(Color::white()),
        Arc::new(Color::black()),
        Matrix::identity(4),
    )));

    // Box: six checker planes enclosing the origin.
    let mut floor = Plane::default();
    floor.set_material(checker_mat.clone());
    floor.set_transformation(Matrix::translation(0.0, -half, 0.0));

    let mut ceiling = Plane::default();
    ceiling.set_material(checker_mat.clone());
    ceiling.set_transformation(Matrix::translation(0.0, half, 0.0) * Matrix::rotate_x(PI));

    let mut back = Plane::default();
    back.set_material(checker_mat.clone());
    back.set_transformation(Matrix::translation(0.0, 0.0, half) * Matrix::rotate_x(PI / 2.0));

    let mut front = Plane::default();
    front.set_material(checker_mat.clone());
    front.set_transformation(Matrix::translation(0.0, 0.0, -half) * Matrix::rotate_x(-PI / 2.0));

    let mut left_wall = Plane::default();
    left_wall.set_material(checker_mat.clone());
    left_wall.set_transformation(Matrix::translation(-half, 0.0, 0.0) * Matrix::rotate_z(-PI / 2.0));

    let mut right_wall = Plane::default();
    right_wall.set_material(checker_mat.clone());
    right_wall.set_transformation(Matrix::translation(half, 0.0, 0.0) * Matrix::rotate_z(PI / 2.0));

    // Glass sphere at the center of the box.
    let mut glass = Sphere::default();
    let mut glass_mat = Material::default();
    glass_mat.set_color(Color::black());
    glass_mat.set_diffuse(0.1);
    glass_mat.set_ambient(0.0);
    glass_mat.set_specular(1.0);
    glass_mat.set_shininess(300.0);
    glass_mat.set_reflection(0.9);
    glass_mat.set_transparency(1.0);
    glass_mat.set_refractive_index(1.5);
    glass.set_material(glass_mat);

    // Air bubble in the center of the glass sphere.
    let mut air = Sphere::default();
    air.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
    let mut air_mat = Material::default();
    air_mat.set_color(Color::black());
    air_mat.set_diffuse(0.1);
    air_mat.set_ambient(0.0);
    air_mat.set_specular(1.0);
    air_mat.set_shininess(300.0);
    air_mat.set_reflection(0.9);
    air_mat.set_transparency(1.0);
    air_mat.set_refractive_index(3.0);
    air.set_material(air_mat);

    let mut world = World::new();
    // Light above and to the left of the sphere, behind the camera.
    world.add_light(Light::new(Coord::point(-4.0, 4.0, -4.5), Color::white()));

    //world.add_obj(Arc::new(floor));
    //world.add_obj(Arc::new(ceiling));
    world.add_obj(Arc::new(back));
    //world.add_obj(Arc::new(front));
    //world.add_obj(Arc::new(left_wall));
    //world.add_obj(Arc::new(right_wall));
    world.add_obj(Arc::new(glass));
    world.add_obj(Arc::new(air));

    // Camera inside the box, looking dead on at the sphere.
    let mut cam = Camera::new(1500, 1500, PI / 3.0);
    cam.transform(Matrix::view_transformation(
        Coord::point(0.0, 0.0, -4.0),
        Coord::point(0.0, 0.0, 0.0),
        Coord::vec(0.0, 1.0, 0.0),
    ));
    world.set_max_depth(6);
    let canvas = world.render_world_multi(&cam);
    let _ = canvas.to_file("bubble.ppm");
}

#[allow(dead_code)]
fn test_cubes() {
    let mut world = World::new();
    world.add_light(Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white()));

    // Checkered floor plane.
    let mut floor = Plane::default();
    let mut floor_mat = Material::default();
    floor_mat.set_specular(0.0);
    floor_mat.set_reflection(0.1);
    floor_mat.set_texture(Arc::new(Pattern::new_checker(
        Arc::new(Color::white()),
        Arc::new(Color::black()),
        Matrix::identity(4),
    )));
    floor.set_material(floor_mat);
    world.add_obj(Arc::new(floor));

    // Back wall plane to give the scene a backdrop.
    let mut back = Plane::default();
    let mut back_mat = Material::default();
    back_mat.set_specular(0.0);
    back_mat.set_color(Color::new(0.6, 0.6, 0.8, 0.0));
    back.set_material(back_mat);
    back.set_transformation(Matrix::translation(0.0, 0.0, 6.0) * Matrix::rotate_x(PI / 2.0));
    world.add_obj(Arc::new(back));

    // Enclosing walls so reflective surfaces always have something to bounce.
    // Placed beyond the light at (-10, 10, -10) so they never block it and stay off-camera.

    // Front wall, behind the camera.
    let mut front = Plane::default();
    let mut front_mat = Material::default();
    front_mat.set_specular(0.0);
    front_mat.set_color(Color::new(0.8, 0.7, 0.6, 0.0));
    front.set_material(front_mat);
    front.set_transformation(Matrix::translation(0.0, 0.0, -12.0) * Matrix::rotate_x(PI / 2.0));
    world.add_obj(Arc::new(front));

    // Left wall.
    let mut left = Plane::default();
    let mut left_mat = Material::default();
    left_mat.set_specular(0.0);
    left_mat.set_color(Color::new(0.7, 0.5, 0.5, 0.0));
    left.set_material(left_mat);
    left.set_transformation(Matrix::translation(-12.0, 0.0, 0.0) * Matrix::rotate_z(PI / 2.0));
    world.add_obj(Arc::new(left));

    // Right wall.
    let mut right = Plane::default();
    let mut right_mat = Material::default();
    right_mat.set_specular(0.0);
    right_mat.set_color(Color::new(0.5, 0.7, 0.5, 0.0));
    right.set_material(right_mat);
    right.set_transformation(Matrix::translation(12.0, 0.0, 0.0) * Matrix::rotate_z(PI / 2.0));
    world.add_obj(Arc::new(right));

    // Ceiling.
    let mut ceiling = Plane::default();
    let mut ceiling_mat = Material::default();
    ceiling_mat.set_specular(0.0);
    ceiling_mat.set_color(Color::new(0.6, 0.6, 0.7, 0.0));
    ceiling.set_material(ceiling_mat);
    ceiling.set_transformation(Matrix::translation(0.0, 12.0, 0.0));
    world.add_obj(Arc::new(ceiling));

    // Red cube, axis-aligned, sitting on the floor.
    let mut cube_a = Cube::default();
    let mut mat_a = Material::default();
    mat_a.set_color(Color::red());
    mat_a.set_diffuse(0.7);
    mat_a.set_specular(0.3);
    cube_a.set_material(mat_a);
    cube_a.set_transformation(
        Matrix::translation(-2.5, 1.0, 0.5) * Matrix::scaling(1.0, 1.0, 1.0),
    );
    world.add_obj(Arc::new(cube_a));

    // Green cube, rotated and squashed.
    let mut cube_b = Cube::default();
    let mut mat_b = Material::default();
    mat_b.set_color(Color::green());
    mat_b.set_reflection(0.3);
    cube_b.set_material(mat_b);
    cube_b.set_transformation(
        Matrix::translation(2.5, 0.75, 0.0)
            * Matrix::rotate_y(PI / 4.0)
            * Matrix::scaling(0.75, 0.75, 0.75),
    );
    world.add_obj(Arc::new(cube_b));

    // Striped tall cube in the back.
    let mut cube_c = Cube::default();
    let mut mat_c = Material::default();
    mat_c.set_specular(0.3);
    mat_c.set_texture(Arc::new(Pattern::new_stripe(
        Arc::new(Color::new(1.0, 0.8, 0.1, 0.0)),
        Arc::new(Color::white()),
        Matrix::scaling(0.25, 0.25, 0.25),
    )));
    cube_c.set_material(mat_c);
    cube_c.set_transformation(
        Matrix::translation(0.0, 1.5, 3.0)
            * Matrix::rotate_y(PI / 6.0)
            * Matrix::scaling(0.6, 1.5, 0.6),
    );
    world.add_obj(Arc::new(cube_c));

    // Blue sphere in front, between the cubes.
    let mut sphere_a = Sphere::default();
    let mut sphere_mat_a = Material::default();
    sphere_mat_a.set_color(Color::blue());
    sphere_mat_a.set_diffuse(0.7);
    sphere_mat_a.set_specular(0.3);
    sphere_a.set_material(sphere_mat_a);
    sphere_a.set_transformation(Matrix::translation(0.0, 1.0, -1.5));
    world.add_obj(Arc::new(sphere_a));

    // Small reflective sphere resting on the green cube's side.
    let mut sphere_b = Sphere::default();
    let mut sphere_mat_b = Material::default();
    sphere_mat_b.set_color(Color::purple());
    sphere_mat_b.set_diffuse(0.7);
    sphere_mat_b.set_specular(0.3);
    sphere_mat_b.set_shininess(100.0);
    sphere_mat_b.set_reflection(0.15);
    sphere_b.set_material(sphere_mat_b);
    sphere_b.set_transformation(
        Matrix::translation(-2.5, 2.5, 0.5) * Matrix::scaling(0.5, 0.5, 0.5),
    );
    world.add_obj(Arc::new(sphere_b));

    let mut cam = Camera::new(1200, 1200, PI / 3.0);
    cam.transform(Matrix::view_transformation(
        Coord::point(0.0, 2.5, -7.0),
        Coord::point(0.0, 1.0, 0.0),
        Coord::vec(0.0, 1.0, 0.0),
    ));
    world.set_max_depth(5);
    let canvas = world.render_world_multi(&cam);
    let _ = canvas.to_file("cubes.ppm");
}

#[allow(dead_code)]
fn draw_saturn_v() {
    let mut world = World::new();
    world.set_max_depth(6);

    world.add_light(Light::new(Coord::point(-6.0, 15.0, -8.0), Color::white()));
    world.add_light(Light::new(
        Coord::point(5.0, 10.0, -4.0),
        Color::new(0.4, 0.4, 0.45, 0.0),
    ));

    // Ground: concrete grey with slight reflection
    let mut floor = Plane::default();
    let mut m = Material::default();
    m.set_color(Color::new(0.52, 0.52, 0.52, 0.0));
    m.set_specular(0.05);
    m.set_reflection(0.05);
    floor.set_material(m);
    world.add_obj(Arc::new(floor));

    // Mirror: vertical reflective plane behind the rocket at z=3.5
    let mut mirror = Plane::default();
    mirror.set_transformation(
        Matrix::translation(0.0, 0.0, 3.5) * Matrix::rotate_x(PI / 2.0),
    );
    let mut m = Material::default();
    m.set_color(Color::new(0.95, 0.95, 0.96, 0.0));
    m.set_reflection(0.95);
    m.set_diffuse(0.05);
    m.set_specular(0.9);
    m.set_shininess(300.0);
    mirror.set_material(m);
    world.add_obj(Arc::new(mirror));

    // Rocket palette
    let off_white   = Color::new(0.93, 0.93, 0.91, 0.0);
    let blk         = Color::new(0.06, 0.06, 0.06, 0.0);
    let dark_grey   = Color::new(0.20, 0.20, 0.22, 0.0);
    let engine_col  = Color::new(0.30, 0.28, 0.26, 0.0);

    // Material closures (capture palette by copy)
    let white_mat = || {
        let mut m = Material::default();
        m.set_color(off_white);
        m.set_specular(0.2);
        m.set_diffuse(0.85);
        m
    };
    let black_mat = || {
        let mut m = Material::default();
        m.set_color(blk);
        m.set_specular(0.1);
        m.set_diffuse(0.7);
        m
    };
    let dark_mat = || {
        let mut m = Material::default();
        m.set_color(dark_grey);
        m.set_specular(0.15);
        m.set_diffuse(0.75);
        m
    };
    let engine_mat = || {
        let mut m = Material::default();
        m.set_color(engine_col);
        m.set_specular(0.4);
        m.set_shininess(80.0);
        m.set_diffuse(0.6);
        m
    };

    // ── S-IC First Stage (y 0.0–4.5, radius 0.5) ──────────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.5, 1.0, 0.5),
        white_mat(), 0.0, 4.5, true,
    )));
    // Black interstage band at top of S-IC
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.51, 1.0, 0.51),
        black_mat(), 4.1, 4.5, true,
    )));

    // ── S-II Second Stage (y 4.5–7.0, radius 0.5) ─────────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.5, 1.0, 0.5),
        white_mat(), 4.5, 7.0, true,
    )));
    // Black interstage band at top of S-II
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.51, 1.0, 0.51),
        black_mat(), 6.7, 7.0, true,
    )));

    // ── S-IVB Third Stage (y 7.0–8.5, radius 0.42) ────────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.42, 1.0, 0.42),
        white_mat(), 7.0, 8.5, true,
    )));

    // ── Instrument Unit (y 8.5–8.7, dark ring) ────────────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.42, 1.0, 0.42),
        dark_mat(), 8.5, 8.7, true,
    )));

    // ── Service Module (y 8.7–9.8, radius 0.35) ───────────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::scaling(0.35, 1.0, 0.35),
        white_mat(), 8.7, 9.8, true,
    )));

    // ── Command Module nose cone (y 9.8–10.6) ─────────────────────
    // Cone local: min=-1 (r=1 → world r=0.35) max=0 (tip)
    // World y: local -1 → -1*0.8+10.6 = 9.8 ✓   local 0 → 10.6 ✓
    world.add_obj(Arc::new(Cone::new(
        Matrix::translation(0.0, 10.6, 0.0) * Matrix::scaling(0.35, 0.8, 0.35),
        white_mat(), -1.0, 0.0, true,
    )));

    // ── Launch Escape System tower (y 10.6–12.4, thin) ────────────
    world.add_obj(Arc::new(Cylinder::new(
        Matrix::translation(0.0, 10.6, 0.0) * Matrix::scaling(0.04, 1.0, 0.04),
        dark_mat(), 0.0, 1.8, true,
    )));
    // LES abort motor cap
    let mut les_cap = Sphere::default();
    les_cap.set_transformation(
        Matrix::translation(0.0, 12.4, 0.0) * Matrix::scaling(0.12, 0.12, 0.12),
    );
    let mut m = Material::default();
    m.set_color(dark_grey);
    m.set_specular(0.3);
    les_cap.set_material(m);
    world.add_obj(Arc::new(les_cap));

    // ── Fins: 4 thin cubes at base ────────────────────────────────
    let fin_mat = {
        let mut m = Material::default();
        m.set_color(off_white);
        m.set_specular(0.1);
        m
    };
    for (tx, tz, sx, sz) in [
        ( 0.62_f32,  0.0_f32, 0.13_f32, 0.05_f32),
        (-0.62,      0.0,     0.13,      0.05    ),
        ( 0.0,       0.62,    0.05,      0.13    ),
        ( 0.0,      -0.62,    0.05,      0.13    ),
    ] {
        let mut fin = Cube::default();
        fin.set_transformation(
            Matrix::translation(tx, 0.7, tz) * Matrix::scaling(sx, 0.7, sz),
        );
        fin.set_material(fin_mat.clone());
        world.add_obj(Arc::new(fin));
    }

    // ── F-1 Engine nozzles: 5 cones (1 centre + 4 outer) ──────────
    // Cone min=-1 (bell opening, r=0.14 at world y=0) max=0 (throat at y=0.28)
    for (ex, ez) in [
        ( 0.0_f32,  0.0_f32),
        ( 0.28,     0.0    ),
        (-0.28,     0.0    ),
        ( 0.0,      0.28   ),
        ( 0.0,     -0.28   ),
    ] {
        world.add_obj(Arc::new(Cone::new(
            Matrix::translation(ex, 0.28, ez) * Matrix::scaling(0.14, 0.28, 0.14),
            engine_mat(), -1.0, 0.0, true,
        )));
    }

    let resolution = 100;
    let mut cam = Camera::new(7*resolution, 10*resolution, PI / 3.0);
    cam.transform(Matrix::view_transformation(
        Coord::point(0.0, 6.5, -9.0),
        Coord::point(0.0, 5.5, 0.0),
        Coord::vec(0.0, 1.0, 0.0),
    ));

    let canvas = world.render_world_multi(&cam);
    let _ = canvas.to_file("saturn_v.ppm");
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
    //draw_bubble_sphere();
    //test_cubes();
    draw_saturn_v();
}
