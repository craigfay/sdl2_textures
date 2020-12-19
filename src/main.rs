extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use image;
use image::{ImageBuffer, Rgba};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let img = image::open("rust.png").unwrap().into_rgba8();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            img.width(),
            img.height(),
        )
        .map_err(|e| e.to_string())?;


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
    })?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;

    canvas.copy_ex(
        &texture,
        None,
        Some(Rect::new(400, 100, 256, 256)), // x, y, width, height
        0.0, // Rotation
        None,
        false,
        false,
    )?;

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    Ok(())
}