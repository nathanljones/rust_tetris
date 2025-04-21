use crate::Rotation;
use rust_tetris::UCoordinate;
const TETROMINO_I: &str = "..X...X...X...X.";
const TETROMINO_O: &str = ".....XX..XX.....";
const TETROMINO_T: &str = "..X..XX...X.....";
const TETROMINO_J: &str = "..X...X..XX.....";
const TETROMINO_L: &str = ".X...X...XX.....";
const TETROMINO_S: &str = ".X...XX...X.....";
const TETROMINO_Z: &str = "..X..XX..X......";
const TETROMINO_SIZE: u32 = 4;
pub enum TetrominoShape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl TetrominoShape {
    fn shape(&self) -> String {
        // for a given shape name return the actual shape
        match self {
            TetrominoShape::I => String::from(TETROMINO_I),
            TetrominoShape::O => String::from(TETROMINO_O),
            TetrominoShape::T => String::from(TETROMINO_T),
            TetrominoShape::J => String::from(TETROMINO_J),
            TetrominoShape::L => String::from(TETROMINO_L),
            TetrominoShape::S => String::from(TETROMINO_S),
            TetrominoShape::Z => String::from(TETROMINO_Z),
        }
    }
}
pub struct Tetromino {
    shape_name: TetrominoShape,
    rotation: Rotation,
    colour:u32, // the board is represented as numbers which then gets converted to a colour
                // so this is just the colour number and will be converted later
}
impl Tetromino {
    fn new(shape_name: TetrominoShape, colour:u32) -> Self {
        Self {
            shape_name,
            rotation: Rotation::Zero,
            colour,
        }
    }
    pub fn get_colour(&self) -> u32 {
        self.colour
    }
    fn rotate(&mut self) {
        // move onto the next rotation. In this setup we always move clockwise
        match self.rotation {
            Rotation::Zero => self.rotation = Rotation::Ninety,
            Rotation::Ninety => self.rotation = Rotation::OneEighty,
            Rotation::OneEighty => self.rotation = Rotation::TwoSeventy,
            Rotation::TwoSeventy => self.rotation = Rotation::Zero,
        }
    }
    pub fn get_rotated_tetromino(&mut self) -> String {
        // We don't hold the rotated shape as part of this structure, we rotate on the fly.
        // We also hold the shape flattened rather than a 2D vector/array. This means we need to
        // loop over it to convert to 2D, rotate and convert back to a flattened shape
        let mut output: String = String::new();
        for y in 0..TETROMINO_SIZE {
            for x in 0..TETROMINO_SIZE {
                output.push(
                    self.shape_name
                        .shape()
                        .chars()
                        .nth(self.rotate_square(UCoordinate::new(x, y)))
                        .unwrap(),
                );
            }
        }
        output
    }
    pub fn rotate_square(&self, coordinate: UCoordinate) -> usize {
        // simple maths to transpose a given X/Y co-ordinate to it's rotated value
        match self.rotation {
            Rotation::Zero => (coordinate.y * 4 + coordinate.x) as usize,
            Rotation::Ninety => (12 + coordinate.y - (coordinate.x * 4)) as usize,
            Rotation::OneEighty => (15 - (coordinate.y * 4) - coordinate.x) as usize,
            Rotation::TwoSeventy => (3 - coordinate.y + (coordinate.x * 4)) as usize,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_rotated_tetromino() {
        let mut tetromino = Tetromino::new(TetrominoShape::I,1);
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("..X...X...X...X.")
        );
        tetromino.rotate();
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("........XXXX....")
        );
        tetromino.rotate();
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from(".X...X...X...X..")
        );

        tetromino.rotate();
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("....XXXX........")
        );
        tetromino.rotate();
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("..X...X...X...X.")
        );
    }
    #[test]
    fn test_get_tetromino_shape() {
        let mut tetromino = Tetromino::new(TetrominoShape::I,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_I);
        let mut tetromino = Tetromino::new(TetrominoShape::O,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_O);
        let mut tetromino = Tetromino::new(TetrominoShape::T,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_T);
        let mut tetromino = Tetromino::new(TetrominoShape::J,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_J);
        let mut tetromino = Tetromino::new(TetrominoShape::L,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_L);
        let mut tetromino = Tetromino::new(TetrominoShape::S,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_S);
        let mut tetromino = Tetromino::new(TetrominoShape::Z,1);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_Z);
    }
}
