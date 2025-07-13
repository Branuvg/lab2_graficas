mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use std::thread;
use std::time;

// --- CONFIGURACIÓN ---
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

// --- ZOOM ---
// Mantenemos la resolución baja para que los píxeles se vean grandes.
const GAME_WIDTH: i32 = 100;
const GAME_HEIGHT: i32 = 75;

const ALIVE_COLOR: Color = Color::WHITE;
const DEAD_COLOR: Color = Color::BLACK;

// --- FUNCIONES PARA DIBUJAR ORGANISMOS ---

// -- Still Lifes --
fn draw_block(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 0, y + 0); fb.set_pixel(x + 1, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 1, y + 1);
}

fn draw_beehive(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0); fb.set_pixel(x + 2, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 3, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 2, y + 2);
}

fn draw_loaf(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0); fb.set_pixel(x + 2, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 3, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 3, y + 2);
    fb.set_pixel(x + 2, y + 3);
}

fn draw_boat(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 0, y + 0); fb.set_pixel(x + 1, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 1, y + 2);
}

fn draw_tub(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0);
    fb.set_pixel(x + 0, y + 1);
    fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 1, y + 2);
}

// -- Oscillators --
fn draw_blinker(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x, y + 1); fb.set_pixel(x, y + 2);
}

fn draw_toad(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0); fb.set_pixel(x + 2, y + 0); fb.set_pixel(x + 3, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 1, y + 1); fb.set_pixel(x + 2, y + 1);
}

fn draw_beacon(fb: &mut Framebuffer, x: i32, y: i32) {
    draw_block(fb, x, y);
    draw_block(fb, x + 2, y + 2);
}

// -- Spaceships --
fn draw_glider(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 0, y + 2); fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 2, y + 2);
}

fn draw_lwss(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y + 0); fb.set_pixel(x + 2, y + 0); fb.set_pixel(x + 3, y + 0); fb.set_pixel(x + 4, y + 0);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 4, y + 1);
    fb.set_pixel(x + 4, y + 2);
    fb.set_pixel(x + 0, y + 3); fb.set_pixel(x + 3, y + 3);
}

fn draw_mwss(fb: &mut Framebuffer, x: i32, y: i32) { // Middle-weight spaceship
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y); fb.set_pixel(x + 3, y); fb.set_pixel(x + 4, y); fb.set_pixel(x + 5, y);
    fb.set_pixel(x + 0, y + 1); fb.set_pixel(x + 5, y + 1);
    fb.set_pixel(x + 5, y + 2);
    fb.set_pixel(x + 0, y + 3); fb.set_pixel(x + 4, y + 3);
    fb.set_pixel(x + 2, y + 4);
}

fn draw_hwss(fb: &mut Framebuffer, x: i32, y: i32) { // Heavy-weight spaceship
    fb.set_pixel(x+1, y); fb.set_pixel(x+2, y); fb.set_pixel(x+3, y); fb.set_pixel(x+4, y); fb.set_pixel(x+5, y); fb.set_pixel(x+6, y);
    fb.set_pixel(x+0, y+1); fb.set_pixel(x+6, y+1);
    fb.set_pixel(x+6, y+2);
    fb.set_pixel(x+0, y+3); fb.set_pixel(x+5, y+3);
    fb.set_pixel(x+2, y+4); fb.set_pixel(x+3, y+4);
}

// -- Guns --
fn draw_gosper_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    let gun_points = [
        (24, 0), (22, 1), (24, 1), (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3), (0, 4), (1, 4), (10, 4),
        (16, 4), (20, 4), (21, 4), (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5),
        (22, 5), (24, 5), (10, 6), (16, 6), (24, 6), (11, 7), (15, 7), (12, 8), (13, 8)
    ];
    for (px, py) in gun_points { fb.set_pixel(x + px, y + py); }
}

