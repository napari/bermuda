use crate::point;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub p: point::Point,
    pub index: point::Index,
    pub is_top: bool,
}

impl Event {
    pub fn new(p: point::Point, index: point::Index, is_top: bool) -> Self {
        Self { p, index, is_top }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p == other.p {
            if self.is_top == other.is_top {
                self.index.cmp(&other.index)
            } else {
                // Note the reversed comparison for is_top
                other.is_top.cmp(&self.is_top)
            }
        } else {
            // Assuming Point implements PartialOrd
            self.p.cmp(&other.p)
        }
    }
}

/// Checks if point `q` lies on the segment defined by points `p` and `r`, assuming all three points are collinear.
///
/// # Arguments
///
/// * `p` - A [`Point`](crate::point::Point) representing one endpoint of the segment.
/// * `q` - A [`Point`](crate::point::Point) to check if it lies on the segment.
/// * `r` - A [`Point`](crate::point::Point) representing the other endpoint of the segment.
///
/// # Returns
///
///
/// * `true` - If `q` lies on the segment defined by `p` and `r`.
/// * `false` - If `q` does not lie on the segment.
///
/// # Example
///
/// ```rust
/// use triangulation::point::Point;
/// use triangulation::intersection::on_segment_if_collinear;
///
/// let p = Point::new(0.0, 0.0);
/// let r = Point::new(4.0, 4.0);
/// let q = Point::new(2.0, 2.0);
///
/// assert!(on_segment_if_collinear(p, q, r)); // `q` lies on the segment
///
/// let q_outside = Point::new(5.0, 5.0);
/// assert!(!on_segment_if_collinear(p, q_outside, r)); // `q_outside` does not lie on the segment
/// ```
pub fn on_segment_if_collinear(p: point::Point, q: point::Point, r: point::Point) -> bool {
    if q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y) {
        true
    } else {
        false
    }
}

pub fn orientation(p: point::Point, q: point::Point, r: point::Point) -> i32 {
    let val1 = (q.y - p.y) * (r.x - q.x);
    let val2 = (r.y - q.y) * (q.x - p.x);
    if val1 == val2 {
        0
    } else if val1 > val2 {
        1
    } else {
        2
    }
}
