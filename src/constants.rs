// Height & width of the game board
pub const BOARD_HEIGHT: u32 = 18;
pub const BOARD_WIDTH: u32 = 12;
pub const DRAW_SCALE: f32 = 30.0; // Scales the rectangles from 1:1 to 1:30
pub const TETROMINO_SIZE: u32 = 4; // Tetrominos are squares so the size is the width & height
pub const SPEED: f64 = 0.5; // speed at which the game runs. 
//Needed as processors run at different speeds
pub const SHOW_FILLED_LINES_TIME: f64 = 0.3; // How long to show the filled lines for.
// Processors run at different speeds so this
// makes sure it's the same for everyone
pub const SCORE_INCREMENT: u32 = 25; // Amount to increase score each time a block lands 
pub const SCORE_COMPLETED_LINES_INCREMENT: u32 = 100; // Amount to increase score by when
// a full line is achieved
