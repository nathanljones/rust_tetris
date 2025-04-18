mod tetromino;

use macroquad::prelude::*;
mod constants {
    pub const BOARD_HEIGHT: u32 = 18;
    pub const BOARD_WIDTH: u32 = 12;
    pub const DRAW_SCALE: f32 = 30.0;
    pub const TETROMINO_SIZE: u32 = 4;
    pub const SPEED: f64 = 0.5;
    pub const SHOW_FILLED_LINES_TIME: f64 = 0.3;
    pub const SCORE_INCREMENT: u32 = 25;
    pub const SCORE_COMPLETED_LINES_INCREMENT: u32 = 100;
}
use constants::*;
const TETROMINO_I: &str = "..X...X...X...X.";
const TETROMINO_O: &str = ".....XX..XX.....";
const TETROMINO_T: &str = "..X..XX...X.....";
const TETROMINO_J: &str = "..X...X..XX.....";
const TETROMINO_L: &str = ".X...X...XX.....";
const TETROMINO_S: &str = ".X...XX...X.....";
const TETROMINO_Z: &str = "..X..XX..X......";

#[derive(Clone, Copy)]
enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

fn rotate(x: i32, y: i32, rotation: Rotation) -> usize {
    match rotation {
        Rotation::Zero => (y * 4 + x) as usize,
        Rotation::Ninety => (12 + y - (x * 4)) as usize,
        Rotation::OneEighty => (15 - (y * 4) - x) as usize,
        Rotation::TwoSeventy => (3 - y + (x * 4)) as usize,
    }
}
fn convert_xy_to_array_pos(x: i32, y: i32) -> usize {
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

fn can_piece_move(
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

fn add_boarders_to_board(
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

fn draw_score(score: u32) {
    let text = format!("Score: {score}");
    let font_size = 30.;
    draw_text(&text, 500.0, 50.0, font_size, WHITE);
}

fn draw_board(board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize]) {
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            if board[convert_xy_to_array_pos(x, y)] != ' ' {
                draw_rectangle(x as f32 * DRAW_SCALE, y as f32 * DRAW_SCALE, DRAW_SCALE, DRAW_SCALE, convert_tetromino_colour(board[convert_xy_to_array_pos(x, y)] as u32),);
            }
        }
    }
}

fn flash_filled_lines(
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

fn lock_tetromino_in_place(
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
fn draw_tetromino(
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
fn draw_game_over_message() {
    let text = "Game Over.";
    let font_size = 30.;
    draw_text(text, 500.0, 250.0, font_size, WHITE);
}
fn rotate_tetromino(
    board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
    current_tetromino: &str,
    rotation: Rotation,
    current_x: i32,
    current_y: i32,
) -> Rotation {
    let temp_rotation:Rotation;
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
fn check_for_filled_lines(board: &[char; (BOARD_HEIGHT * BOARD_WIDTH) as usize]) -> Vec<i32> {
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
#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut current_x: i32=5;
    let mut current_y: i32 = 0;
    let mut new_x: i32=5;
    let mut new_y: i32 = 0;
    let mut rotation = Rotation::Zero;
    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut force_down: bool = false;
    let mut current_tetromino: &str;
    let mut tetromino_number: usize;
    let mut filled_lines: Vec<i32> = Vec::new();
    let mut last_show_lines_update = get_time();
    let mut game_over = false;
    let mut score: u32 = 0;

    // represent the playing board as a single dimension array
    let mut board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] =
        [' '; (BOARD_HEIGHT * BOARD_WIDTH) as usize];
    let tetrominos: [&str; 7] = [
        TETROMINO_I,
        TETROMINO_J,
        TETROMINO_L,
        TETROMINO_S,
        TETROMINO_Z,
        TETROMINO_O,
        TETROMINO_T,
    ];
    rand::srand(miniquad::date::now() as _);
    // put the borders on the board
    board = add_boarders_to_board(&board);
    tetromino_number = rand::gen_range(0, 6);
    current_tetromino = tetrominos[tetromino_number];
    //current_x = 5;
    //new_x = current_x;
    loop {
        if !game_over {
            if is_key_down(KeyCode::Right) && !navigation_lock && can_piece_move(
                    current_tetromino,
                    current_x + 1,
                    current_y,
                    rotation,
                    &board,
                ) {
                new_x += 1;
                new_y = current_y;
                navigation_lock = true;
            }
            if is_key_down(KeyCode::Left) && !navigation_lock && can_piece_move(
                    current_tetromino,
                    current_x - 1,
                    current_y,
                    rotation,
                    &board,
                ) {
                new_x -= 1;
                new_y = current_y;
                navigation_lock = true;
            }

            if is_key_down(KeyCode::Up) && !navigation_lock {
                rotation =
                    rotate_tetromino(&board, current_tetromino, rotation, current_x, current_y);
            }

            if get_time() - last_update > SPEED {
                last_update = get_time();
                force_down = true;
                navigation_lock = false;
            }

            if get_time() - last_show_lines_update > SHOW_FILLED_LINES_TIME {
                last_show_lines_update = get_time();
                board = flash_filled_lines(&board, &filled_lines);
                filled_lines.clear();
            }

            if can_piece_move(current_tetromino, new_x, new_y, rotation, &board) {
                current_x = new_x;
                current_y = new_y;
            }

            if force_down {
                if can_piece_move(current_tetromino, current_x, current_y + 1, rotation, &board,) {
                    new_y += 1;
                } else {
                    board = lock_tetromino_in_place(&board, current_tetromino, rotation, current_x, current_y, tetromino_number,);
                    score += SCORE_INCREMENT;
                    // check for full lines
                    filled_lines = check_for_filled_lines(&board);
                    if !filled_lines.is_empty() {
                        for line in &filled_lines {
                            for x in 0..BOARD_WIDTH as i32 {
                                board[convert_xy_to_array_pos(x, *line)] = '8';
                            }
                        }
                        score += filled_lines.len() as u32 * SCORE_COMPLETED_LINES_INCREMENT;
                    }

                    current_x = 5;
                    current_y = 0;
                    new_x = 5;
                    new_y = 0;
                    tetromino_number = rand::gen_range(0, 6);
                    current_tetromino = tetrominos[tetromino_number];
                    rotation = Rotation::Zero;
                    if !can_piece_move(current_tetromino, current_x, current_y, rotation, &board) {
                        game_over = true;
                    }
                }
                force_down = false;
            }
        }
        // draw the screen
        draw_board(&board);
        draw_score(score);
        draw_tetromino(current_tetromino, rotation, current_x, current_y, tetromino_number,);

        if game_over {
            draw_game_over_message();
        }

        next_frame().await;
    }
}
