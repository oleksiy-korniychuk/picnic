// --- Game Constants ---
pub const GRID_WIDTH: usize = 25;
pub const GRID_HEIGHT: usize = 25;
pub const TILE_SIZE: f32 = 32.0;
pub const TICK_RATE_HZ: f64 = 2.0;

// --- Window/Camera Constants ---
pub const DEFAULT_WINDOW_WIDTH: f32 = 1200.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;
pub const DEFAULT_ZOOM: f32 = 1.0;
pub const MIN_ZOOM: f32 = 0.1;
pub const ZOOM_SPEED: f32 = 0.1;
pub const CAMERA_PAN_SPEED: f32 = 400.0;

// --- World Generation Constants ---
pub const WATER_LEVEL: f32 = 0.3; // Tiles below this are lakes
pub const SCALE: f64 = 0.02;      // Controls how zoomed in/out the noise is
    