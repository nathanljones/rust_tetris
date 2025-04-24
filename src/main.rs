use macroquad::prelude::*;
use rust_tetris::{Direction, draw_board, draw_score, draw_tetromino, spawn_tetromino};

use rust_tetris::board::Board;
use rust_tetris::constants::{SCORE_COMPLETED_LINES_INCREMENT, SCORE_INCREMENT, SPEED};
use rust_tetris::tetromino::Tetromino;

#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut board = Board::new();
    let mut score: u32 = 0;
    let mut current_tetromino: Tetromino;
    let mut last_update = get_time();
    let mut force_down: bool = false;
    let mut navigation_lock: bool = false;

    current_tetromino = spawn_tetromino();

    loop {
        if is_key_down(KeyCode::Left) && !navigation_lock {
            if board.can_piece_move(current_tetromino, Direction::Left) {
                current_tetromino.move_left();
            }
            navigation_lock = true;
        }
        if is_key_down(KeyCode::Right) && !navigation_lock {
            if board.can_piece_move(current_tetromino, Direction::Right) {
                current_tetromino.move_right();
            }
            navigation_lock = true;
        }
        if is_key_pressed(KeyCode::Up) {
            current_tetromino.rotate();
            navigation_lock = true;
        }

        if get_time() - last_update > SPEED {
            last_update = get_time();
            force_down = true;
            navigation_lock = false;
            board.remove_filled_lines();
        }

        if force_down {
            if board.can_piece_move(current_tetromino, Direction::Down) {
                current_tetromino.move_down();
                force_down = false;
            } else {
                force_down = false;
                board.lock_tetromino_in_place(current_tetromino);
                if board.get_filled_lines().len() == 4 {
                    score += SCORE_COMPLETED_LINES_INCREMENT;
                } else if !board.get_filled_lines().is_empty() {
                    score += SCORE_INCREMENT;
                }
                board.colour_in_filled_lines();
                current_tetromino = spawn_tetromino();
            }
        }
        draw_board(&board);
        draw_tetromino(&mut current_tetromino);
        draw_score(score);
        next_frame().await;
    }
}

// keep below for reference
/*
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
*/
