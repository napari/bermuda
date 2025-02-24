#![allow(dead_code)]
use crate::monotone_polygon::MonotonePolygon;
use crate::point::{orientation, Index, Orientation, Point, Segment};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Interval {
    last_seen: Point,
    left_segment: Segment,
    right_segment: Segment,
    polygons_list: Vec<MonotonePolygon>,
}

#[derive(Debug, Clone)]
pub struct PointEdgeInfo {
    pub edge_index: Index,
    pub opposite_point: Point,
}

impl PointEdgeInfo {
    pub fn new(edge_index: Index, opposite_point: Point) -> Self {
        Self {
            edge_index,
            opposite_point,
        }
    }
}

// Implement comparison traits
impl PartialEq for PointEdgeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.opposite_point == other.opposite_point
    }
}

impl Eq for PointEdgeInfo {}

impl PartialOrd for PointEdgeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointEdgeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.opposite_point.cmp(&other.opposite_point)
    }
}

pub type PointToEdges = HashMap<Point, Vec<PointEdgeInfo>>;

fn get_points_edges(edges: &[Segment]) -> PointToEdges {
    let mut point_to_edges = PointToEdges::new();

    // Populate the map with edges
    for (i, edge) in edges.iter().enumerate() {
        point_to_edges
            .entry(edge.bottom)
            .or_default()
            .push(PointEdgeInfo {
                edge_index: i,
                opposite_point: edge.top,
            });

        point_to_edges
            .entry(edge.top)
            .or_default()
            .push(PointEdgeInfo {
                edge_index: i,
                opposite_point: edge.bottom,
            });
    }

    // Sort each vector of edges
    for edges_vec in point_to_edges.values_mut() {
        edges_vec.sort_by(|a, b| {
            // Note: We reverse the comparison to match the C++ version
            b.opposite_point.cmp(&a.opposite_point)
        });
    }

    point_to_edges
}

#[inline]
fn left_right_share_top(s1: &Segment, s2: &Segment) -> Ordering {
    match orientation(s1.bottom, s1.top, s2.bottom) {
        Orientation::CounterClockwise => Ordering::Less,
        Orientation::Clockwise => Ordering::Greater,
        Orientation::Collinear => Ordering::Equal,
    }
}

#[inline]
fn left_right_share_bottom(s1: &Segment, s2: &Segment) -> Ordering {
    match orientation(s1.top, s1.bottom, s2.top) {
        Orientation::CounterClockwise => Ordering::Greater,
        Orientation::Clockwise => Ordering::Less,
        Orientation::Collinear => Ordering::Equal,
    }
}

impl Interval {
    // Default constructor not needed in Rust as we're not using it

    // Constructor with segments only
    pub fn new(p: Point, left: Segment, right: Segment) -> Self {
        Self {
            last_seen: p,
            left_segment: left,
            right_segment: right,
            polygons_list: Vec::new(),
        }
    }

    // Constructor with segments and polygon
    pub fn with_polygon(p: Point, left: Segment, right: Segment, polygon: MonotonePolygon) -> Self {
        let polygons_list = vec![polygon];

        Self {
            last_seen: p,
            left_segment: left,
            right_segment: right,
            polygons_list,
        }
    }

    pub fn replace_segment(&mut self, old_segment: &Segment, new_segment: Segment) {
        if self.left_segment == *old_segment {
            self.left_segment = new_segment;
            return;
        } else if self.right_segment == *old_segment {
            self.right_segment = new_segment;
            return;
        }
        panic!("Segment not found in interval");
    }

    pub fn opposite_segment(&self, segment: &Segment) -> Segment {
        if *segment == self.left_segment {
            self.right_segment.clone()
        } else if *segment == self.right_segment {
            self.left_segment.clone()
        } else {
            panic!("Segment not found in interval");
        }
    }
}

// Display implementation for Interval
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Last Seen: {}, Left Segment: {}, Right Segment: {}, Polygons count: {}",
            self.last_seen,
            self.left_segment,
            self.right_segment,
            self.polygons_list.len()
        )
    }
}

