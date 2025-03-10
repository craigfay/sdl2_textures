// https://sunjay.dev/learn-game-dev/smooth-movement.html
extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::WindowCanvas;

use image;
use image::{ImageBuffer, Rgba, RgbaImage};

#[derive(Debug, Copy, Clone)]
pub struct ControllerInput {
    U: u8,
    D: u8,
    L: u8,
    R: u8,
}

impl ControllerInput {
    fn new() -> ControllerInput {
        ControllerInput {
            U: 0,
            D: 0,
            L: 0,
            R: 0,
        }
    }
}

trait GameDevice {
    fn get_controller_input(&self) -> ControllerInput;
    fn render(&self);
}

struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
    input: ControllerInput,
    is_available: bool,
}

impl Window {
    pub fn new() -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let input = ControllerInput::new();

        Window {
            canvas,
            event_pump,
            input,
            is_available: true,
        }
    }

    pub fn get_controller_input(&mut self) -> ControllerInput {
        for event in self.event_pump.poll_iter() {
            match event {

                Event::Quit { .. } => {
                    self.is_available = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_available = false;
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.input.L = 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.input.R = 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.input.U = 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.input.D = 1;
                }

                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    self.input.L = 0;
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    self.input.R = 0;
                }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    self.input.U = 0;
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    self.input.D = 0;
                }
                _ => {}
            }
        }

        self.input.clone()
    }

    pub fn render(&mut self, img: RgbaImage) {
        let texture_creator = self.canvas.texture_creator();

        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA32,
                img.width(),
                img.height(),
            )
            .map_err(|e| e.to_string()).unwrap();

        // A blend mode needs to be set in order for alpha channels
        // to take effect.
        texture.set_blend_mode(BlendMode::Blend);

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // pitch is the number of bytes in a row of pixel data,
            // including padding between lines
            for y in 0..img.height() {
                for x in 0..img.width() {

                    let offset: usize = y as usize * pitch + x as usize * 4;
                    let pixel = img.get_pixel(x as u32, y as u32);

                    buffer[offset] = pixel[0];
                    buffer[offset + 1] = pixel[1];
                    buffer[offset + 2] = pixel[2];
                    buffer[offset + 3] = pixel[3];
                }
            }
        }).unwrap();

        self.canvas.clear();
        self.canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();

        self.canvas.copy_ex(
            &texture,
            None,
            Some(Rect::new(400, 100, 256, 256)), // x, y, width, height
            0.0, // Rotation
            None,
            false,
            false,
        ).unwrap();

        self.canvas.present();
    }
}

pub fn main() {
    let mut win = Window::new();

    let img = image::open("rust.png").unwrap().into_rgba8();

    win.render(img);

    while win.is_available {
        let input = win.get_controller_input();
    }
}
