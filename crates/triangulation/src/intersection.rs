use crate::point;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::hash::Hash;

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

#[derive(Default, Clone)]
struct EventData {
    tops: Vec<usize>,
    bottoms: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedPair(point::Index, point::Index);

impl PartialOrd for OrderedPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedPair {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.min(self.1), self.0.max(self.1)).cmp(&(other.0.min(other.1), other.0.max(other.1)))
    }
}

impl OrderedPair {
    pub fn new(i1: point::Index, i2: point::Index) -> Self {
        OrderedPair(i1.min(i2), i1.max(i2))
    }

    pub fn first(&self) -> point::Index {
        self.0
    }

    pub fn second(&self) -> point::Index {
        self.1
    }
}

/// Checks if point `q` lies on the segment defined by points `p` and `r`, assuming all three points are collinear.
///
/// # Arguments
///
/// * `p` - A [`Point`](point::Point) representing one endpoint of the segment.
/// * `q` - A [`Point`](point::Point) to check if it lies on the segment.
/// * `r` - A [`Point`](point::Point) representing the other endpoint of the segment.
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

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise,
}

/// Determines the orientation of three points (p, q, r).
///
/// This function calculates the orientation of the ordered triplet (p, q, r).
/// The possible return values and their meanings are:
///
/// # Arguments
///
/// * `p` - The first [`Point`](point::Point).
/// * `q` - The second [`Point`](point::Point).
/// * `r` - The third [`Point`](point::Point).
///
/// # Returns
///
///  Proper Orientation Enum
///
/// # Example
///
/// ```rust
/// use triangulation::point::Point;
/// use triangulation::intersection::{orientation, Orientation};
///
/// let p = Point::new(0.0, 0.0);
/// let q = Point::new(1.0, 1.0);
/// let r = Point::new(2.0, 2.0);
///
/// assert_eq!(orientation(p, q, r), Orientation::Collinear); // Collinear points
///
/// let r_clockwise = Point::new(2.0, 0.0);
/// assert_eq!(orientation(p, q, r_clockwise), Orientation::Clockwise); // Clockwise orientation
///
/// let r_counterclockwise = Point::new(0.0, 2.0);
/// assert_eq!(orientation(p, q, r_counterclockwise), Orientation::CounterClockwise); // Counterclockwise orientation
///
/// ```

pub fn orientation(p: point::Point, q: point::Point, r: point::Point) -> Orientation {
    let val1 = (q.y - p.y) * (r.x - q.x);
    let val2 = (r.y - q.y) * (q.x - p.x);
    if val1 == val2 {
        Orientation::Collinear
    } else if val1 > val2 {
        Orientation::Clockwise
    } else {
        Orientation::CounterClockwise
    }
}

