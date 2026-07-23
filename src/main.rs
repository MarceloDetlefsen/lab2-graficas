mod framebuffer;
mod life;
mod line;
mod logo;
mod patterns;

use framebuffer::Framebuffer;
use life::Grid;
use raylib::prelude::*;
use std::time::{Duration, Instant};

/// Resolución lógica del juego (cada celda = 1 "pixel" del framebuffer).
const GRID_WIDTH: u32 = 120;
const GRID_HEIGHT: u32 = 120;

/// Resolución real de la ventana. GRID -> WINDOW se escala en swap_buffers_scaled.
const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 900;

const BACKGROUND_COLOR: Color = Color::BLACK;
const TICK_INTERVAL_MS: u64 = 120;

fn place_pattern(grid: &mut Grid, origin_x: i32, origin_y: i32, cells: Vec<(i32, i32)>) {
    for (dx, dy) in cells {
        let x = origin_x + dx;
        let y = origin_y + dy;

        if let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) {
            grid.set_alive(x, y, true);
        }
    }
}

fn place_pattern_colored(
    grid: &mut Grid,
    origin_x: i32,
    origin_y: i32,
    cells: Vec<(i32, i32, Color)>,
) {
    for (dx, dy, color) in cells {
        let x = origin_x + dx;
        let y = origin_y + dy;

        if let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) {
            grid.set_alive_color(x, y, Some(color));
        }
    }
}

fn build_initial_state() -> Grid {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    // --- Logo del meteoro (silueta) al centro del grid ---
    let logo_w = 46;
    let logo_h = 46;
    let logo_cells = logo::load_logo_pattern_colored(
        "assets/ff7_logo.png",
        logo_w,
        logo_h,
        140,
        true,
    );

    let logo_origin_x = (GRID_WIDTH as i32 - logo_w as i32) / 2;
    let logo_origin_y = (GRID_HEIGHT as i32 - logo_h as i32) / 2 - 6;

    place_pattern_colored(&mut grid, logo_origin_x, logo_origin_y, logo_cells);

    // --- Patrones clásicos distribuidos con más espacio entre ellos y

    // Glider gun arriba a la izquierda, disparando hacia el centro.
    place_pattern(&mut grid, 3, 3, patterns::gosper_glider_gun());

    // Pulsar arriba a la derecha.
    place_pattern(&mut grid, 90, 4, patterns::pulsar());

    // Pentadecathlon abajo a la izquierda.
    place_pattern(&mut grid, 6, 95, patterns::pentadecathlon());

    // LWSS viajando por la orilla inferior.
    place_pattern(&mut grid, 45, 110, patterns::lwss());

    // MWSS abajo a la derecha.
    place_pattern(&mut grid, 90, 108, patterns::mwss());

    // HWSS a la derecha, altura media.
    place_pattern(&mut grid, 100, 55, patterns::hwss());

    // Glider suelto, arriba al centro, "entrando en cámara".
    place_pattern(&mut grid, 55, 3, patterns::glider());

    // Beacon y toad a los costados, altura media.
    place_pattern(&mut grid, 105, 90, patterns::beacon());
    place_pattern(&mut grid, 3, 60, patterns::toad());

    // Still lifes (loaf, boat, tub, block, beehive) como relleno,
    // repartidos en huecos libres alrededor del logo.
    place_pattern(&mut grid, 55, 100, patterns::loaf());
    place_pattern(&mut grid, 15, 20, patterns::boat());
    place_pattern(&mut grid, 100, 25, patterns::tub());
    place_pattern(&mut grid, 3, 45, patterns::block());
    place_pattern(&mut grid, 3, 80, patterns::beehive());

    // Blinker de relleno cerca del centro-derecha.
    place_pattern(&mut grid, 95, 100, patterns::blinker());

    grid
}

fn render(framebuffer: &mut Framebuffer, grid: &Grid) {
    framebuffer.clear();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let color = grid.get_color(x, y).unwrap_or(BACKGROUND_COLOR);
            framebuffer.set_pixel(x as i32, y as i32, color);
        }
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

    let mut grid = build_initial_state();
    let mut paused = false;
    let mut frame_count: u64 = 0;
    let mut generation: u64 = 0;
    let tick_interval = Duration::from_millis(TICK_INTERVAL_MS);
    let mut last_frame_time = Instant::now();
    let mut step_accumulator = Duration::ZERO;

    while !window.window_should_close() {
        frame_count += 1;

        let now = Instant::now();
        let delta = now - last_frame_time;
        last_frame_time = now;

        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        if window.is_key_pressed(KeyboardKey::KEY_R) {
            grid = build_initial_state();
            generation = 0;
            step_accumulator = Duration::ZERO;
        }

        if paused {
            if window.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                grid = grid.step();
                generation += 1;
            }
        } else {
            step_accumulator += delta;

            while step_accumulator >= tick_interval {
                grid = grid.step();
                generation += 1;
                step_accumulator -= tick_interval;
            }
        }

        let title = format!("Conway's Game of Life - Gen: {generation}");
        window.set_window_title(&raylib_thread, &title);

        render(&mut framebuffer, &grid);
        framebuffer.swap_buffers_scaled(
            &mut window,
            &raylib_thread,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            Some(&title),
        );
    }
}