fn draw_flipped_gosper_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    let gun_points = [
        (24, 0), (22, 1), (24, 1), (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3), (0, 4), (1, 4), (10, 4),
        (16, 4), (20, 4), (21, 4), (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5),
        (22, 5), (24, 5), (10, 6), (16, 6), (24, 6), (11, 7), (15, 7), (12, 8), (13, 8)
    ];
    let max_x = 35; // The width of the pattern
    for (px, py) in gun_points {
        fb.set_pixel(x + (max_x - px), y + py);
    }
}


/// Dibuja el patrón inicial en el tablero.
fn setup_initial_pattern(fb: &mut Framebuffer) {
    fb.set_current_color(ALIVE_COLOR);

    // --- Cañones Enfrentados (Confronted Cannons) ---
    draw_gosper_glider_gun(fb, 1, 1);
    draw_flipped_gosper_glider_gun(fb, GAME_WIDTH - 38, 1);

    // --- Flotas Masivas (Massive Fleets) ---
    for i in 0..10 {
        if 5 + i * 10 < GAME_WIDTH - 10 {
            draw_lwss(fb, 2 + i * 10, 50);
            draw_mwss(fb, 4 + i * 10, 60);
            draw_hwss(fb, 6 + i * 10, 40);
        }
    }

    // --- Llenado de Fondo con Patrones Variados (Background Fill) ---
    for y in (0..GAME_HEIGHT).step_by(7) {
        for x in (0..GAME_WIDTH).step_by(7) {
            // Chequeo para no sobreescribir los patrones grandes
            if fb.get_pixel_color(x, y).unwrap() != DEAD_COLOR {
                continue;
            }

            let pattern_type = (x + y * 3) % 9;

            if x < GAME_WIDTH - 8 && y < GAME_HEIGHT - 8 {
                match pattern_type {
                    0 => draw_block(fb, x, y),
                    1 => draw_beehive(fb, x, y),
                    2 => draw_blinker(fb, x + 2, y),
                    3 => draw_boat(fb, x, y),
                    4 => draw_tub(fb, x, y),
                    5 => draw_loaf(fb, x, y),
                    6 => draw_toad(fb, x, y),
                    7 => draw_beacon(fb, x, y),
                    _ => { /* Dejar algunos espacios vacíos para la dinámica */ }
                }
            }
        }
    }
}

/// Ejecuta un paso de la simulación del Juego de la Vida.
fn run_game_of_life_step(fb: &mut Framebuffer) {
    let mut next_gen_buffer = fb.color_buffer.clone();
    for y in 0..fb.height {
        for x in 0..fb.width {
            let mut live_neighbors = 0;
            for j in -1..=1 {
                for i in -1..=1 {
                    if i == 0 && j == 0 { continue; }
                    let neighbor_x = (x + i + fb.width) % fb.width;
                    let neighbor_y = (y + j + fb.height) % fb.height;
                    if let Some(color) = fb.get_pixel_color(neighbor_x, neighbor_y) {
                        if color == ALIVE_COLOR { live_neighbors += 1; }
                    }
                }
            }
            let current_cell_color = fb.get_pixel_color(x, y).unwrap();
            let is_alive = current_cell_color == ALIVE_COLOR;
            if is_alive && (live_neighbors < 2 || live_neighbors > 3) {
                next_gen_buffer.draw_pixel(x, y, DEAD_COLOR);
            } else if !is_alive && live_neighbors == 3 {
                next_gen_buffer.draw_pixel(x, y, ALIVE_COLOR);
            } else {
                next_gen_buffer.draw_pixel(x, y, current_cell_color);
            }
        }
    }
    fb.color_buffer = next_gen_buffer;
}

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Game of Life de Conway - Densidad Máxima")
        .build();

    let mut framebuffer = Framebuffer::new(GAME_WIDTH, GAME_HEIGHT, DEAD_COLOR);
    setup_initial_pattern(&mut framebuffer);

    while !window.window_should_close() {
        run_game_of_life_step(&mut framebuffer);
        let texture = window.load_texture_from_image(&raylib_thread, &framebuffer.color_buffer).unwrap();
        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::DARKGRAY);
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, GAME_WIDTH as f32, GAME_HEIGHT as f32),
            Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        d.draw_fps(10, 10);
        thread::sleep(time::Duration::from_millis(50));
    }
}