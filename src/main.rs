use canvas::Color;
use cgmath::{InnerSpace, Vector3};
use image::Rgb;

use crate::canvas::Canvas;

const IMGX: u32 = 1920;
const IMGY: u32 = 1080;
const BACKGROUND_COLOR: Color = Rgb([255, 255, 255]);

mod canvas;
struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    color: Color,
}

struct Scene {
    spheres: Vec<Sphere>,
}

fn main() {
    let camera_position = Vector3::new(0., 0., 0.);

    let scene = Scene {
        spheres: vec![
            Sphere {
                center: Vector3::new(0., -1., 3.),
                radius: 1.,
                color: Rgb([255, 0, 0]), // Red
            },
            Sphere {
                center: Vector3::new(2., 0., 4.),
                radius: 1.,
                color: Rgb([0, 0, 255]), // Blue
            },
            Sphere {
                center: Vector3::new(-2., 0., 4.),
                radius: 1.,
                color: Rgb([0, 255, 0]), // Green
            },
        ],
    };

    let mut canvas = Canvas::new(IMGX, IMGY);

    for (x, y) in canvas.iter_pixels() {
        let direction = canvas.viewport_direction_at(x, y);
        let color = trace_ray(&scene, &camera_position, direction, 1., f32::INFINITY);
        canvas.put_pixel(x, y, color);
    }

    canvas.save();
}

fn trace_ray(
    scene: &Scene,
    camera_position: &Vector3<f32>,
    direction: Vector3<f32>,
    t_min: f32,
    t_max: f32,
) -> Color {
    let (_, closest_sphere) = scene.spheres.iter().fold(
        (f32::INFINITY, None),
        |(closest_t, closest_sphere), sphere| {
            let (t1, t2) = intersect_ray_sphere(camera_position, &direction, sphere);
            if t1 > t_min && t1 < t_max && t1 < closest_t {
                return (t1, Some(sphere));
            }
            if t2 > t_min && t2 < t_max && t2 < closest_t {
                return (t2, Some(sphere));
            }
            (closest_t, closest_sphere)
        },
    );

    match closest_sphere {
        Some(sphere) => sphere.color,
        None => BACKGROUND_COLOR,
    }
}

fn intersect_ray_sphere(
    camera_position: &Vector3<f32>,
    direction: &Vector3<f32>,
    sphere: &Sphere,
) -> (f32, f32) {
    let ray = camera_position - sphere.center;

    let a_coef = direction.magnitude2();
    let b_coef = 2. * ray.dot(*direction);
    let c_coef = ray.magnitude2() - sphere.radius * sphere.radius;

    let discriminant = b_coef * b_coef - 4. * a_coef * c_coef;
    if discriminant < 0. {
        return (f32::INFINITY, f32::INFINITY);
    }

    let d_sqrt = discriminant.sqrt();

    let t1 = (-b_coef + d_sqrt) / (2. * a_coef);
    let t2 = (-b_coef - d_sqrt) / (2. * a_coef);
    (t1, t2)
}
