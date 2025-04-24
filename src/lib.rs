use macroquad::color::{
    Color, DARKBLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, VIOLET, WHITE, YELLOW,
};
use macroquad::miniquad;
use macroquad::prelude::{draw_rectangle, draw_text, rand};
pub mod board;
pub mod constants;
pub mod coordinate;
pub mod tetromino;

use crate::tetromino::TetrominoShape;
use board::Board;
use constants::{BOARD_HEIGHT, BOARD_WIDTH, DRAW_SCALE, TETROMINO_SIZE};
use coordinate::UCoordinate;
use tetromino::Tetromino;

#[derive(Clone, Copy)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

pub enum Direction {
    Left,
    Right,
    Down,
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

pub fn draw_score(score: u32) {
    let text = format!("Score: {score}");
    let font_size = 30.;
    draw_text(&text, 500.0, 50.0, font_size, WHITE);
}

pub fn draw_board(board: &Board) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if board.get_board_character_at_coordinate(&UCoordinate::new(x, y)) != ' ' {
                //debug!("Colour is {}",char::to_digit(board.get_board_character_at_coordinate(&UCoordinate::new(x, y)),10).unwrap());
                draw_rectangle(
                    x as f32 * DRAW_SCALE,
                    y as f32 * DRAW_SCALE,
                    DRAW_SCALE,
                    DRAW_SCALE,
                    convert_tetromino_colour(
                        char::to_digit(
                            board.get_board_character_at_coordinate(&UCoordinate::new(x, y)),
                            10,
                        )
                        .unwrap(),
                    ),
                );
            }
        }
    }
}
pub fn draw_tetromino(tetromino: &mut Tetromino) {
    for y in 0..TETROMINO_SIZE {
        for x in 0..TETROMINO_SIZE {
            if tetromino.get_val_at_xy(&UCoordinate::new(x, y)) == 'X' {
                draw_rectangle(
                    (x as i32 + tetromino.get_coordinates().x) as f32 * DRAW_SCALE,
                    (y as i32 + tetromino.get_coordinates().y) as f32 * DRAW_SCALE,
                    DRAW_SCALE,
                    DRAW_SCALE,
                    convert_tetromino_colour(tetromino.get_colour()),
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
pub fn initialise_tetrominos() -> [Tetromino; 7] {
    let ret_tetrominos: [Tetromino; 7] = [
        Tetromino::new(TetrominoShape::I),
        Tetromino::new(TetrominoShape::J),
        Tetromino::new(TetrominoShape::L),
        Tetromino::new(TetrominoShape::S),
        Tetromino::new(TetrominoShape::Z),
        Tetromino::new(TetrominoShape::O),
        Tetromino::new(TetrominoShape::T),
    ];
    ret_tetrominos
}
pub fn spawn_tetromino() -> Tetromino {
    let mut tetromino: Tetromino;
    rand::srand(miniquad::date::now() as _);
    let tetromino_number:usize = rand::gen_range(0, 6);
    tetromino = initialise_tetrominos()[tetromino_number];
    tetromino.set_colour(tetromino_number as u32);
    tetromino
}