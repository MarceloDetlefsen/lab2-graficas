mod framebuffer;
mod life;
mod line;
mod logo;
mod patterns;

use std::collections::HashSet;

use framebuffer::Framebuffer;
use raylib::prelude::*;

/// Resolución lógica del juego (cada celda = 1 "pixel" del framebuffer).
const GRID_WIDTH: u32 = 120;
const GRID_HEIGHT: u32 = 120;

/// Resolución real de la ventana. GRID -> WINDOW se escala en swap_buffers_scaled.
const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 900;

fn build_initial_state() -> HashSet<(i32, i32)> {
    let mut live_cells: HashSet<(i32, i32)> = HashSet::new();

    // --- Logo del meteoro (silueta) al centro del grid ---
    let logo_w = 46;
    let logo_h = 46;
    let logo_cells = logo::load_logo_pattern(
        "assets/ff7_logo.png",
        logo_w,
        logo_h,
        140,
        true,
    );

    let logo_origin_x = (GRID_WIDTH as i32 - logo_w as i32) / 2;
    let logo_origin_y = (GRID_HEIGHT as i32 - logo_h as i32) / 2 - 6;

    for (dx, dy) in logo_cells {
        live_cells.insert((logo_origin_x + dx, logo_origin_y + dy));
    }

    // --- Patrones clásicos distribuidos con más espacio entre ellos y

    // Glider gun arriba a la izquierda, disparando hacia el centro.
    for (dx, dy) in patterns::gosper_glider_gun() {
        live_cells.insert((3 + dx, 3 + dy));
    }

    // Pulsar arriba a la derecha.
    for (dx, dy) in patterns::pulsar() {
        live_cells.insert((90 + dx, 4 + dy));
    }

    // Pentadecathlon abajo a la izquierda.
    for (dx, dy) in patterns::pentadecathlon() {
        live_cells.insert((6 + dx, 95 + dy));
    }

    // LWSS viajando por la orilla inferior.
    for (dx, dy) in patterns::lwss() {
        live_cells.insert((45 + dx, 110 + dy));
    }

    // MWSS abajo a la derecha.
    for (dx, dy) in patterns::mwss() {
        live_cells.insert((90 + dx, 108 + dy));
    }

    // HWSS a la derecha, altura media.
    for (dx, dy) in patterns::hwss() {
        live_cells.insert((100 + dx, 55 + dy));
    }

    // Glider suelto, arriba al centro, "entrando en cámara".
    for (dx, dy) in patterns::glider() {
        live_cells.insert((55 + dx, 3 + dy));
    }

    // Beacon y toad a los costados, altura media.
    for (dx, dy) in patterns::beacon() {
        live_cells.insert((105 + dx, 90 + dy));
    }
    for (dx, dy) in patterns::toad() {
        live_cells.insert((3 + dx, 60 + dy));
    }

    // Still lifes (loaf, boat, tub, block, beehive) como relleno,
    // repartidos en huecos libres alrededor del logo.
    for (dx, dy) in patterns::loaf() {
        live_cells.insert((55 + dx, 100 + dy));
    }
    for (dx, dy) in patterns::boat() {
        live_cells.insert((15 + dx, 20 + dy));
    }
    for (dx, dy) in patterns::tub() {
        live_cells.insert((100 + dx, 25 + dy));
    }
    for (dx, dy) in patterns::block() {
        live_cells.insert((3 + dx, 45 + dy));
    }
    for (dx, dy) in patterns::beehive() {
        live_cells.insert((3 + dx, 80 + dy));
    }

    // Blinker de relleno cerca del centro-derecha.
    for (dx, dy) in patterns::blinker() {
        live_cells.insert((95 + dx, 100 + dy));
    }

    live_cells
}

fn render(framebuffer: &mut Framebuffer, live_cells: &HashSet<(i32, i32)>) {
    framebuffer.clear();
    for &(x, y) in live_cells {
        framebuffer.set_pixel(x, y, Color::WHITE);
    }
}

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life - FF7 Meteor")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        Color::BLACK,
    );

    let live_cells = build_initial_state();
    render(&mut framebuffer, &live_cells);

    while !window.window_should_close() {
        framebuffer.swap_buffers_scaled(
            &mut window,
            &raylib_thread,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );
    }
}