// Debug implementation might be useful
impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interval")
            .field("last_seen", &self.last_seen)
            .field("left_segment", &self.left_segment)
            .field("right_segment", &self.right_segment)
            .field("polygons_count", &self.polygons_list.len())
            .finish()
    }
}

pub struct MonotonePolygonBuilder {
    segment_to_line: HashMap<Segment, Rc<RefCell<Interval>>>,
    edges: Vec<Segment>,
    point_to_edges: HashMap<Point, Vec<PointEdgeInfo>>,
    monotone_polygons: Vec<MonotonePolygon>,
}

fn get_left_right_edges_top(s1: &Segment, s2: &Segment) -> (Segment, Segment) {
    if orientation(s1.bottom, s1.top, s2.bottom) == Orientation::CounterClockwise {
        (s2.clone(), s1.clone())
    } else {
        (s1.clone(), s2.clone())
    }
}

fn get_left_right_edges_bottom(s1: &Segment, s2: &Segment) -> (Segment, Segment) {
    if orientation(s1.top, s1.bottom, s2.top) == Orientation::Clockwise {
        (s2.clone(), s1.clone())
    } else {
        (s1.clone(), s2.clone())
    }
}

impl MonotonePolygonBuilder {
    pub fn new() -> Self {
        Self {
            segment_to_line: HashMap::new(),
            edges: Vec::new(),
            point_to_edges: HashMap::new(),
            monotone_polygons: Vec::new(),
        }
    }

    pub fn new_with_edges(edges: Vec<Segment>) -> Self {
        let point_to_edges = get_points_edges(&edges);
        Self {
            segment_to_line: HashMap::new(),
            edges,
            point_to_edges,
            monotone_polygons: Vec::new(),
        }
    }

    fn get_left_right_edges_top(&self, p: &Point) -> (Segment, Segment) {
        let point_info = self.point_to_edges.get(p).unwrap();
        let fst_idx = point_info[0].edge_index;
        let snd_idx = point_info[1].edge_index;

        if orientation(
            self.edges[fst_idx].bottom,
            self.edges[fst_idx].top,
            self.edges[snd_idx].bottom,
        ) == Orientation::CounterClockwise
        {
            (self.edges[snd_idx].clone(), self.edges[fst_idx].clone())
        } else {
            (self.edges[fst_idx].clone(), self.edges[snd_idx].clone())
        }
    }

    fn get_left_right_edges_bottom(&self, p: &Point) -> (Segment, Segment) {
        let point_info = self.point_to_edges.get(p).unwrap();
        let fst_idx = point_info[0].edge_index;
        let snd_idx = point_info[1].edge_index;

        if orientation(
            self.edges[fst_idx].top,
            self.edges[fst_idx].bottom,
            self.edges[snd_idx].top,
        ) == Orientation::Clockwise
        {
            (self.edges[snd_idx].clone(), self.edges[fst_idx].clone())
        } else {
            (self.edges[fst_idx].clone(), self.edges[snd_idx].clone())
        }
    }

    fn process_end_point(
        &mut self,
        p: Point,
        edge_left: Segment,
        edge_right: Segment,
        interval: Rc<RefCell<Interval>>,
    ) {
        for polygon in &interval.borrow().polygons_list {
            let mut polygon = polygon.clone();
            polygon.bottom = Some(p);
            self.monotone_polygons.push(polygon);
        }

        self.segment_to_line.remove(&edge_left);
        self.segment_to_line.remove(&edge_right);
    }

