/// Cardinal direction used for snake movement.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::East => "east",
            Self::West => "west",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "north" => Some(Self::North),
            "south" => Some(Self::South),
            "east" => Some(Self::East),
            "west" => Some(Self::West),
            _ => None,
        }
    }
}

/// Compute direction *from* `a` *to* `b` (adjacent cells).
pub fn direction_between(a: (i32, i32), b: (i32, i32)) -> Direction {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    if dx > 0 {
        Direction::East
    } else if dx < 0 {
        Direction::West
    } else if dy > 0 {
        Direction::North
    } else {
        Direction::South
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_pairs() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    #[test]
    fn opposite_is_involution() {
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            assert_eq!(dir.opposite().opposite(), dir);
        }
    }

    #[test]
    fn delta_values() {
        assert_eq!(Direction::North.delta(), (0, 1));
        assert_eq!(Direction::South.delta(), (0, -1));
        assert_eq!(Direction::East.delta(), (1, 0));
        assert_eq!(Direction::West.delta(), (-1, 0));
    }

    #[test]
    fn as_str_roundtrip() {
        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            assert_eq!(Direction::from_str(dir.as_str()), Some(dir));
        }
    }

    #[test]
    fn from_str_invalid_returns_none() {
        assert_eq!(Direction::from_str("up"), None);
        assert_eq!(Direction::from_str(""), None);
        assert_eq!(Direction::from_str("North"), None); // case-sensitive
    }

    #[test]
    fn direction_between_adjacent_cells() {
        assert_eq!(direction_between((5, 5), (6, 5)), Direction::East);
        assert_eq!(direction_between((5, 5), (4, 5)), Direction::West);
        assert_eq!(direction_between((5, 5), (5, 6)), Direction::North);
        assert_eq!(direction_between((5, 5), (5, 4)), Direction::South);
    }

    #[test]
    fn direction_between_diagonal_prefers_horizontal() {
        // When both dx and dy are non-zero, horizontal (East/West) wins.
        assert_eq!(direction_between((0, 0), (1, 1)), Direction::East);
        assert_eq!(direction_between((0, 0), (-1, -1)), Direction::West);
    }
}