/// Determines if two segments intersect.
///
/// This function checks whether two line segments, `s1` and `s2`, intersect with each other.
///
/// # Arguments
///
/// * `s1` - A reference to the first [`Segment`](point::Segment).
/// * `s2` - A reference to the second [`Segment`](point::Segment).
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

    if o1 == Orientation::Collinear && on_segment_if_collinear(p1, p2, q1) {
        return true;
    }
    if o2 == Orientation::Collinear && on_segment_if_collinear(p1, q2, q1) {
        return true;
    }
    if o3 == Orientation::Collinear && on_segment_if_collinear(p2, p1, q2) {
        return true;
    }
    if o4 == Orientation::Collinear && on_segment_if_collinear(p2, q1, q2) {
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

#[derive(Debug, PartialEq)]
pub enum Intersection {
    NoIntersection,
    PointIntersection(point::Point),
    CollinearNoOverlap,
    CollinearWithOverlap(Vec<point::Point>),
}

/// Finds the intersection point of two line segments, if it exists.
///
/// This function calculates the intersection point of two given line segments.
/// Each segment is defined by two endpoints. If the segments do not intersect,
/// or are collinear and overlapping, the function returns a vector of the shared points.
/// If they are collinear and don't overlap, an empty vector is returned.
/// If they intersect at a single point, the function returns a vector containing that single point.
/// If the segments are not collinear but intersect, the function returns a vector containing the intersection point.
///
/// # Arguments
///
/// * `s1` - The first line segment.
/// * `s2` - The second line segment.
///
/// # Returns
///
/// An element of Intersection enum with intersection points
///
/// # Example
///
/// ```
/// use triangulation::point::{Point, Segment};
/// use triangulation::intersection::{find_intersection, Intersection};
///
/// let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
/// let s2 = Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0));
/// let intersection = find_intersection(&s1, &s2);
/// assert_eq!(intersection, Intersection::PointIntersection(Point::new(1.0, 1.0))); // Intersecting segments
///
/// let s3 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
/// let s4 = Segment::new(Point::new(2.0, 2.0), Point::new(3.0, 3.0));
/// let intersection = find_intersection(&s3, &s4);
/// assert!(matches!(intersection, Intersection::CollinearNoOverlap)); // Non-intersecting segments
///
/// let s5 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 0.0));
/// let s6 = Segment::new(Point::new(1.0, 0.0), Point::new(3.0, 0.0));
/// let intersection = find_intersection(&s5, &s6);
/// assert!(matches!(intersection, Intersection::CollinearWithOverlap(_))); // Overlapping collinear segments
///
///
/// ```
pub fn find_intersection(s1: &point::Segment, s2: &point::Segment) -> Intersection {
    let a1 = s1.top.y - s1.bottom.y;
    let b1 = s1.bottom.x - s1.top.x;
    let a2 = s2.top.y - s2.bottom.y;
    let b2 = s2.bottom.x - s2.top.x;
    let det = a1 * b2 - a2 * b1;

    if det == 0.0 {
        // collinear case
        let mut res = Vec::new();
        if s1.point_on_line(s2.bottom) {
            res.push(s2.bottom);
        }
        if s1.point_on_line(s2.top) {
            res.push(s2.top);
        }
        if s2.point_on_line(s1.bottom) {
            res.push(s1.bottom);
        }
        if s2.point_on_line(s1.top) {
            res.push(s1.top);
        }

        // remove duplicates from the collinear intersection case
        res.sort();
        res.dedup();
        if res.len() == 0 {
            return Intersection::CollinearNoOverlap;
        }
        if res.len() == 1 {
            return Intersection::PointIntersection(res[0]);
        }
        return Intersection::CollinearWithOverlap(res);
    }

    let t = ((s2.top.x - s1.top.x) * (s2.bottom.y - s2.top.y)
        - (s2.top.y - s1.top.y) * (s2.bottom.x - s2.top.x))
        / det;

    // clip to handle problems with floating point precision
    if t < 0.0 {
        return Intersection::PointIntersection(s1.top);
    }
    if t > 1.0 {
        return Intersection::PointIntersection(s1.bottom);
    }

    let x = s1.top.x + t * b1;
    let y = s1.top.y + t * (-a1);
    Intersection::PointIntersection(point::Point { x, y })
}

/// Finds intersections among a set of line segments.
///
/// This function takes a vector of line segments and returns a set of pairs of
/// segment indices that intersect. The pairs are ordered to ensure uniqueness
/// regardless of the order of segments in the input vector.
///
/// # Arguments
///
/// * `segments` - A vector of [`Segment`](point::Segment) representing the line segments.
///
/// # Returns
///
/// A [`HashSet`](HashSet) of [`OrderedPair`], where each `OrderedPair` contains the indices of two intersecting segments.
///
/// # Example
///
/// ```
/// use triangulation::point::{Point, Segment};
/// use triangulation::intersection::{find_intersections, OrderedPair};
/// use std::collections::HashSet;
///
/// let segments = vec![
///     Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0)),
///     Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0)),
/// ];
/// let intersections = find_intersections(&segments);
///
/// let expected_intersections: HashSet<OrderedPair> = [(0, 1)].iter().map(|&(a, b)| OrderedPair::new(a, b)).collect();
/// assert_eq!(intersections, expected_intersections);
/// ```
pub fn find_intersections(segments: &[point::Segment]) -> HashSet<OrderedPair> {
    let mut intersections = HashSet::new();
    let mut intersection_events: BTreeMap<point::Point, EventData> = BTreeMap::new();
    for (i, segment) in segments.iter().enumerate() {
        intersection_events
            .entry(segment.top)
            .or_default()
            .tops
            .push(i);
        intersection_events
            .entry(segment.bottom)
            .or_default()
            .bottoms
            .push(i);
    }

    let mut active: BTreeMap<point::Point, HashSet<point::Index>> = BTreeMap::new();

    while let Some((&point, event_data)) = intersection_events.iter().next_back() {
        for &event_index in &event_data.tops {
            for active_el in active.iter() {
                for &index in active_el.1 {
                    if do_intersect(&segments[event_index], &segments[index])
                        && !share_endpoint(&segments[event_index], &segments[index])
                    {
                        intersections.insert(OrderedPair::new(event_index, index));
                    }
                }
            }
        }
        active
            .entry(point)
            .or_default()
            .extend(event_data.tops.iter());

        for &event_index in &event_data.bottoms {
            if let Some(entry) = active.get_mut(&segments[event_index].top) {
                entry.remove(&event_index);
                if entry.is_empty() {
                    active.remove(&segments[event_index].top);
                }
            }
        }

        intersection_events.remove(&point);
    }

    intersections
}
