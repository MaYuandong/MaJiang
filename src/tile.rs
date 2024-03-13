use core::fmt;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct Tile {
    number: u8,
    kind: char,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.kind == other.kind
    }
}

impl Eq for Tile {}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.kind)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.number != other.number {
            self.number.cmp(&other.number)
        } else {
            self.kind.cmp(&other.kind)
        }
    }
}

impl Add<u8> for Tile {
    type Output = Tile;

    fn add(self, other: u8) -> Tile {
        Tile {
            number: self.number + other,
            kind: self.kind,
        }
    }
}

impl Sub for Tile {
    type Output = (u8, u8);

    fn sub(self, other: Tile) -> (u8, u8) {
        (
            self.number - other.number,
            self.kind as u8 - other.kind as u8,
        )
    }
}

fn is_serial(first: Tile, second: Tile, tested: Tile) -> bool {
    todo!();
}

fn is_triple(first: Tile, second: Tile, tested: Tile) -> bool {
    match (first - tested, second - tested) {
        ((0, 0), (0, 0)) => true,
        _ => false,
    }
}

fn is_pair(first: Tile, second: Tile) -> bool {
    match first - second {
        (0, 0) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tile() {
        let b1 = Tile {
            number: 8,
            kind: 'B',
        };

        let b2 = Tile {
            number: 9,
            kind: 'B',
        };

        let b3 = Tile {
            number: 9,
            kind: 'B',
        };
        println!("b1, b2, b3 : {} {} {}", b1, b2, b3);
        println!("b2 = b1 + 1 {} and b2 = b3 {}", b2 == b1 + 1, b2 == b3);
        println!("b2 - b1 {:#?}, b2 - b3 {:#?}", b2 - b1, b2 - b3);
    }
}
