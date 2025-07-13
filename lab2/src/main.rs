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

fn draw_pulsar(fb: &mut Framebuffer, x: i32, y: i32) {
    for i in 0..4 {
        let (sin, cos) = (i as f32 * std::f32::consts::PI / 2.0).sin_cos();
        let (sin, cos) = (sin.round() as i32, cos.round() as i32);
        let transform = |px: i32, py: i32| -> (i32, i32) { (x + 6 + px * cos - py * sin, y + 6 + px * sin + py * cos) };
        let points = [(0, 2), (0, 3), (0, 4), (5, 2), (5, 3), (5, 4), (2, 0), (3, 0), (4, 0), (2, 5), (3, 5), (4, 5)];
        for (px, py) in points {
            let (tx, ty) = transform(px, py); fb.set_pixel(tx, ty);
            let (tx, ty) = transform(-px, py); fb.set_pixel(tx, ty);
        }
    }
}

fn draw_pentadecathlon(fb: &mut Framebuffer, x: i32, y: i32) {
    let points = [(0, 1), (1, 1), (2, 0), (2, 2), (3, 1), (4, 1), (5, 1), (6, 1), (7, 0), (7, 2), (8, 1), (9, 1)];
    for (px, py) in points { fb.set_pixel(x + px, y + py); }
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


/// Dibuja el patrón inicial en el tablero.
fn setup_initial_pattern(fb: &mut Framebuffer) {
    fb.set_current_color(ALIVE_COLOR);

    // --- Cañón de Gliders ---
    // Un gran cañón en la esquina superior izquierda.
    draw_gosper_glider_gun(fb, 2, 5);

    // --- La Gran Flota (The Great Fleet) ---
    // Una enorme cantidad de naves espaciales de varios tamaños moviéndose por la pantalla.
    for i in 0..5 {
        draw_lwss(fb, 5 + i * 18, 65);
        draw_mwss(fb, 8 + i * 18, 55);
        draw_hwss(fb, 11 + i * 18, 45);
    }
    for i in 0..10 {
        draw_glider(fb, 2 + i * 8, 35);
    }

    // --- Campo de Osciladores ---
    // Un área dedicada a patrones que parpadean y cambian.
    draw_pulsar(fb, 65, 5);
    draw_pulsar(fb, 65, 25);

    for i in 0..6 {
        draw_beacon(fb, 45 + i * 8, 5);
        draw_toad(fb, 45 + i * 8, 15);
    }
    for i in 0..4 {
        draw_pentadecathlon(fb, 40, 25 + i * 5);
    }

    // --- Lluvia de Blinkers ---
    // Llenar los espacios vacíos restantes con el oscilador más simple.
    for y in (0..GAME_HEIGHT).step_by(5) {
        for x in (0..GAME_WIDTH).step_by(3) {
            // Una forma simple de añadir aleatoriedad y llenar espacios.
            if (x + y * 2) % 13 == 0 {
                // Verificar que no dibujamos sobre algo importante (chequeo muy simple)
                if fb.get_pixel_color(x, y).unwrap() == DEAD_COLOR {
                    draw_blinker(fb, x, y);
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
        .title("Game of Life de Conway - Ecosistema en Movimiento")
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