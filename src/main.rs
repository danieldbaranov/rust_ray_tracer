#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod color;

use cgmath::Vector3;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::color::*;

// Representation of the application state. In this example, a box will bounce around the screen.
struct Renderer {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin:  Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut renderer = Renderer::new();

    let window = {
        let size = LogicalSize::new(renderer.image_width as f64, renderer.image_height as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(renderer.image_width, renderer.image_height, surface_texture)?
    };


    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.draw(pixels.frame_mut());
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
            if input.key_held(VirtualKeyCode::W){
                renderer.origin.y += 0.01;
            }
            if input.key_held(VirtualKeyCode::R){
                renderer.origin.y -= 0.01;
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
            renderer.update();
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

impl Renderer {
    /// Create a new `World` instance that can draw a moving box.
    /// Create a new `Renderer` instance.
    fn new() -> Self {
        // Image
        let aspect_ratio = 16. / 9.;
        let image_width = 400;
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        //Camera
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

        Renderer {
            aspect_ratio,
            image_width,
            image_height,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        /*
        if self.box_x <= 0 || self.box_x + BOX_SIZE > IMAGE_WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > IMAGE_HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
        */
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.image_width as usize) as i16;
            let y = 256 - (i / self.image_width as usize) as i16;

            let u = x as f32 / (self.image_width - 1) as f32;
            let v = y as f32 / (self.image_height - 1) as f32;

            let ray: Ray = Ray::new(self.origin,
                            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin);

            let pixel_color = ray_color(ray);

            pixel.copy_from_slice( &write_color(pixel_color));
        }
    }
}