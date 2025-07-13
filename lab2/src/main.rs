mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use std::thread;
use std::time;

// --- CONFIGURACIÓN ---
const WINDOW_WIDTH: i32 = 1200;  // Ventana más grande
const WINDOW_HEIGHT: i32 = 900;
const GAME_WIDTH: i32 = 300;     // Mundo más pequeño para mejor visualización
const GAME_HEIGHT: i32 = 225;
const ALIVE_COLOR: Color = Color::WHITE;
const DEAD_COLOR: Color = Color::BLACK;
const ZOOM_FACTOR: f32 = 4.0;   // Factor de zoom para píxeles grandes

// --- FUNCIONES PARA DIBUJAR ORGANISMOS ---

// Still Lifes
fn draw_block(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x + 1, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 1, y + 1);
}

fn draw_beehive(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 3, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 2, y + 2);
}

fn draw_loaf(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 3, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 3, y + 2);
    fb.set_pixel(x + 2, y + 3);
}

fn draw_boat(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x + 1, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 1, y + 2);
}

fn draw_ship(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x + 1, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 2, y + 2);
}

fn draw_tub(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x + 1, y + 2);
}

// Oscillators
fn draw_blinker_h(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y);
}

fn draw_blinker_v(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x, y); fb.set_pixel(x, y + 1); fb.set_pixel(x, y + 2);
}

fn draw_toad(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y); fb.set_pixel(x + 3, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 1, y + 1); fb.set_pixel(x + 2, y + 1);
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

// Spaceships
fn draw_glider(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y + 1);
    fb.set_pixel(x, y + 2); fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 2, y + 2);
}

fn draw_lwss(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y); fb.set_pixel(x + 3, y); fb.set_pixel(x + 4, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 4, y + 1);
    fb.set_pixel(x + 4, y + 2);
    fb.set_pixel(x, y + 3); fb.set_pixel(x + 3, y + 3);
}

fn draw_mwss(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y); fb.set_pixel(x + 3, y); fb.set_pixel(x + 4, y); fb.set_pixel(x + 5, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 5, y + 1);
    fb.set_pixel(x + 5, y + 2);
    fb.set_pixel(x, y + 3); fb.set_pixel(x + 4, y + 3);
    fb.set_pixel(x + 2, y + 4);
}

fn draw_hwss(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x+1, y); fb.set_pixel(x+2, y); fb.set_pixel(x+3, y); fb.set_pixel(x+4, y); fb.set_pixel(x+5, y); fb.set_pixel(x+6, y);
    fb.set_pixel(x, y+1); fb.set_pixel(x+6, y+1);
    fb.set_pixel(x+6, y+2);
    fb.set_pixel(x, y+3); fb.set_pixel(x+5, y+3);
    fb.set_pixel(x+2, y+4); fb.set_pixel(x+3, y+4);
}

// Naves espaciales avanzadas
fn draw_copperhead(fb: &mut Framebuffer, x: i32, y: i32) {
    let points = [
        (1,0), (2,0), (4,0), (5,0),
        (0,1), (6,1),
        (0,2), (6,2),
        (1,3), (2,3), (4,3), (5,3),
        (3,4),
        (2,5), (4,5),
        (1,6), (5,6),
        (0,7), (6,7),
        (0,8), (1,8), (5,8), (6,8)
    ];
    for (px, py) in points { fb.set_pixel(x + px, y + py); }
}

fn draw_weekender(fb: &mut Framebuffer, x: i32, y: i32) {
    let points = [
        (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0),
        (1,1), (8,1),
        (0,2), (8,2),
        (0,3), (7,3),
        (0,4), (6,4),
        (7,5), (8,5),
        (5,6), (6,6), (8,6),
        (6,7)
    ];
    for (px, py) in points { fb.set_pixel(x + px, y + py); }
}

// Guns
fn draw_gosper_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    let gun_points = [
        (24, 0), (22, 1), (24, 1), (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3), (0, 4), (1, 4), (10, 4),
        (16, 4), (20, 4), (21, 4), (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5),
        (22, 5), (24, 5), (10, 6), (16, 6), (24, 6), (11, 7), (15, 7), (12, 8), (13, 8)
    ];
    for (px, py) in gun_points { fb.set_pixel(x + px, y + py); }
}

fn draw_simkin_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    let gun_points = [
        (0,0), (1,0), (0,1), (1,1),
        (10,0), (11,0), (12,0), (9,1), (13,1), (8,2), (14,2),
        (8,3), (14,3), (11,4),
        (8,5), (9,5), (13,5), (14,5),
        (10,6), (11,6), (12,6),
        (5,9), (6,9), (5,10), (6,10),
        (3,15), (4,15), (3,16), (4,16)
    ];
    for (px, py) in gun_points { fb.set_pixel(x + px, y + py); }
}

// Patrones de crecimiento
fn draw_r_pentomino(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y); fb.set_pixel(x + 2, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 1, y + 1);
    fb.set_pixel(x + 1, y + 2);
}

fn draw_diehard(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 6, y);
    fb.set_pixel(x, y + 1); fb.set_pixel(x + 1, y + 1);
    fb.set_pixel(x + 1, y + 2); fb.set_pixel(x + 5, y + 2); 
    fb.set_pixel(x + 6, y + 2); fb.set_pixel(x + 7, y + 2);
}

