use macroquad::color::{
    Color, DARKBLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, VIOLET, WHITE, YELLOW,
};
use macroquad::prelude::{draw_rectangle, draw_text};
mod constants;
use constants::{BOARD_HEIGHT, BOARD_WIDTH, DRAW_SCALE, TETROMINO_SIZE};
#[derive(Clone, Copy)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

//Hold X & Y values as a U32
pub struct UCoordinate {
    pub x: u32,
    pub y: u32,
}
impl UCoordinate {
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
pub fn rotate(x: i32, y: i32, rotation: Rotation) -> usize {
    match rotation {
        Rotation::Zero => (y * 4 + x) as usize,
        Rotation::Ninety => (12 + y - (x * 4)) as usize,
        Rotation::OneEighty => (15 - (y * 4) - x) as usize,
        Rotation::TwoSeventy => (3 - y + (x * 4)) as usize,
    }
}
pub fn convert_xy_to_array_pos(x: i32, y: i32) -> usize {
    (x + y * BOARD_WIDTH as i32) as usize
}

fn convert_tetromino_colour(tetromino_number: u32) -> Color {
    match tetromino_number {
        0 => SKYBLUE,
        1 => DARKBLUE,
        2 => ORANGE,
        3 => GREEN,
        4 => RED,
        5 => YELLOW,
        6 => PURPLE,
        8 => VIOLET,
        _ => WHITE,
    }
}

pub fn can_piece_move(
    current_tetromino: &str,
    current_x: i32,
    current_y: i32,
    current_rotation: Rotation,
    board: &[char],
) -> bool {
    for y in 0..TETROMINO_SIZE as i32 {
        for x in 0..TETROMINO_SIZE as i32 {
            if current_tetromino
                .chars()
                .nth(rotate(x, y, current_rotation))
                .unwrap()
                == 'X'
            {
                if x + current_x >= 0 && x + current_x < BOARD_WIDTH as i32 {
                    if y + current_y >= 0 && y + current_y < BOARD_HEIGHT as i32 {
                        if board[convert_xy_to_array_pos(x + current_x, y + current_y)] != ' ' {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

pub fn add_boarders_to_board(
    board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
) -> [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] {
    let mut ret_board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] = *board;
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            if (x == 0 || x == BOARD_WIDTH as i32 - 1) || y == BOARD_HEIGHT as i32 - 1 {
                ret_board[convert_xy_to_array_pos(x, y)] = '9';
            }
        }
    }
    ret_board
}

pub fn draw_score(score: u32) {
    let text = format!("Score: {score}");
    let font_size = 30.;
    draw_text(&text, 500.0, 50.0, font_size, WHITE);
}

pub fn draw_board(board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize]) {
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            if board[convert_xy_to_array_pos(x, y)] != ' ' {
                draw_rectangle(
                    x as f32 * DRAW_SCALE,
                    y as f32 * DRAW_SCALE,
                    DRAW_SCALE,
                    DRAW_SCALE,
                    convert_tetromino_colour(board[convert_xy_to_array_pos(x, y)] as u32),
                );
            }
        }
    }
}

pub fn flash_filled_lines(
    board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    filled_lines: &Vec<i32>,
) -> [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] {
    let mut ret_board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] = *board;
    for line in filled_lines {
        for y in (1..line + 1).rev() {
            for x in 0..BOARD_WIDTH as i32 {
                ret_board[convert_xy_to_array_pos(x, y)] = board[convert_xy_to_array_pos(x, y - 1)];
            }
        }
    }
    for x in 1..BOARD_WIDTH - 1 {
        ret_board[x as usize] = ' ';
    }
    ret_board
}

pub fn lock_tetromino_in_place(
    board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    current_tetromino: &str,
    rotation: Rotation,
    current_x: i32,
    current_y: i32,
    tetromino_number: usize,
) -> [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] {
    let mut ret_board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] = *board;
    for y in 0..TETROMINO_SIZE as i32 {
        for x in 0..TETROMINO_SIZE as i32 {
            if current_tetromino
                .chars()
                .nth(rotate(x, y, rotation))
                .unwrap()
                == 'X'
            {
                ret_board[convert_xy_to_array_pos(current_x + x, current_y + y)] =
                    char::from_u32(tetromino_number as u32).unwrap();
            }
        }
    }
    ret_board
}
pub fn draw_tetromino(
    current_tetromino: &str,
    rotation: Rotation,
    current_x: i32,
    current_y: i32,
    tetromino_number: usize,
) {
    for y in 0..TETROMINO_SIZE as i32 {
        for x in 0..TETROMINO_SIZE as i32 {
            if current_tetromino
                .chars()
                .nth(rotate(x, y, rotation))
                .unwrap()
                == 'X'
            {
                draw_rectangle(
                    (x + current_x) as f32 * DRAW_SCALE,
                    (y + current_y) as f32 * DRAW_SCALE,
                    DRAW_SCALE,
                    DRAW_SCALE,
                    convert_tetromino_colour(tetromino_number as u32),
                );
            }
        }
    }
}
pub fn draw_game_over_message() {
    let text = "Game Over.";
    let font_size = 30.;
    draw_text(text, 500.0, 250.0, font_size, WHITE);
}
pub fn rotate_tetromino(
    board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    current_tetromino: &str,
    rotation: Rotation,
    current_x: i32,
    current_y: i32,
) -> Rotation {
    let temp_rotation: Rotation;
    match rotation {
        Rotation::Zero => temp_rotation = Rotation::Ninety,
        Rotation::Ninety => temp_rotation = Rotation::OneEighty,
        Rotation::OneEighty => temp_rotation = Rotation::TwoSeventy,
        Rotation::TwoSeventy => temp_rotation = Rotation::Zero,
    }
    if can_piece_move(
        current_tetromino,
        current_x,
        current_y,
        temp_rotation,
        board,
    ) {
        temp_rotation
    } else {
        rotation
    }
}
pub fn check_for_filled_lines(board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize]) -> Vec<i32> {
    let mut ret_filled_lines: Vec<i32> = Vec::new();
    for y in 0..(BOARD_HEIGHT - 1) as i32 {
        let mut has_a_gap: bool = false;
        for x in 0..BOARD_WIDTH as i32 {
            if board[convert_xy_to_array_pos(x, y)] == ' ' {
                has_a_gap = true;
            }
        }
        if !has_a_gap {
            ret_filled_lines.push(y);
        }
    }
    ret_filled_lines
}
