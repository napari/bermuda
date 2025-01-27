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

/// Determines the orientation of three points (p, q, r).
///
/// This function calculates the orientation of the ordered triplet (p, q, r).
/// The possible return values and their meanings are:
///
/// * 0: The points are collinear.
/// * 1: Clockwise orientation.
/// * 2: Counterclockwise orientation.
///
/// # Arguments
///
/// * `p` - The first [`Point`](crate::point::Point).
/// * `q` - The second [`Point`](crate::point::Point).
/// * `r` - The third [`Point`](crate::point::Point).
///
/// # Returns
///
/// An integer representing the orientation: 0 for collinear, 1 for clockwise, 2 for counterclockwise.
///
/// # Example
///
/// ```rust
/// use triangulation::point::Point;
/// use triangulation::intersection::orientation;
///
/// let p = Point::new(0.0, 0.0);
/// let q = Point::new(1.0, 1.0);
/// let r = Point::new(2.0, 2.0);
///
/// assert_eq!(orientation(p, q, r), 0); // Collinear points
///
/// let r_clockwise = Point::new(2.0, 0.0);
/// assert_eq!(orientation(p, q, r_clockwise), 1); // Clockwise orientation
///
/// let r_counterclockwise = Point::new(0.0, 2.0);
/// assert_eq!(orientation(p, q, r_counterclockwise), 2); // Counterclockwise orientation
///
/// ```

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

/// Determines if two segments intersect.
///
/// This function checks whether two line segments, `s1` and `s2`, intersect with each other.
///
/// # Arguments
///
/// * `s1` - A reference to the first [`Segment`](crate::point::Segment).
/// * `s2` - A reference to the second [`Segment`](crate::point::Segment).
///
/// # Returns
///
/// * `true` - If the segments intersect.
/// * `false` - If the segments do not intersect.
///
/// # Examples
///
/// ```rust
/// use triangulation::point::{Point, Segment};
/// use triangulation::intersection::do_intersect;
///
/// let seg1 = Segment::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
/// let seg2 = Segment::new(Point::new(0.0, 4.0), Point::new(4.0, 0.0));
///
/// assert!(do_intersect(&seg1, &seg2)); // The segments intersect
///
/// let seg3 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
/// let seg4 = Segment::new(Point::new(3.0, 3.0), Point::new(4.0, 4.0));
///
/// assert!(!do_intersect(&seg3, &seg4)); // The segments do not intersect
/// ```
pub fn do_intersect(s1: &point::Segment, s2: &point::Segment) -> bool {
    let p1 = s1.bottom;
    let q1 = s1.top;
    let p2 = s2.bottom;
    let q2 = s2.top;

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == 0 && on_segment_if_collinear(p1, p2, q1) {
        return true;
    }
    if o2 == 0 && on_segment_if_collinear(p1, q2, q1) {
        return true;
    }
    if o3 == 0 && on_segment_if_collinear(p2, p1, q2) {
        return true;
    }
    if o4 == 0 && on_segment_if_collinear(p2, q1, q2) {
        return true;
    }

    false
}

/// Checks if two segments share an endpoint.
///
/// This function determines whether two segments, each defined by
/// two endpoints, share any endpoint. Specifically, it checks if
/// the bottom or top endpoint of the first segment is equal to the
/// bottom or top endpoint of the second segment.
///
/// # Arguments
///
/// * `s1` - The first segment.
/// * `s2` - The second segment.
///
/// # Returns
///
/// `true` if the segments share at least one endpoint, `false` otherwise.
///
/// # Example
///
/// ```
/// use triangulation::point::{Point, Segment};
/// use triangulation::intersection::share_endpoint;
///
/// let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
/// let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0));
/// assert!(share_endpoint(&s1, &s2)); // Shared endpoint
///
/// let s3 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
/// let s4 = Segment::new(Point::new(2.0, 2.0), Point::new(3.0, 3.0));
/// assert!(!share_endpoint(&s3, &s4)); // No shared endpoint
/// ```
pub fn share_endpoint(s1: &point::Segment, s2: &point::Segment) -> bool {
    s1.bottom == s2.bottom || s1.bottom == s2.top || s1.top == s2.bottom || s1.top == s2.top
}
