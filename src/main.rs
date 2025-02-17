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

#[derive(Clone, Copy)]
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
        8 => VIOLET,
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
    let mut current_tetronimo: &str;
    let mut tetromino_number: usize;
    let mut filled_lines: Vec<i32> = Vec::new();
    let mut show_filled_lines_time = 0.3;
    let mut last_show_lines_update = get_time();
    let mut game_over = false;

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
    rand::srand(macroquad::miniquad::date::now() as _);
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
    current_x = 5;
    new_x = 5;
    rotation = Rotation::Zero;
    loop {
        if !game_over {
            if is_key_down(KeyCode::Right) && !navigation_lock {
                if can_piece_move(
                    current_tetronimo,
                    current_x + 1,
                    current_y,
                    &rotation,
                    &board,
                ) == true
                {
                    new_x += 1;
                    new_y = current_y;
                    navigation_lock = true;
                }
            }
            if is_key_down(KeyCode::Left) && !navigation_lock {
                if can_piece_move(
                    current_tetronimo,
                    current_x - 1,
                    current_y,
                    &rotation,
                    &board,
                ) == true
                {
                    new_x -= 1;
                    new_y = current_y;
                    navigation_lock = true;
                }
            }

            if is_key_down(KeyCode::Up) && !navigation_lock {
                let mut temp_rotation = rotation.clone();
                match rotation {
                    Rotation::Zero => temp_rotation = Rotation::Ninety,
                    Rotation::Ninety => temp_rotation = Rotation::OneEighty,
                    Rotation::OneEighty => temp_rotation = Rotation::TwoSeventy,
                    Rotation::TwoSeventy => temp_rotation = Rotation::Zero,
                }
                if can_piece_move(
                    current_tetronimo,
                    current_x,
                    current_y,
                    &temp_rotation,
                    &board,
                ) == true
                {
                    rotation = temp_rotation;
                }
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                force_down = true;
                navigation_lock = false;
            }

            if get_time() - last_show_lines_update > show_filled_lines_time {
                last_show_lines_update = get_time();
                for line in filled_lines.iter() {
                    for y in (1..line + 1).rev() {
                        for x in (0..BOARD_WIDTH as i32) {
                            board[convert_xy_to_array_pos(x, y)] =
                                board[convert_xy_to_array_pos(x, y - 1)];
                        }
                    }
                }
                for x in 0..BOARD_WIDTH {
                    board[x as usize] = ' ';
                }
                filled_lines.clear();
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
                    // check for full lines
                    for y in 0..(BOARD_HEIGHT - 1) as i32 {
                        let mut has_a_gap: bool = false;
                        for x in 0..BOARD_WIDTH as i32 {
                            if board[convert_xy_to_array_pos(x, y)] == ' ' {
                                has_a_gap = true;
                            }
                        }
                        if has_a_gap != true {
                            filled_lines.push(y);
                        }
                    }
                    if !filled_lines.is_empty() {
                        for line in filled_lines.iter() {
                            for x in 0..BOARD_WIDTH as i32 {
                                board[convert_xy_to_array_pos(x, *line)] = '8';
                            }
                        }
                    }

                    current_x = 5;
                    current_y = 0;
                    new_x = 5;
                    new_y = 0;
                    tetromino_number = rand::gen_range(0, 6);
                    current_tetronimo = tetronimos[tetromino_number];
                    rotation = Rotation::Zero;
                    if !can_piece_move(current_tetronimo, current_x, current_y, &rotation, &board){
                       game_over = true;
                    }

                }
                force_down = false;
            }
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

        if game_over {
            let text = "Game Over.";
            let font_size = 30.;
            draw_text(text, 500.0, 250.0, font_size, WHITE);
        }

        next_frame().await
    }
}
