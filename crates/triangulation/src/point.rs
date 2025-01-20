use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

pub(crate) type Coord = f32;
pub(crate) type Index = usize;
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Point) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add_vector(&self, vector: &Vector) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y == other.y {
            self.x.partial_cmp(&other.x)
        } else {
            self.y.partial_cmp(&other.y)
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x_hash = (self.x.to_bits() as u64).rotate_left(16);
        let y_hash = self.y.to_bits() as u64;
        state.write_u64(x_hash ^ y_hash);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: Coord,
    pub y: Coord,
}

impl Vector {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn scale(&self, factor: Coord) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Div<Coord> for Vector {
    type Output = Vector;

    fn div(self, rhs: Coord) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<Coord> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Coord) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub top: Point,
    pub bottom: Point,
}

impl Segment {
    pub fn new(p1: Point, p2: Point) -> Self {
        if p1 == p2 {
            panic!("Segment cannot have two identical points: {}", p1);
        }

        if p1 < p2 {
            Self {
                bottom: p1,
                top: p2,
            }
        } else {
            Self {
                bottom: p2,
                top: p1,
            }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.bottom.y == self.top.y
    }

    pub fn is_vertical(&self) -> bool {
        self.bottom.x == self.top.x
    }

    pub fn point_on_line_x(&self, y: Coord) -> Coord {
        if self.bottom.y == self.top.y {
            self.bottom.x
        } else {
            self.bottom.x
                + (y - self.bottom.y)
                    * ((self.top.x - self.bottom.x) / (self.top.y - self.bottom.y))
        }
    }

    pub fn point_projection_factor(&self, p: Point) -> Coord {
        let numerator = (p.x - self.top.x) * (self.bottom.x - self.top.x)
            + (p.y - self.top.y) * (self.bottom.y - self.top.y);
        let denominator =
            (self.top.x - self.bottom.x).powi(2) + (self.top.y - self.bottom.y).powi(2);

        numerator / denominator
    }

    pub fn point_on_line(&self, p: Point) -> bool {
        if self.is_horizontal() {
            return self.bottom.x <= p.x && p.x <= self.top.x;
        }

        if self.is_vertical() {
            return self.bottom.y <= p.y && p.y <= self.top.y;
        }

        let x_coord = self.point_on_line_x(p.y);
        self.bottom.x <= x_coord && x_coord <= self.top.x
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[bottom={}, top={}]", self.bottom, self.top)
    }
}

impl Hash for Segment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bottom.hash(state);
        self.top.hash(state);
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.bottom == other.bottom && self.top == other.top
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.bottom == other.bottom {
            self.top.partial_cmp(&other.top)
        } else {
            self.bottom.partial_cmp(&other.bottom)
        }
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub x: Index,
    pub y: Index,
    pub z: Index,
}

impl Triangle {
    pub fn new(x: Index, y: Index, z: Index) -> Self {
        Triangle { x, y, z }
    }
}

pub fn vector_length(p1: Point, p2: Point) -> Coord {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
