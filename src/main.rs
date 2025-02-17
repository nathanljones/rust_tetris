use macroquad::prelude::*;

const BOARD_HEIGHT: u32 = 18;
const BOARD_WIDTH: u32 = 12;
const DRAW_SCALE: f32 = 30.0;
const TETRONIMO_SIZE: u32 = 4;

const TETROMINO_I: &str = "..X...X...X...X.";
const TETROMINO_O: &str = ".....XX..XX.....";
const TETROMINO_T: &str = "..X..XX...X.....";
const TETROMINO_J: &str = "..X...X..XX.....";
const TETROMINO_L: &str = ".X...X...XX.....";
const TETROMINO_S: &str = ".X...XX...X.....";
const TETROMINO_Z: &str = "..X..XX..X......";

enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}
fn rotate(x: i32, y: i32, rotation: &Rotation) -> usize {
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
        _ => WHITE,
    }
}

fn can_piece_move(
    current_tetronimo: &str,
    current_x: i32,
    current_y: i32,
    current_rotation: &Rotation,
    board: &[char],
) -> bool {
    for y in 0..TETRONIMO_SIZE as i32 {
        for x in 0..TETRONIMO_SIZE as i32 {
            if current_tetronimo
                .chars()
                .nth(rotate(x, y, &current_rotation))
                .unwrap()
                == 'X'
            {
                if x as i32 + current_x >= 0 && x as i32 + current_x < BOARD_WIDTH as i32 {
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

#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut new_x: i32 = 0;
    let mut new_y: i32 = 0;
    let mut rotation = Rotation::Zero;
    let mut speed = 0.5;
    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut force_down: bool = false;
    let mut temp_hide: bool = false;
    let mut current_tetronimo: &str;
    let mut tetromino_number: usize;

    // represent the playing board as a single dimension array
    let mut board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize] =
        [' '; (BOARD_HEIGHT * BOARD_WIDTH) as usize];
    let tetronimos: [&str; 7] = [
        TETROMINO_I,
        TETROMINO_J,
        TETROMINO_L,
        TETROMINO_S,
        TETROMINO_Z,
        TETROMINO_O,
        TETROMINO_T,
    ];

    // put the borders on the board
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            if (x == 0 || x == BOARD_WIDTH as i32 - 1) || y == BOARD_HEIGHT as i32 - 1 {
                board[convert_xy_to_array_pos(x, y)] = '9';
            }
        }
    }
    tetromino_number = rand::gen_range(0, 6);
    current_tetronimo = tetronimos[tetromino_number];
    loop {
        if is_key_down(KeyCode::Right) && !navigation_lock {
            let temp_x: i32 = current_x as i32 + 1;
            if can_piece_move(current_tetronimo, temp_x, current_y, &rotation, &board) == true {
                new_x += 1;
                navigation_lock = true;
            }
        }
        if is_key_down(KeyCode::Left) && !navigation_lock {
            let temp_x: i32 = current_x as i32 - 1;
            if can_piece_move(current_tetronimo, temp_x, current_y, &rotation, &board) == true {
                new_x -= 1;
                navigation_lock = true;
            }
        }

        if get_time() - last_update > speed {
            last_update = get_time();
            force_down = true;
        }

        if can_piece_move(current_tetronimo, new_x, new_y, &rotation, &board) == true {
            current_x = new_x;
            current_y = new_y;
        }

        if force_down == true {
            if can_piece_move(
                current_tetronimo,
                current_x,
                current_y + 1,
                &rotation,
                &board,
            ) == true
            {
                new_y += 1;
            } else {
                for y in 0..TETRONIMO_SIZE as i32 {
                    for x in 0..TETRONIMO_SIZE as i32 {
                        if current_tetronimo
                            .chars()
                            .nth(rotate(x, y, &rotation))
                            .unwrap()
                            == 'X'
                        {
                            board[convert_xy_to_array_pos(current_x + x, current_y + y)] =
                                char::from_u32(tetromino_number as u32).unwrap();
                        }
                    }
                }
                current_x = 0;
                current_y = 0;
                new_x = 0;
                new_y = 0;
                tetromino_number = rand::gen_range(0, 6);
                current_tetronimo = tetronimos[tetromino_number];
            }
            force_down = false;
        }

        // draw the screen
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

        for y in 0..TETRONIMO_SIZE as i32 {
            for x in 0..TETRONIMO_SIZE as i32 {
                if current_tetronimo
                    .chars()
                    .nth(rotate(x, y, &rotation))
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

        next_frame().await
    }
}