fn draw_acorn(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_pixel(x + 1, y);
    fb.set_pixel(x + 3, y + 1);
    fb.set_pixel(x, y + 2); fb.set_pixel(x + 1, y + 2); 
    fb.set_pixel(x + 4, y + 2); fb.set_pixel(x + 5, y + 2); 
    fb.set_pixel(x + 6, y + 2);
}

// Simple RNG sin dependencias externas
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005) + 1;
        (self.state >> 32) as u32
    }

    fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if min == max {
            min
        } else {
            let range = (max - min) as u32;
            min + (self.next() % range) as usize
        }
    }

    fn gen_bool(&mut self, probability: f32) -> bool {
        let max = 10000;
        let threshold = (probability * max as f32) as u32;
        (self.next() % max) < threshold
    }
}

/// Configuración inicial con múltiples patrones móviles
fn setup_dynamic_pattern(fb: &mut Framebuffer) {
    fb.set_current_color(ALIVE_COLOR);
    let mut rng = SimpleRng::new();

    // Lista de patrones con sus dimensiones (ahora con más patrones móviles)
    let patterns: Vec<(&dyn Fn(&mut Framebuffer, i32, i32), i32, i32)> = vec![
        // Still lifes
        (&draw_block, 2, 2),
        (&draw_beehive, 4, 3),
        (&draw_loaf, 4, 4),
        (&draw_boat, 3, 3),
        (&draw_ship, 3, 3),
        (&draw_tub, 3, 3),
        
        // Oscillators
        (&draw_blinker_h, 3, 1),
        (&draw_blinker_v, 1, 3),
        (&draw_toad, 4, 2),
        (&draw_beacon, 4, 4),
        (&draw_pulsar, 13, 13),
        (&draw_pentadecathlon, 10, 3),
        
        // Spaceships
        (&draw_glider, 3, 3),
        (&draw_lwss, 5, 4),
        (&draw_mwss, 6, 5),
        (&draw_hwss, 7, 5),
        (&draw_copperhead, 7, 9),
        (&draw_weekender, 9, 8),
        
        // Patrones de crecimiento
        (&draw_r_pentomino, 3, 3),
        (&draw_diehard, 8, 3),
        (&draw_acorn, 7, 3),
    ];

    // Colocar patrones en posiciones aleatorias (más patrones)
    for _ in 0..200 {
        let idx = rng.gen_range(0, patterns.len());
        let (pattern, width, height) = patterns[idx];
        let x = rng.gen_range(0, (GAME_WIDTH - width) as usize) as i32;
        let y = rng.gen_range(0, (GAME_HEIGHT - height) as usize) as i32;
        pattern(fb, x, y);
    }

    // Colocar armas de gliders (más probabilidad)
    if rng.gen_bool(0.5) {
        draw_gosper_glider_gun(fb, 10, 10);
    }
    if rng.gen_bool(0.5) {
        draw_simkin_glider_gun(fb, GAME_WIDTH - 40, 10);
    }
    if rng.gen_bool(0.4) {
        draw_gosper_glider_gun(fb, 10, GAME_HEIGHT - 40);
    }
    if rng.gen_bool(0.4) {
        draw_simkin_glider_gun(fb, GAME_WIDTH - 40, GAME_HEIGHT - 40);
    }

    // Agregar células aleatorias para más dinamismo
    for _ in 0..(GAME_WIDTH * GAME_HEIGHT / 10) {
        let x = rng.gen_range(0, GAME_WIDTH as usize) as i32;
        let y = rng.gen_range(0, GAME_HEIGHT as usize) as i32;
        fb.set_pixel(x, y);
    }
}

/// Ejecuta un paso de la simulación
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
        .title("Game of Life - Mega Ecosistema Dinámico")
        .build();

    let mut framebuffer = Framebuffer::new(GAME_WIDTH, GAME_HEIGHT, DEAD_COLOR);
    setup_dynamic_pattern(&mut framebuffer);

    while !window.window_should_close() {
        run_game_of_life_step(&mut framebuffer);
        
        // Crear textura con zoom
        let mut zoomed_image = Image::gen_image_color(
            (GAME_WIDTH as f32 * ZOOM_FACTOR) as i32,
            (GAME_HEIGHT as f32 * ZOOM_FACTOR) as i32,
            Color::BLACK
        );
        
        // Aplicar zoom a la imagen
        for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                let color = framebuffer.get_pixel_color(x, y).unwrap();
                for dy in 0..ZOOM_FACTOR as i32 {
                    for dx in 0..ZOOM_FACTOR as i32 {
                        zoomed_image.draw_pixel(
                            (x as f32 * ZOOM_FACTOR) as i32 + dx,
                            (y as f32 * ZOOM_FACTOR) as i32 + dy,
                            color
                        );
                    }
                }
            }
        }
        
        let texture = window.load_texture_from_image(&raylib_thread, &zoomed_image).unwrap();
        
        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::DARKGRAY);
        
        // Dibujar textura con zoom
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, zoomed_image.width as f32, zoomed_image.height as f32),
            Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        
        d.draw_text("Ecosistema Dinámico con Zoom", 10, 10, 20, Color::GREEN);
        d.draw_fps(10, 40);
        
        thread::sleep(time::Duration::from_millis(30));
    }
}