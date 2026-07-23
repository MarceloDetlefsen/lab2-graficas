use std::collections::HashSet;

use image::GenericImageView;
use raylib::prelude::Color;

pub fn load_logo_pattern(
    path: &str,
    target_w: u32,
    target_h: u32,
    threshold: u8,
    invert: bool,
) -> Vec<(i32, i32)> {
    load_logo_pattern_colored(path, target_w, target_h, threshold, invert, 1.0)
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect()
}

/// Igual que `load_logo_pattern`, pero conserva el color real de cada celda
/// viva para poder dibujar el logo con su degradado original en el frame
/// inicial.
///
/// `density` controla qué porcentaje de las celdas candidatas se conserva
/// como vivas. `1.0` mantiene todas, `0.4` conserva aproximadamente el 40%.
pub fn load_logo_pattern_colored(
    path: &str,
    target_w: u32,
    target_h: u32,
    threshold: u8,
    invert: bool,
    density: f32,
) -> Vec<(i32, i32, Color)> {
    let img = image::open(path).expect("no se pudo abrir la imagen del logo");
    let density = if density.is_finite() {
        density.clamp(0.0, 1.0)
    } else {
        1.0
    };

    // Triangle suaviza el downscale sin introducir tanto ringing como
    // filtros más agresivos, y queda mejor para limpiar la silueta.
    let resized = img.resize_exact(target_w, target_h, image::imageops::FilterType::Triangle);

    let has_transparency =
        (0..target_h).any(|y| (0..target_w).any(|x| resized.get_pixel(x, y).0[3] < 255));

    let mut cells = Vec::new();

    for y in 0..target_h {
        for x in 0..target_w {
            let pixel = resized.get_pixel(x, y);
            let [r, g, b, a] = pixel.0;

            let is_alive = if has_transparency {
                a > threshold
            } else {
                let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
                luminance < threshold as f32
            };

            let is_alive = if invert { is_alive } else { !is_alive };

            if is_alive && rand::random::<f32>() < density {
                cells.push((x as i32, y as i32, Color::new(r, g, b, a)));
            }
        }
    }

    // Limpieza mínima para quitar píxeles sueltos que aparecen por el resize.
    limpiar_celdas_aisladas(cells)
}

fn limpiar_celdas_aisladas(cells: Vec<(i32, i32, Color)>) -> Vec<(i32, i32, Color)> {
    let occupied: HashSet<(i32, i32)> = cells.iter().map(|(x, y, _)| (*x, *y)).collect();

    cells
        .into_iter()
        .filter(|(x, y, _)| {
            for ny in (y - 1)..=(y + 1) {
                for nx in (x - 1)..=(x + 1) {
                    if nx == *x && ny == *y {
                        continue;
                    }

                    if occupied.contains(&(nx, ny)) {
                        return true;
                    }
                }
            }

            false
        })
        .collect()
}
