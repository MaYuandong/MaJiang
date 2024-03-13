use core::fmt;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Tile {
    number: u8,
    kind: char,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.kind)
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
    let mut tiles: Vec<Tile> = [first, second, tested].to_vec();
    tiles.sort_by_key(|key: &Tile| (key.kind, key.number));
    match (tiles[2] - tiles[1], tiles[1] - tiles[0]) {
        ((1, 0), (1, 0)) => true,
        _ => false,
    }
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
            number: 7,
            kind: 'B',
        };

        let b2 = Tile {
            number: 8,
            kind: 'B',
        };

        let b3 = Tile {
            number: 9,
            kind: 'B',
        };
        let b4 = Tile {
            number: 9,
            kind: 'B',
        };
        let b5 = Tile {
            number: 9,
            kind: 'B',
        };
        let b6 = Tile {
            number: 9,
            kind: 'B',
        };
        assert_eq!(is_serial(b1, b2, b3), true);
        assert_eq!(is_triple(b4, b5, b6), true);
    }
}
