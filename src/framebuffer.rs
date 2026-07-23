use raylib::prelude::*;
use raylib::texture::Image;

pub struct Framebuffer {
    width: u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, color);
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, filename: &str) {
        self.color_buffer.export_image(filename);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    /// Como swap_buffers, pero escala el framebuffer (resolución del grid,
    /// p.ej. 100x100) al tamaño real de la ventana (p.ej. 800x800).
    /// Útil para que cada "célula" ocupe varios píxeles en pantalla.
    pub fn swap_buffers_scaled(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        window_width: i32,
        window_height: i32,
        overlay_text: Option<&str>,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let source = Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32);
            let dest = Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32);
            let overlay_layout = overlay_text.map(|text| {
                let font_size = 20;
                let text_width = window.measure_text(text, font_size);
                let padding_x = 12;
                let padding_y = 8;
                let box_width = text_width + padding_x * 2;
                let box_height = font_size + padding_y * 2;
                let box_x = (window_width - box_width) / 2;
                let box_y = window_height - box_height - 10;
                let text_x = box_x + padding_x;
                let text_y = box_y + padding_y;

                (text_x, text_y, box_x, box_y, box_width, box_height, font_size, text)
            });
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture_pro(
                &texture,
                source,
                dest,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            if let Some((text_x, text_y, box_x, box_y, box_width, box_height, font_size, text)) =
                overlay_layout
            {
                // Pequeño HUD para que el estado visible no dependa de la decoración
                // de la ventana del sistema operativo.
                renderer.draw_rectangle(
                    box_x,
                    box_y,
                    box_width,
                    box_height,
                    Color::new(0, 0, 0, 180),
                );
                renderer.draw_text(text, text_x, text_y, font_size, Color::RAYWHITE);
            }
        }
    }
}
