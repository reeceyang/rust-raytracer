use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    /// angle between self and rhs in radians
    pub fn angle_between(self, rhs: Vec3) -> f64 {
        f64::acos(self.dot(rhs) / (self.length() * rhs.length()))
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -(self.x * rhs.z - self.z * rhs.x),
            z: self.x * rhs.y - self.y * rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3x3 {
    pub col1: Vec3,
    pub col2: Vec3,
    pub col3: Vec3,
}

impl Mul<Vec3> for Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.col1.x * rhs.x + self.col2.x * rhs.y + self.col3.x * rhs.z,
            y: self.col1.y * rhs.x + self.col2.y * rhs.y + self.col3.y * rhs.z,
            z: self.col1.z * rhs.x + self.col2.z * rhs.y + self.col3.z * rhs.z,
        }
    }
}

impl Mat3x3 {
    pub const IDENTITY: Mat3x3 = Mat3x3 {
        col1: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        col2: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        col3: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    pub fn new(col1: Vec3, col2: Vec3, col3: Vec3) -> Self {
        Mat3x3 { col1, col2, col3 }
    }

    // adapted from https://stackoverflow.com/a/18574797
    /// get the rotation matrix of rotating to dir from up
    /// up must be nonzero
    pub fn rotation_mat(dir: Vec3, up: Vec3) -> Self {
        let x_axis = up.cross(dir).normalize();
        if x_axis.x.is_nan() {
            return Mat3x3::IDENTITY;
        }
        let y_axis = dir.cross(x_axis).normalize();

        Mat3x3 {
            col1: Vec3 {
                x: x_axis.x,
                y: y_axis.x,
                z: dir.x,
            },
            col2: Vec3 {
                x: x_axis.y,
                y: y_axis.y,
                z: dir.y,
            },
            col3: Vec3 {
                x: x_axis.z,
                y: y_axis.z,
                z: dir.z,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// multiplies u by f and clamps the product to the valid range of u8 values
fn clamped_mul(u: u8, f: f64) -> u8 {
    let product = ((u as f64) * f).clamp(u8::MIN as f64, u8::MAX as f64);
    product as u8
}

fn clamped_add(u: u8, v: u8) -> u8 {
    let sum = (u as u16 + v as u16).clamp(u8::MIN as u16, u8::MAX as u16);
    sum as u8
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: clamped_add(self.r, rhs.r),
            g: clamped_add(self.g, rhs.g),
            b: clamped_add(self.b, rhs.b),
            a: clamped_add(self.a, rhs.a),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: clamped_mul(self.r, rhs),
            g: clamped_mul(self.g, rhs),
            b: clamped_mul(self.b, rhs),
            a: clamped_mul(self.a, rhs),
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: clamped_mul(rhs.r, self),
            g: clamped_mul(rhs.g, self),
            b: clamped_mul(rhs.b, self),
            a: clamped_mul(rhs.a, self),
        }
    }
}

impl Color {
    pub const RED: Color = Color {
        r: 0xff,
        g: 0,
        b: 0,
        a: 0xff,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 0xff,
        b: 0,
        a: 0xff,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 0xff,
        a: 0xff,
    };
    pub const WHITE: Color = Color {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    pub fn as_u8_slice(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub color: Color,
    pub specularity: Specularity,
    /// 0.0 (not reflective at all) to 1.0 (a perfect mirror)
    pub reflectiveness: f64,
}

#[derive(Clone, Copy)]
pub enum Specularity {
    Specular(f64),
    Matte,
}

impl Sphere {
    pub fn new(
        radius: f64,
        center: Vec3,
        color: Color,
        specularity: Specularity,
        reflectiveness: f64,
    ) -> Self {
        Sphere {
            radius,
            center,
            color,
            specularity,
            reflectiveness,
        }
    }
}

pub struct Surface {
    pub w: f64,
    pub h: f64,
}

impl Surface {
    pub fn new(w: f64, h: f64) -> Self {
        Surface { w, h }
    }
}

pub struct AmbientLight {
    pub intensity: f64,
}

impl AmbientLight {
    pub fn new(intensity: f64) -> Self {
        AmbientLight { intensity }
    }
}

pub struct PointLight {
    pub intensity: f64,
    pub position: Vec3,
}

impl PointLight {
    pub fn new(intensity: f64, position: Vec3) -> Self {
        PointLight {
            intensity,
            position,
        }
    }
}

pub struct DirectionalLight {
    pub intensity: f64,
    pub dir: Vec3,
}

impl DirectionalLight {
    pub fn new(intensity: f64, dir: Vec3) -> Self {
        DirectionalLight { intensity, dir }
    }
}

pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Directional(DirectionalLight),
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub bg_color: Color,
    pub canvas: Surface,
    pub viewport: Surface,
    pub camera_dist: f64,
    pub lights: Vec<Light>,
}

pub struct Camera {
    pub position: Vec3,
    pub y_rot: f64,
    pub x_rot: f64,
}
