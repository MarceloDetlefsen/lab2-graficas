/// Estado de un tablero de Conway's Game of Life.
///
/// El tablero se guarda en un buffer lineal `Vec<bool>` en orden por filas:
/// índice = `y * width + x`.
pub struct Grid {
    pub cells: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    /// Crea un tablero nuevo, inicialmente todo muerto.
    pub fn new(width: u32, height: u32) -> Self {
        let size = width
            .checked_mul(height)
            .expect("grid demasiado grande");

        Self {
            cells: vec![false; size as usize],
            width,
            height,
        }
    }

    /// Marca una celda como viva o muerta.
    pub fn set_alive(&mut self, x: u32, y: u32, alive: bool) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = alive;
        }
    }

    /// Consulta si una celda está viva.
    pub fn is_alive(&self, x: u32, y: u32) -> bool {
        self.index(x, y)
            .map(|index| self.cells[index])
            .unwrap_or(false)
    }

    /// Cuenta vecinos vivos usando wraparound toroidal.
    pub fn count_neighbors(&self, x: u32, y: u32) -> u8 {
        if self.width == 0 || self.height == 0 {
            return 0;
        }

        let x = x % self.width;
        let y = y % self.height;
        let mut count = 0;

        for dy in [-1i32, 0, 1] {
            for dx in [-1i32, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = Self::wrap_coord(x, dx, self.width);
                let ny = Self::wrap_coord(y, dy, self.height);

                if self.is_alive(nx, ny) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Calcula la siguiente generación de Conway sin mutar el tablero actual.
    pub fn step(&self) -> Grid {
        let mut next = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let neighbors = self.count_neighbors(x, y);

                let next_alive = match (alive, neighbors) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set_alive(x, y, next_alive);
            }
        }

        next
    }

    fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y
            .checked_mul(self.width)?
            .checked_add(x)?;

        Some(index as usize)
    }

    fn wrap_coord(coord: u32, delta: i32, size: u32) -> u32 {
        let size_i32 = size as i32;
        let coord_i32 = coord as i32;
        let wrapped = (coord_i32 + delta).rem_euclid(size_i32);
        wrapped as u32
    }
}
