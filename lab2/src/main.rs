mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;
use std::thread;
use std::time;

fn render (framebuffer: &mut Framebuffer, translate_x: f32, translate_y: f32) {
    
    line(framebuffer,
    Vector2::new(50.0 + translate_x, 50.0 + translate_y),
    Vector2::new(350.0 + translate_x, 350.0 + translate_y),
    );

    line(framebuffer,
    Vector2::new(50.0 + translate_x, 350.0 + translate_y),
    Vector2::new(350.0 + translate_x, 50.0 + translate_y),
    ); 
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let frame_buffer_width = window_width;
    let frame_buffer_height = window_height;

    let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Window Example")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

    let mut framebuffer = Framebuffer::new(frame_buffer_width, frame_buffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);
    
    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;
        
        framebuffer.clear();
        
        render(&mut framebuffer, translate_x, translate_y);
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(time::Duration::from_millis(16));
    }
}
