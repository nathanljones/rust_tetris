use macroquad::prelude::*;
use rust_tetris::{
    Direction, draw_board, draw_game_over_message, draw_score, draw_tetromino, spawn_tetromino,
};

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
    let mut game_over: bool = false;

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
        if is_key_down(KeyCode::Down) && !navigation_lock {
            if board.can_piece_move(current_tetromino, Direction::Down) {
                current_tetromino.move_down();
            }
            navigation_lock = true;
        }

        if is_key_down(KeyCode::Space) && game_over {
            score = 0;
            board = Board::new();
            current_tetromino = spawn_tetromino();
            game_over = false;
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
                if !game_over {
                    current_tetromino = spawn_tetromino();
                    if !board.can_piece_move(current_tetromino, Direction::Down) {
                        game_over = true;
                    }
                }
            }
        }
        draw_board(&board);
        draw_tetromino(&mut current_tetromino);
        draw_score(score);
        if game_over {
            draw_game_over_message();
        }
        next_frame().await;
    }
}
