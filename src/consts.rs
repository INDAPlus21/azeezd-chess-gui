use ggez::{graphics};

/// A chess board is 8x8 tiles.
pub const GRID_SIZE: i16 = 8;

/// Sutible size of each tile.
pub const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32 + 150.0,
);

// GUI Color representations

/// Very Dark Grey
pub const BLACK: graphics::Color = graphics::Color::new(30.0/255.0, 30.0/255.0, 30.0/255.0, 1.0);

/// Less Darker than `BLACK`
pub const WHITE: graphics::Color = graphics::Color::new(70.0/255.0, 70.0/255.0, 70.0/255.0, 1.0);