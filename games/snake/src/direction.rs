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