    fn process_merge_point(&mut self, p: Point, edge_left: Segment, edge_right: Segment) {
        let left_interval_ref = self
            .segment_to_line
            .get_mut(&edge_left)
            .expect("Left edge interval not found")
            .clone();
        let right_interval_ref = self
            .segment_to_line
            .get(&edge_right)
            .expect("Right edge interval not found")
            .clone();

        if !Rc::ptr_eq(&left_interval_ref, &right_interval_ref) {
            #[cfg(debug_assertions)]
            {
                if right_interval_ref.borrow().right_segment == edge_right {
                    panic!(
                        "The right edge of merge point should be the left edge of the right interval.\n\
                        Got interval: {:?} and edge: {:?}",
                        right_interval_ref, edge_right
                    );
                }
            }

            self.segment_to_line.remove(&edge_right);
            self.segment_to_line.remove(&edge_left);

            left_interval_ref.borrow_mut().right_segment =
                right_interval_ref.borrow().right_segment.clone();

            #[cfg(debug_assertions)]
            {
                if !self
                    .segment_to_line
                    .contains_key(&right_interval_ref.borrow().right_segment)
                {
                    panic!("Segment not found in the map2");
                }
            }

            self.segment_to_line.insert(
                right_interval_ref.borrow().right_segment.clone(),
                left_interval_ref.clone(),
            );
            left_interval_ref.borrow_mut().last_seen = p;

            // Update polygons
            if let Some(last_polygon) = left_interval_ref.borrow_mut().polygons_list.last_mut() {
                last_polygon.right.push(p);
            }
            if let Some(first_polygon) = right_interval_ref.borrow_mut().polygons_list.first_mut() {
                first_polygon.left.push(p);
            }

            // Move polygons from right interval to left interval
            left_interval_ref
                .borrow_mut()
                .polygons_list
                .append(&mut right_interval_ref.borrow_mut().polygons_list);
        } else {
            // This is the end point
            self.process_end_point(p, edge_left, edge_right, left_interval_ref);
        }
    }

    fn process_normal_point(
        &mut self,
        p: Point,
        edge_top: Segment,
        edge_bottom: Segment,
    ) -> Result<(), String> {
        let interval_ref = self
            .segment_to_line
            .get_mut(&edge_top)
            .ok_or_else(|| "Segment not found in the map".to_string())?
            .clone();

        if interval_ref.borrow().polygons_list.len() > 1 {
            if edge_top == interval_ref.borrow().right_segment {
                // We are on right side of the interval
                // End all polygons except the first one
                for poly in interval_ref.borrow_mut().polygons_list.iter_mut().skip(1) {
                    let mut polygon = poly.clone();
                    polygon.bottom = Some(p);
                    self.monotone_polygons.push(polygon);
                }
            } else {
                // We are on left side of the interval
                // End all polygons except the last one
                if let Some((last, all_but_last)) =
                    interval_ref.borrow_mut().polygons_list.split_last_mut()
                {
                    for poly in all_but_last {
                        let mut polygon = poly.clone();
                        polygon.bottom = Some(p);
                        self.monotone_polygons.push(polygon);
                    }
                    interval_ref.borrow_mut().polygons_list[0] = last.to_owned();
                }
            }
            interval_ref.borrow_mut().polygons_list.truncate(1);
        }

        // Update the remaining polygon
        if edge_top == interval_ref.borrow().right_segment {
            if let Some(polygon) = interval_ref.borrow_mut().polygons_list.first_mut() {
                polygon.right.push(p);
            }
        } else if let Some(polygon) = interval_ref.borrow_mut().polygons_list.first_mut() {
            polygon.left.push(p);
        }

        self.segment_to_line
            .insert(edge_bottom.clone(), interval_ref.clone());
        interval_ref.borrow_mut().last_seen = p;
        interval_ref
            .borrow_mut()
            .replace_segment(&edge_top, edge_bottom);
        self.segment_to_line.remove(&edge_top);

        #[cfg(debug_assertions)]
        {
            if !self
                .segment_to_line
                .contains_key(&interval_ref.borrow().left_segment)
            {
                return Err("Left segment not found in the map".to_string());
            }
            if !self
                .segment_to_line
                .contains_key(&interval_ref.borrow().right_segment)
            {
                return Err("Right segment not found in the map".to_string());
            }
        }

        Ok(())
    }

