use crate::Rotation;
use crate::constants::{TETROMINO_START_X, TETROMINO_START_Y};
use crate::coordinate::{ICoordinate, UCoordinate};
const TETROMINO_I: &str = "..X...X...X...X.";
const TETROMINO_O: &str = ".....XX..XX.....";
const TETROMINO_T: &str = "..X..XX...X.....";
const TETROMINO_J: &str = "..X...X..XX.....";
const TETROMINO_L: &str = ".X...X...XX.....";
const TETROMINO_S: &str = ".X...XX...X.....";
const TETROMINO_Z: &str = "..X..XX..X......";
const TETROMINO_SIZE: u32 = 4;
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Tetromino {
    shape_name: TetrominoShape,
    rotation: Rotation,
    colour: u32, // the board is represented as numbers which then gets converted to a colour
    // so this is just the colour number and will be converted later
    coordinates: ICoordinate,
}
impl Tetromino {
    pub fn new(shape_name: TetrominoShape) -> Self {
        Self {
            shape_name,
            rotation: Rotation::Zero,
            colour: 0,
            coordinates: ICoordinate {
                x: TETROMINO_START_X,
                y: TETROMINO_START_Y,
            },
        }
    }
    pub fn get_colour(&self) -> u32 {
        self.colour
    }
    pub fn set_colour(&mut self, colour: u32) {
        self.colour = colour;
    }
    pub fn get_coordinates(&self) -> ICoordinate {
        self.coordinates
    }
    pub fn move_left(&mut self) {
        self.coordinates.x -= 1;
    }
    pub fn move_right(&mut self) {
        self.coordinates.x += 1;
    }
    pub fn move_down(&mut self) {
        self.coordinates.y += 1;
    }
    pub fn rotate(&mut self) {
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
                        .nth(self.rotate_square(&UCoordinate::new(x, y)))
                        .unwrap(),
                );
            }
        }
        output
    }
    pub fn rotate_square(&self, coordinate: &UCoordinate) -> usize {
        // simple maths to transpose a given X/Y co-ordinate to it's rotated value
        match self.rotation {
            Rotation::Zero => (coordinate.y * 4 + coordinate.x) as usize,
            Rotation::Ninety => (12 + coordinate.y - (coordinate.x * 4)) as usize,
            Rotation::OneEighty => (15 - (coordinate.y * 4) - coordinate.x) as usize,
            Rotation::TwoSeventy => (3 - coordinate.y + (coordinate.x * 4)) as usize,
        }
    }
    pub fn get_val_at_xy(&mut self, coordinate: &UCoordinate) -> char {
        // this allows us to take an X,Y and return its value in flattened tetromino
        self.get_rotated_tetromino()
            .chars()
            .nth((coordinate.x + coordinate.y * TETROMINO_SIZE) as usize)
            .unwrap()
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
        let mut tetromino = Tetromino::new(TetrominoShape::I);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_I);
        let mut tetromino = Tetromino::new(TetrominoShape::O);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_O);
        let mut tetromino = Tetromino::new(TetrominoShape::T);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_T);
        let mut tetromino = Tetromino::new(TetrominoShape::J);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_J);
        let mut tetromino = Tetromino::new(TetrominoShape::L);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_L);
        let mut tetromino = Tetromino::new(TetrominoShape::S);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_S);
        let mut tetromino = Tetromino::new(TetrominoShape::Z);
        assert_eq!(tetromino.get_rotated_tetromino(), TETROMINO_Z);
    }
    #[test]
    fn test_rotated_position() {
        let mut tetromino = Tetromino::new(TetrominoShape::I);
        let val = tetromino.get_val_at_xy(&UCoordinate::new(0, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(1, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(2, 0));
        assert_eq!(val, 'X');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(3, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(0, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(1, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(2, 1));
        assert_eq!(val, 'X');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(3, 1));
        assert_eq!(val, '.');
        tetromino.rotate();
        let val = tetromino.get_val_at_xy(&UCoordinate::new(0, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(1, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(2, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(3, 0));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(0, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(1, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(2, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(3, 1));
        assert_eq!(val, '.');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(0, 2));
        assert_eq!(val, 'X');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(1, 2));
        assert_eq!(val, 'X');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(2, 2));
        assert_eq!(val, 'X');
        let val = tetromino.get_val_at_xy(&UCoordinate::new(3, 2));
        assert_eq!(val, 'X');
    }
}
