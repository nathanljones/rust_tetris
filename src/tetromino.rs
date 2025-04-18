
use crate::Rotation;
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
}
impl Tetromino {
    fn new(shape_name: TetrominoShape) -> Self {
        Self {
            shape_name,
            rotation: Rotation::Zero,
        }
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
    fn get_rotated_tetromino(&mut self) -> String {
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
                        .nth(self.rotate_square(x, y))
                        .unwrap(),
                );
            }
        }
        output
    }
    fn rotate_square(&self, x: u32, y: u32) -> usize {
        // simple maths to transpose a given X/Y co-ordinate to it's rotated value
        match self.rotation {
            Rotation::Zero => (y * 4 + x) as usize,
            Rotation::Ninety => (12 + y - (x * 4)) as usize,
            Rotation::OneEighty => (15 - (y * 4) - x) as usize,
            Rotation::TwoSeventy => (3 - y + (x * 4)) as usize,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_rotated_tetromino() {
        let mut tetromino = Tetromino::new(TetrominoShape::I);
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("..X...X...X...X.")
        );
        tetromino.rotate();
        assert_eq!(
            tetromino.get_rotated_tetromino(),
            String::from("........XXXX....")
        );
    }
}