    fn process_start_point(&mut self, p: Point, edge_left: Segment, edge_right: Segment) {
        let mut mut_interval = Interval::new(p, edge_left.clone(), edge_right.clone());
        mut_interval.polygons_list.push(MonotonePolygon::new_top(p));

        let line_interval = Rc::new(RefCell::new(mut_interval));
        self.segment_to_line
            .insert(edge_left, line_interval.clone());
        self.segment_to_line.insert(edge_right, line_interval);
    }

    fn find_interval_with_point(&self, p: Point) -> Option<Rc<RefCell<Interval>>> {
        for (segment, interval) in self.segment_to_line.iter() {
            if *segment == interval.borrow().right_segment {
                // as each interval is listed twice we
                // skip this pointed by right segment to avoid duplication
                continue;
            }
            if !(interval.borrow().left_segment.point_on_line_x(p.y) < p.x
                && interval.borrow().right_segment.point_on_line_x(p.y) > p.x)
            {
                // we are outside the interval so this interval do not contain the split point
                return Some(interval.clone());
            }
        }
        None
    }

    fn process_split_point(&mut self, p: Point, edge_left: Segment, edge_right: Segment) {
        if let Some(interval) = self.find_interval_with_point(p) {
            let right_segment = interval.borrow().right_segment.clone();
            interval.borrow_mut().right_segment = edge_left.clone();
            interval.borrow_mut().last_seen = p;
            self.segment_to_line
                .insert(edge_left.clone(), interval.clone());

            let new_interval = Rc::new(RefCell::new(Interval::new(
                p,
                edge_right.clone(),
                right_segment.clone(),
            )));
            self.segment_to_line
                .insert(edge_right.clone(), new_interval.clone());
            self.segment_to_line
                .insert(right_segment, new_interval.clone());

            if interval.borrow().polygons_list.len() == 1 {
                let mut new_polygon = if interval.borrow().polygons_list[0].right.is_empty() {
                    MonotonePolygon::new_top(interval.borrow().polygons_list[0].top)
                } else {
                    MonotonePolygon::new_top(
                        *interval.borrow().polygons_list[0].right.last().unwrap(),
                    )
                };
                new_polygon.left.push(p);
                new_interval.borrow_mut().polygons_list.push(new_polygon);
                interval.borrow_mut().polygons_list[0].right.push(p);
            }
            if interval.borrow().polygons_list.len() >= 2 {
                interval.borrow_mut().polygons_list[0].right.push(p);
                interval
                    .borrow_mut()
                    .polygons_list
                    .last_mut()
                    .unwrap()
                    .left
                    .push(p);
                for polygon in interval
                    .borrow()
                    .polygons_list
                    .iter()
                    .skip(1)
                    .take(interval.borrow().polygons_list.len() - 2)
                {
                    let mut poly = polygon.clone();
                    poly.bottom = Some(p);
                    self.monotone_polygons.push(poly);
                }
                if let Some(last_polygon) = interval.borrow_mut().polygons_list.pop() {
                    new_interval.borrow_mut().polygons_list.push(last_polygon)
                }
                interval.borrow_mut().polygons_list.truncate(1);
            }
        } else {
            self.process_start_point(p, edge_left, edge_right);
        }
    }

