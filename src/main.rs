#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use raytracer::geometry::*;
use raytracer::raytracer::*;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const ORIGIN: Vec3 = Vec3::ZERO;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Raytracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let scene = Scene {
        spheres: vec![
            Sphere::new(
                1.0,
                Vec3::new(0.0, -1.0, 3.0),
                Color::new(0xb2, 0x0d, 0x30, 0xff),
                Material::Specular(500.0),
            ),
            Sphere::new(
                1.0,
                Vec3::new(2.0, 0.0, 4.0),
                Color::new(0x3f, 0x84, 0xe5, 0xff),
                Material::Specular(500.0),
            ),
            Sphere::new(
                1.0,
                Vec3::new(-2.0, 0.0, 4.0),
                Color::new(0x3f, 0x78, 0x4c, 0xff),
                Material::Specular(10.0),
            ),
            Sphere::new(
                5000.0,
                Vec3::new(0.0, -5001.0, 0.0),
                Color::new(0xc1, 0x78, 0x17, 0xff),
                Material::Specular(1000.0),
            ),
        ],
        bg_color: Color::WHITE,
        canvas: Surface::new(WIDTH as f64, HEIGHT as f64),
        viewport: Surface::new(1.0, HEIGHT as f64 / WIDTH as f64),
        camera_dist: 1.0,
        lights: vec![
            Light::Ambient(AmbientLight::new(0.2)),
            Light::Point(PointLight::new(0.6, Vec3::new(2.0, 1.0, 0.0))),
            Light::Directional(DirectionalLight::new(0.2, Vec3::new(1.0, 4.0, 4.0))),
        ],
    };

    scene.draw(pixels.frame_mut());

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // scene.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

trait Drawable {
    fn draw(&self, frame: &mut [u8]);
}

impl Drawable for Scene {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as f64;
            let y = (i / WIDTH as usize) as f64;
            let cx = x - (WIDTH / 2) as f64;
            let cy = (HEIGHT / 2) as f64 - y;

            let dir = canvas_to_viewport(self, cx, cy);
            let color = trace_ray(self, ORIGIN, dir, 1.0, f64::INFINITY);

            pixel.copy_from_slice(&color.as_u8_slice());
        }
    }
}
