extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

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

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, 256, 256)
        .map_err(|e| e.to_string())?;

    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        // pitch is the number of bytes in a row of pixel data,
        // including padding between lines

        for y in 0..256 {
            for x in 0..256 {
                let offset = y * pitch + x * 4;
                buffer[offset] = x as u8; // Red
                buffer[offset + 1] = 0; // Gree
                buffer[offset + 2] = 100 as u8; // Blue
                buffer[offset + 3] = 200; // ?
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;

    canvas.copy_ex(
        &texture,
        None, // 
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