    fn process_intersection_point(&mut self, p: Point, edges: Vec<Segment>) -> Result<(), String> {
        let mut processed_segments = BTreeSet::new();
        let mut segments_to_normal_process = Vec::new();
        let mut top_segments = Vec::new();
        let mut bottom_segments = Vec::new();

        for edge in edges.iter() {
            if processed_segments.contains(edge) {
                continue;
            }
            if self.segment_to_line.contains_key(edge) {
                let interval = self.segment_to_line.get(edge).unwrap();
                let opposite_edge = interval.borrow().opposite_segment(edge);
                if edge.bottom == p && opposite_edge.bottom == p {
                    self.process_end_point(
                        p,
                        edge.clone(),
                        opposite_edge.clone(),
                        interval.clone(),
                    );
                    processed_segments.insert(opposite_edge);
                    processed_segments.insert(edge.clone());
                    continue;
                }
            }
            if edge.top == p {
                top_segments.push(edge.clone());
            } else {
                bottom_segments.push(edge.clone());
            }
            segments_to_normal_process.push(edge.clone());
        }
        if bottom_segments.is_empty() && top_segments.is_empty() {
            return Ok(());
        }

        top_segments.sort_by(left_right_share_bottom);
        bottom_segments.sort_by(left_right_share_top);

        let mut bottom_begin = bottom_segments.iter().peekable();
        let mut top_begin = top_segments.iter();
        if !top_segments.is_empty() {
            let first_top = top_segments.first().unwrap();
            if *first_top == self.segment_to_line[first_top].borrow().right_segment {
                top_begin.next();
                let first_bottom = bottom_begin.next().unwrap();
                self.process_normal_point(p, first_top.clone(), first_bottom.clone())?;
            }
            let top_segments_last = top_segments.last().unwrap();
            if top_begin.count() > 0
                && *top_segments_last
                    == self.segment_to_line[top_segments_last]
                        .borrow()
                        .left_segment
            {
                let last_bottom = bottom_begin.next_back().unwrap();
                self.process_normal_point(p, top_segments_last.clone(), last_bottom.clone())?;
            }
        }
        while bottom_begin.peek().is_some() {
            self.process_start_point(
                p,
                bottom_begin.next().unwrap().clone(),
                bottom_begin.next().unwrap().clone(),
            );
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PointType {
    Intersection(Vec<Segment>),
    Split(Segment, Segment),
    Merge(Segment, Segment),
    Normal(Segment, Segment),
}

pub fn get_point_type(p: Point, point_to_edges: &PointToEdges) -> PointType {
    match point_to_edges.get(&p) {
        None => panic!("Point not found in the map"),
        Some(edges) => {
            if edges.is_empty() {
                panic!("Empty point found in the map");
            }

            // Convert edge info to segments
            let segments: Vec<Segment> = edges
                .iter()
                .map(|edge_info| Segment::new(p, edge_info.opposite_point))
                .collect();

            if segments.len() != 2 {
                return PointType::Intersection(segments);
            }

            let (seg1, seg2) = (segments[0].clone(), segments[1].clone());

            // Both opposite points are less than p -> Split point
            if edges[0].opposite_point < p && edges[1].opposite_point < p {
                let (left, right) = get_left_right_edges_bottom(&seg1, &seg2);
                return PointType::Split(left, right);
            }
            // Both opposite points are greater than p -> Merge point
            if p < edges[0].opposite_point && p < edges[1].opposite_point {
                let (left, right) = get_left_right_edges_top(&seg1, &seg2);
                return PointType::Merge(left, right);
            }
            // Otherwise it's a normal point
            PointType::Normal(seg1, seg2)
        }
    }
}

pub fn sweeping_line_triangulation(edges: Vec<Segment>) -> Vec<MonotonePolygon> {
    let mut builder = MonotonePolygonBuilder::new_with_edges(edges);
    let mut points = builder
        .point_to_edges
        .keys()
        .cloned()
        .collect::<Vec<Point>>();
    points.sort();
    points.reverse();
    for p in points {
        let point_type = get_point_type(p, &builder.point_to_edges);
        match point_type {
            PointType::Intersection(li) => {
                builder.process_intersection_point(p, li).unwrap();
            }
            PointType::Split(left, right) => {
                builder.process_split_point(p, left, right);
            }
            PointType::Merge(left, right) => {
                builder.process_merge_point(p, left, right);
            }
            PointType::Normal(top, bottom) => {
                builder.process_normal_point(p, top, bottom).unwrap();
            }
        }
    }
    builder.monotone_polygons
}
