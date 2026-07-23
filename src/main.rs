mod framebuffer;
mod line;
mod logo;
mod patterns;

use std::collections::{HashMap, HashSet};

use framebuffer::Framebuffer;
use raylib::prelude::*;

/// Resolución lógica del juego (cada celda = 1 "pixel" del framebuffer).
/// Se mantiene baja a propósito para que el escalado a la ventana se note.
const GRID_WIDTH: u32 = 160;
const GRID_HEIGHT: u32 = 160;

/// Resolución real de la ventana. GRID -> WINDOW se escala en swap_buffers_scaled.
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn build_initial_state() -> (HashSet<(i32, i32)>, HashMap<(i32, i32), Color>) {
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut logo_colors: HashMap<(i32, i32), Color> = HashMap::new();

    // --- Logo del meteoro (silueta) al centro del grid ---
    let logo_w = 70;
    let logo_h = 70;
    let logo_cells = logo::load_logo_pattern_colored(
        "assets/ff7_logo.png",
        logo_w,
        logo_h,
        40,
        true,
    );

    let logo_origin_x = (GRID_WIDTH as i32 - logo_w as i32) / 2;
    let logo_origin_y = (GRID_HEIGHT as i32 - logo_h as i32) / 2 - 10;

    for (dx, dy, color) in logo_cells {
        live_cells.insert((logo_origin_x + dx, logo_origin_y + dy));
        logo_colors.insert((logo_origin_x + dx, logo_origin_y + dy), color);
    }

    // --- Patrones clásicos alrededor, para que se note la mecánica real ---

    // Glider gun arriba a la izquierda, disparando hacia el centro.
    for (dx, dy) in patterns::gosper_glider_gun() {
        live_cells.insert((5 + dx, 5 + dy));
    }

    // Pulsar arriba a la derecha.
    for (dx, dy) in patterns::pulsar() {
        live_cells.insert((120 + dx, 10 + dy));
    }

    // Pentadecathlon abajo a la izquierda.
    for (dx, dy) in patterns::pentadecathlon() {
        live_cells.insert((15 + dx, 130 + dy));
    }

    // LWSS viajando por la orilla inferior.
    for (dx, dy) in patterns::lwss() {
        live_cells.insert((60 + dx, 145 + dy));
    }

    // MWSS abajo a la derecha.
    for (dx, dy) in patterns::mwss() {
        live_cells.insert((120 + dx, 140 + dy));
    }

    // Beacon y toad como relleno cerca del centro-derecha.
    for (dx, dy) in patterns::beacon() {
        live_cells.insert((140 + dx, 70 + dy));
    }
    for (dx, dy) in patterns::toad() {
        live_cells.insert((10 + dx, 70 + dy));
    }

    // Blinker y block/beehive como detalle extra.
    for (dx, dy) in patterns::blinker() {
        live_cells.insert((80 + dx, 5 + dy));
    }
    for (dx, dy) in patterns::block() {
        live_cells.insert((5 + dx, 100 + dy));
    }
    for (dx, dy) in patterns::beehive() {
        live_cells.insert((150 + dx, 100 + dy));
    }

    (live_cells, logo_colors)
}

fn build_logo_palette(logo_colors: &HashMap<(i32, i32), Color>) -> Vec<Color> {
    let mut palette = Vec::new();

    for &color in logo_colors.values() {
        if !palette.contains(&color) {
            palette.push(color);
        }
    }

    palette.sort_by_key(|color| {
        (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32) as u32
    });

    palette
}

fn color_for_cell(
    x: i32,
    y: i32,
    logo_colors: &HashMap<(i32, i32), Color>,
    logo_palette: &[Color],
) -> Color {
    if let Some(color) = logo_colors.get(&(x, y)) {
        return *color;
    }

    if logo_palette.is_empty() {
        return Color::WHITE;
    }

    let index =
        ((x.wrapping_mul(31) ^ y.wrapping_mul(17)).unsigned_abs() as usize) % logo_palette.len();
    logo_palette[index]
}

fn render(
    framebuffer: &mut Framebuffer,
    live_cells: &HashSet<(i32, i32)>,
    logo_colors: &HashMap<(i32, i32), Color>,
) {
    let logo_palette = build_logo_palette(logo_colors);

    framebuffer.clear();
    for &(x, y) in live_cells {
        let color = color_for_cell(x, y, logo_colors, &logo_palette);
        framebuffer.set_pixel(x, y, color);
    }
}

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life - FF7 Meteor")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT, Color::BLACK);

    let (live_cells, logo_colors) = build_initial_state();
    render(&mut framebuffer, &live_cells, &logo_colors);

    while !window.window_should_close() {
        framebuffer.swap_buffers_scaled(&mut window, &raylib_thread, WINDOW_WIDTH, WINDOW_HEIGHT);
    }
}
