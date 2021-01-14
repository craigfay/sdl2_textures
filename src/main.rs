// https://sunjay.dev/learn-game-dev/smooth-movement.html
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use image;
use image::{ImageBuffer, Rgba};

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
}

struct Window {

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

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        let texture_creator = canvas.texture_creator();

        let img = image::open("rust.png").unwrap().into_rgba8();

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

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();

        canvas.copy_ex(
            &texture,
            None,
            Some(Rect::new(400, 100, 256, 256)), // x, y, width, height
            0.0, // Rotation
            None,
            false,
            false,
        ).unwrap();

        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut input = ControllerInput::new();

        'running: loop {

            for event in event_pump.poll_iter() {
                match event {

                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,


                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        input.L = 1;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        input.R = 1;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        input.U = 1;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        input.D = 1;
                    }

                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        input.L = 0;
                    }
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        input.R = 0;
                    }
                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                        input.U = 0;
                    }
                    Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                        input.D = 0;
                    }


                    _ => {}
                }

                println!("{:?}", input);
            }
        }

        Window {}
    }

}

pub fn main() {
    let win = Window::new();
}
