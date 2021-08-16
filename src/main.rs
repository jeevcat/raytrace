use canvas::Color;
use cgmath::{InnerSpace, Vector3};
use image::{Pixel, Rgb};

use crate::canvas::Canvas;

const IMGX: u32 = 1920;
const IMGY: u32 = 1080;
const BACKGROUND_COLOR: Color = Rgb([255, 255, 255]);

mod canvas;
struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    color: Color,
    specular: Option<f32>,
}

enum Light {
    Ambient {
        intensity: f32,
    },
    Point {
        intensity: f32,
        position: Vector3<f32>,
    },
    Directional {
        intensity: f32,
        direction: Vector3<f32>,
    },
}

struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

fn main() {
    let camera_position = Vector3::new(0., 0., 0.);

    let scene = Scene {
        spheres: vec![
            Sphere {
                center: Vector3::new(0., -1., 3.),
                radius: 1.,
                color: Rgb([255, 0, 0]), // Red
                specular: Some(500.),    // Shiny
            },
            Sphere {
                center: Vector3::new(2., 0., 4.),
                radius: 1.,
                color: Rgb([0, 0, 255]), // Blue
                specular: Some(500.),    // Shiny
            },
            Sphere {
                center: Vector3::new(-2., 0., 4.),
                radius: 1.,
                color: Rgb([0, 255, 0]), // Green
                specular: Some(10.),     // Somewhat shiny
            },
            Sphere {
                center: Vector3::new(0., -5001., 0.),
                radius: 5000.,
                color: Rgb([255, 255, 0]), // Yellow
                specular: Some(1000.),     // Very shiny
            },
        ],
        lights: vec![
            Light::Ambient { intensity: 0.2 },
            Light::Point {
                intensity: 0.6,
                position: Vector3::new(2., 1., 0.),
            },
            Light::Directional {
                intensity: 0.2,
                direction: Vector3::new(1., 4., 4.),
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
    // TODO: can this be faster?
    let (closest_t, closest_sphere) = scene
        .spheres
        .iter()
        .flat_map(|sphere| {
            intersect_ray_sphere(camera_position, &direction, sphere)
                .iter()
                .map(|t| (*t, sphere))
                .collect::<Vec<_>>()
        })
        .fold(
            (f32::INFINITY, None),
            |(closest_t, closest_sphere), (t, sphere)| {
                if t > t_min && t < t_max && t < closest_t {
                    return (t, Some(sphere));
                }
                (closest_t, closest_sphere)
            },
        );

    match closest_sphere {
        Some(sphere) => {
            let surface_position = camera_position + closest_t * direction;
            let surface_normal = (surface_position - sphere.center).normalize();
            let lighting = compute_lighting(
                scene,
                &surface_position,
                &surface_normal,
                -direction,
                sphere.specular,
            );
            sphere.color.map(|c| (c as f32 * lighting) as u8)
        }
        None => BACKGROUND_COLOR,
    }
}

fn intersect_ray_sphere(
    camera_position: &Vector3<f32>,
    direction: &Vector3<f32>,
    sphere: &Sphere,
) -> [f32; 2] {
    let ray = camera_position - sphere.center;

    let a_coef = direction.magnitude2();
    let b_coef = 2. * ray.dot(*direction);
    let c_coef = ray.magnitude2() - sphere.radius * sphere.radius;

    let discriminant = b_coef * b_coef - 4. * a_coef * c_coef;
    if discriminant < 0. {
        return [f32::INFINITY, f32::INFINITY];
    }

    let d_sqrt = discriminant.sqrt();

    let t1 = (-b_coef + d_sqrt) / (2. * a_coef);
    let t2 = (-b_coef - d_sqrt) / (2. * a_coef);
    [t1, t2]
}

fn compute_lighting(
    scene: &Scene,
    surface_position: &Vector3<f32>,
    surface_normal: &Vector3<f32>,
    surface_to_camera: Vector3<f32>,
    specularity: Option<f32>,
) -> f32 {
    scene.lights.iter().fold(0., |acc, l| {
        acc + match l {
            Light::Ambient { intensity } => *intensity,
            Light::Point {
                intensity,
                position,
            } => {
                let incident_ray = position - surface_position;
                diffuse(&incident_ray, surface_normal, *intensity)
                    + specular(
                        &incident_ray,
                        surface_normal,
                        surface_to_camera,
                        *intensity,
                        specularity,
                    )
            }
            Light::Directional {
                intensity,
                direction,
            } => {
                diffuse(direction, surface_normal, *intensity)
                    + specular(
                        direction,
                        surface_normal,
                        surface_to_camera,
                        *intensity,
                        specularity,
                    )
            }
        }
    })
}

fn diffuse(incident_ray: &Vector3<f32>, surface_normal: &Vector3<f32>, intensity: f32) -> f32 {
    let dot = surface_normal.dot(*incident_ray);
    if dot > 0. {
        intensity * dot / (surface_normal.magnitude() * incident_ray.magnitude())
    } else {
        0.
    }
}

fn specular(
    incident_ray: &Vector3<f32>,
    surface_normal: &Vector3<f32>,
    surface_to_camera: Vector3<f32>,
    intensity: f32,
    specularity: Option<f32>,
) -> f32 {
    if let Some(specularity) = specularity {
        let reflected_ray = 2. * surface_normal * surface_normal.dot(*incident_ray) - incident_ray;
        let dot = reflected_ray.dot(surface_to_camera);
        if dot > 0. {
            return intensity
                * (dot / (reflected_ray.magnitude() * surface_to_camera.magnitude()))
                    .powf(specularity);
        }
    }
    0.
}
