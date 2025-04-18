mod constants;
mod tetromino;
use macroquad::prelude::*;
use rust_tetris::{
    Rotation, add_boarders_to_board, can_piece_move, check_for_filled_lines,
    convert_xy_to_array_pos, draw_board, draw_game_over_message, draw_score, draw_tetromino,
    flash_filled_lines, lock_tetromino_in_place, rotate_tetromino,
};

use constants::{
    BOARD_HEIGHT, BOARD_WIDTH, SCORE_COMPLETED_LINES_INCREMENT, SCORE_INCREMENT,
    SHOW_FILLED_LINES_TIME, SPEED,
};
const TETROMINO_I: &str = "..X...X...X...X.";
const TETROMINO_O: &str = ".....XX..XX.....";
const TETROMINO_T: &str = "..X..XX...X.....";
const TETROMINO_J: &str = "..X...X..XX.....";
const TETROMINO_L: &str = ".X...X...XX.....";
const TETROMINO_S: &str = ".X...XX...X.....";
const TETROMINO_Z: &str = "..X..XX..X......";

#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut current_x: i32 = 5;
    let mut current_y: i32 = 0;
    let mut new_x: i32 = 5;
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
            if is_key_down(KeyCode::Right)
                && !navigation_lock
                && can_piece_move(
                    current_tetromino,
                    current_x + 1,
                    current_y,
                    rotation,
                    &board,
                )
            {
                new_x += 1;
                new_y = current_y;
                navigation_lock = true;
            }
            if is_key_down(KeyCode::Left)
                && !navigation_lock
                && can_piece_move(
                    current_tetromino,
                    current_x - 1,
                    current_y,
                    rotation,
                    &board,
                )
            {
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
                if can_piece_move(
                    current_tetromino,
                    current_x,
                    current_y + 1,
                    rotation,
                    &board,
                ) {
                    new_y += 1;
                } else {
                    board = lock_tetromino_in_place(
                        &board,
                        current_tetromino,
                        rotation,
                        current_x,
                        current_y,
                        tetromino_number,
                    );
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
        draw_tetromino(
            current_tetromino,
            rotation,
            current_x,
            current_y,
            tetromino_number,
        );

        if game_over {
            draw_game_over_message();
        }

        next_frame().await;
    }
}
