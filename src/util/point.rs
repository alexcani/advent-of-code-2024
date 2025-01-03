use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONALS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right, top to bottom
pub const DIAGONALS: [Point; 8] = [
    Point::new(-1, -1),
    UP,
    Point::new(1, -1),
    LEFT,
    RIGHT,
    Point::new(-1, 1),
    DOWN,
    Point::new(1, 1),
];
pub const UPPER_LEFT: Point = DIAGONALS[0];
pub const UPPER_RIGHT: Point = DIAGONALS[2];
pub const LOWER_LEFT: Point = DIAGONALS[5];
pub const LOWER_RIGHT: Point = DIAGONALS[7];

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl From<u8> for Point {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'>' | b'R' => RIGHT,
            b'<' | b'L' => LEFT,
            b'^' | b'U' => UP,
            b'v' | b'D' => DOWN,
            _ => panic!("Unknown direction: {}", value),
        }
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Neg for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
