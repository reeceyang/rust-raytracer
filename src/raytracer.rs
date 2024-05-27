use std::ops::Add;

use crate::geometry::*;

pub fn canvas_to_viewport(scene: &Scene, x: f64, y: f64) -> Vec3 {
    let vw = scene.viewport.w;
    let vh = scene.viewport.h;
    let cw = scene.canvas.w;
    let ch = scene.canvas.h;
    Vec3::new(x * vw / cw, y * vh / ch, scene.camera_dist)
}

/// finds the color of the sphere at the nearest intersection of the ray
/// origin + dir * t within the given range of t
pub fn trace_ray(scene: &Scene, origin: Vec3, dir: Vec3, t_min: f64, t_max: f64) -> Color {
    let closest_sphere = scene
        .spheres
        .iter()
        // get the values of t at which the ray intersects the sphere
        .map(|sphere| (intersect_ray_sphere(origin, dir, sphere), sphere))
        // filter out values of t not in the given range
        .filter(|((t1, t2), _)| *t1 >= t_min && *t1 <= t_max && *t2 >= t_min && *t2 <= t_max)
        // get the closer value of t
        .map(|((t1, t2), sphere)| (t1.min(t2), sphere))
        // filter out t values at infinity
        .filter(|(t, _)| *t < f64::INFINITY)
        // find the sphere with the least t value
        .min_by(|(t, _), (u, _)| t.total_cmp(u));
    if let Some((t, sphere)) = closest_sphere {
        let point = origin + t * dir;
        let normal = (point - sphere.center).normalize();
        return sphere.color * compute_lighting(scene, point, normal, -dir, sphere.material);
    }
    scene.bg_color
}

/// finds the values of t for which the ray origin + dir * t intersects with
/// the sphere
fn intersect_ray_sphere(origin: Vec3, dir: Vec3, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = origin - sphere.center;

    let a = dir.dot(dir);
    let b = 2.0 * co.dot(dir);
    let c = co.dot(co) - r * r;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    let t1 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
    let t2 = (-b - f64::sqrt(discriminant)) / (2.0 * a);

    (t1, t2)
}

/// compute the lighting at the point with the given normal vector
fn compute_lighting(
    scene: &Scene,
    point: Vec3,
    normal: Vec3,
    point_to_camera: Vec3,
    material: Material,
) -> f64 {
    scene
        .lights
        .iter()
        .map(|light| {
            let calculate_intensity = |intensity: f64, light_dir: Vec3| {
                let n_dot_l = normal.dot(light_dir);
                let diffuse = if n_dot_l > 0.0 {
                    n_dot_l / (normal.length() * light_dir.length())
                } else {
                    0.0
                };
                let specular = match material {
                    Material::Specular(s) => {
                        let reflect_dir = 2.0 * normal * normal.dot(light_dir) - light_dir;
                        let r_dot_v = reflect_dir.dot(point_to_camera);
                        if r_dot_v > 0.0 {
                            intensity
                                * f64::powf(
                                    r_dot_v / (reflect_dir.length() * point_to_camera.length()),
                                    s,
                                )
                        } else {
                            0.0
                        }
                    }
                    Material::Matte => 0.0,
                };
                diffuse + specular
            };

            match light {
                Light::Ambient(light) => light.intensity,
                Light::Point(light) => calculate_intensity(light.intensity, light.position - point),
                Light::Directional(light) => calculate_intensity(light.intensity, light.dir),
            }
        })
        .fold(0.0, Add::add)
}
