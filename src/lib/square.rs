#![allow(dead_code)]

pub type Square = u32;

pub const fn to_square(x: u32, y: u32) -> Square {
    x + (y * 8)
}

pub const fn from_square(square: Square) -> (u32, u32) {
    let x = square % 8;
    let y = square / 8;

    (x, y)
}

pub fn square_to_name(square: Square) -> String {
    let (x, y) = from_square(square);
    let col = (('A' as u8) + (x as u8)) as char;
    let row = (('1' as u8) + (y as u8)) as char;

    let mut square_name = String::new();
    square_name.push(col);
    square_name.push(row);

    square_name
}

pub const A1: Square = to_square(0, 0);
pub const A2: Square = to_square(0, 1);
pub const A3: Square = to_square(0, 2);
pub const A4: Square = to_square(0, 3);
pub const A5: Square = to_square(0, 4);
pub const A6: Square = to_square(0, 5);
pub const A7: Square = to_square(0, 6);
pub const A8: Square = to_square(0, 7);

pub const B1: Square = to_square(1, 0);
pub const B2: Square = to_square(1, 1);
pub const B3: Square = to_square(1, 2);
pub const B4: Square = to_square(1, 3);
pub const B5: Square = to_square(1, 4);
pub const B6: Square = to_square(1, 5);
pub const B7: Square = to_square(1, 6);
pub const B8: Square = to_square(1, 7);

pub const C1: Square = to_square(2, 0);
pub const C2: Square = to_square(2, 1);
pub const C3: Square = to_square(2, 2);
pub const C4: Square = to_square(2, 3);
pub const C5: Square = to_square(2, 4);
pub const C6: Square = to_square(2, 5);
pub const C7: Square = to_square(2, 6);
pub const C8: Square = to_square(2, 7);

pub const D1: Square = to_square(3, 0);
pub const D2: Square = to_square(3, 1);
pub const D3: Square = to_square(3, 2);
pub const D4: Square = to_square(3, 3);
pub const D5: Square = to_square(3, 4);
pub const D6: Square = to_square(3, 5);
pub const D7: Square = to_square(3, 6);
pub const D8: Square = to_square(3, 7);

pub const E1: Square = to_square(4, 0);
pub const E2: Square = to_square(4, 1);
pub const E3: Square = to_square(4, 2);
pub const E4: Square = to_square(4, 3);
pub const E5: Square = to_square(4, 4);
pub const E6: Square = to_square(4, 5);
pub const E7: Square = to_square(4, 6);
pub const E8: Square = to_square(4, 7);

pub const F1: Square = to_square(5, 0);
pub const F2: Square = to_square(5, 1);
pub const F3: Square = to_square(5, 2);
pub const F4: Square = to_square(5, 3);
pub const F5: Square = to_square(5, 4);
pub const F6: Square = to_square(5, 5);
pub const F7: Square = to_square(5, 6);
pub const F8: Square = to_square(5, 7);

pub const G1: Square = to_square(6, 0);
pub const G2: Square = to_square(6, 1);
pub const G3: Square = to_square(6, 2);
pub const G4: Square = to_square(6, 3);
pub const G5: Square = to_square(6, 4);
pub const G6: Square = to_square(6, 5);
pub const G7: Square = to_square(6, 6);
pub const G8: Square = to_square(6, 7);

pub const H1: Square = to_square(7, 0);
pub const H2: Square = to_square(7, 1);
pub const H3: Square = to_square(7, 2);
pub const H4: Square = to_square(7, 3);
pub const H5: Square = to_square(7, 4);
pub const H6: Square = to_square(7, 5);
pub const H7: Square = to_square(7, 6);
pub const H8: Square = to_square(7, 7);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_to_name() {
        assert_eq!(square_to_name(A1), "A1");
        assert_eq!(square_to_name(A8), "A8");
        assert_eq!(square_to_name(E4), "E4");
        assert_eq!(square_to_name(H1), "H1");
        assert_eq!(square_to_name(H8), "H8");
    }
}
