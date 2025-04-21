use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TETROMINO_SIZE};
use crate::tetromino::Tetromino;
use rust_tetris::{Direction, UCoordinate};

struct Board {
    board: [char; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
}
impl Board {
    fn new() -> Self {
        Self {
            board: [' '; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
        }
    }

    fn add_boarders_to_board(&mut self) {
        // the pieces on the board are represented as numbers
        // that way they can be coloured in later
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if (x == 0 || x == BOARD_WIDTH - 1) || y == BOARD_HEIGHT - 1 {
                    self.board[self.convert_xy_to_array_position(&UCoordinate::new(x, y))] = '9';
                }
            }
        }
    }
    fn convert_xy_to_array_position(&self, coordinate: &UCoordinate) -> usize {
        // this allows us to take an X,Y and flatten it out onto the board 1D array
        (coordinate.x + coordinate.y * BOARD_WIDTH) as usize
    }

    fn can_piece_move(
        &self,
        mut tetromino: Tetromino,
        current_coordinate: &UCoordinate,
        direction: Direction,
    ) -> bool {
        for y in 0..TETROMINO_SIZE {
            for x in 0..TETROMINO_SIZE {
                if tetromino
                    .get_rotated_tetromino()
                    .chars()
                    .nth(tetromino.rotate_square(UCoordinate::new(x, y)))
                    .unwrap()
                    == 'X'
                {
                    match direction {
                        Direction::Left => {}
                        Direction::Right => {}
                        Direction::Down => {}
                    }
                    if x + current_coordinate.x < BOARD_WIDTH
                        && y + current_coordinate.y < BOARD_HEIGHT
                    {
                        if self.board[self.convert_xy_to_array_position(&UCoordinate::new(
                            x + current_coordinate.x,
                            y + current_coordinate.y,
                        ))] != ' '
                        {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
    pub fn get_for_filled_lines(&self) -> Vec<u32> {
        let mut ret_filled_lines: Vec<u32> = Vec::new();
        for y in 0..BOARD_HEIGHT - 1 {
            let mut has_a_gap: bool = false;
            for x in 0..BOARD_WIDTH {
                if self.board[self.convert_xy_to_array_position(&UCoordinate::new(x, y))] == ' ' {
                    has_a_gap = true;
                }
            }
            if !has_a_gap {
                ret_filled_lines.push(y);
            }
        }
        ret_filled_lines
    }
    pub fn lock_tetromino_in_place(
        &mut self,
        mut tetromino:Tetromino,
        current_coordinate: &UCoordinate,
    ) {
        for y in 0..TETROMINO_SIZE {
            for x in 0..TETROMINO_SIZE{
                if tetromino.get_rotated_tetromino()
                    .chars()
                    .nth(tetromino.rotate_square(UCoordinate::new(x, y)))
                    .unwrap()
                    == 'X'
                {
                    self.board[self.convert_xy_to_array_position(&UCoordinate::new(current_coordinate.x+ x, current_coordinate.y + y))] =
                        char::from_u32(tetromino.get_colour()).unwrap();
                }
            }
        }

    }

}
