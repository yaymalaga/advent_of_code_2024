#[derive(Debug)]
pub enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

pub struct Offset {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Direction {
    pub fn generate_directions_list() -> [Self; 8] {
        [
            Self::Top,
            Self::TopRight,
            Self::Right,
            Self::BottomRight,
            Self::Bottom,
            Self::BottomLeft,
            Self::Left,
            Self::TopLeft,
        ]
    }

    pub fn get_offset(direction: &Direction) -> Offset {
        match direction {
            Direction::Top => Offset { x: 0, y: -1 },
            Direction::TopRight => Offset { x: 1, y: -1 },
            Direction::Right => Offset { x: 1, y: 0 },
            Direction::BottomRight => Offset { x: 1, y: 1 },
            Direction::Bottom => Offset { x: 0, y: 1 },
            Direction::BottomLeft => Offset { x: -1, y: 1 },
            Direction::Left => Offset { x: -1, y: 0 },
            Direction::TopLeft => Offset { x: -1, y: -1 },
        }
    }

    pub fn apply_offset(direction: &Direction, x: isize, y: isize) -> Position {
        let offset = Self::get_offset(direction);

        Position {
            x: x + offset.x,
            y: y + offset.y,
        }
    }

    pub fn apply_90_clockwise_rotation(direction: &Direction) -> Direction {
        match direction {
            Direction::Top => Direction::Right,
            Direction::TopRight => Direction::BottomRight,
            Direction::Right => Direction::Bottom,
            Direction::BottomRight => Direction::BottomLeft,
            Direction::Bottom => Direction::Left,
            Direction::BottomLeft => Direction::TopLeft,
            Direction::Left => Direction::Top,
            Direction::TopLeft => Direction::TopRight,
        }
    }
}
