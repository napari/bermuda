use crate::monotone_polygon::Side::LEFT;
use crate::point::{orientation, Orientation, Point, PointTriangle};
use std::cmp::PartialEq;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MonotonePolygon {
    top: Point,
    bottom: Option<Point>,
    left: Vec<Point>,
    right: Vec<Point>,
}

impl MonotonePolygon {
    pub fn new(top: Point) -> Self {
        MonotonePolygon {
            top,
            bottom: None,
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    /// Check if monotone polygon is finished by checking if bottom is set  
    pub fn finished(&self) -> bool {
        self.bottom.is_some()
    }
}

fn _build_triangles_opposite_edge(
    stack: &mut VecDeque<Point>,
    result: &mut Vec<PointTriangle>,
    current_point: Point,
) {
    for i in 0..stack.len() - 1 {
        result.push(PointTriangle::new(current_point, stack[i], stack[i + 1]));
    }

    let back = stack.pop_back().unwrap(); //  Get the last element
    stack.pop_front(); // Remove the first element.
    stack.push_front(back); //  Put last element at the beginning (equivalent to stack[0] = stack.back())
    stack.push_front(current_point); // Put the current point at the beginning (equivalent to stack[1] = current_point)

    // In Rust we don't need to manually erase from index 2 onwards. The
    // previous pop/push operations have already left only the first two elements
    // in the stack.
}

fn _build_triangles_current_edge(
    stack: &mut VecDeque<Point>,
    result: &mut Vec<PointTriangle>,
    current_point: Point,
    expected_orientation: &Side,
) {
    // Conversion of iterator logic to Rust using indices is generally preferred
    let mut i = stack.len() - 1;
    let orientation_ = if (*expected_orientation == Side::LEFT) {
        Orientation::Collinear
    } else {
        Orientation::CounterClockwise
    };

    while i > 0 && orientation(stack[i - 1], stack[i], current_point) == orientation_ {
        result.push(PointTriangle::new(current_point, stack[i], stack[i - 1]));
        i -= 1;
    }

    stack.truncate(i + 1); // Efficiently remove elements from the back by truncating the vector

    stack.push_back(current_point);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Side {
    TopOrBottom,
    LEFT,
    RIGHT,
}

pub fn triangulate_monotone_polygon(polygon: &MonotonePolygon) -> Vec<PointTriangle> {
    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    let mut stack: VecDeque<Point> = VecDeque::new(); // Using VecDeque for O(1) push_front
    let mut points = Vec::new();

    result.reserve(polygon.left.len() + polygon.right.len());
    points.reserve(polygon.left.len() + polygon.right.len() + 2);

    points.push((polygon.top, Side::TopOrBottom));

    while left_index < polygon.left.len() && right_index < polygon.right.len() {
        if polygon.left[left_index] < polygon.right[right_index] {
            points.push((polygon.right[right_index], Side::RIGHT));
            right_index += 1;
        } else {
            points.push((polygon.left[left_index], Side::LEFT));
            left_index += 1;
        }
    }

    while left_index < polygon.left.len() {
        points.push((polygon.left[left_index], Side::LEFT));
        left_index += 1;
    }

    while right_index < polygon.right.len() {
        points.push((polygon.right[right_index], Side::RIGHT));
        right_index += 1;
    }

    points.push((polygon.bottom.unwrap(), Side::TopOrBottom));

    stack.push_back(points[0].0);
    stack.push_back(points[1].0);
    let mut side = &points[1].1;

    for i in 2..points.len() {
        if *side == points[i].1 {
            _build_triangles_current_edge(&mut stack, &mut result, points[i].0, side);
        } else {
            _build_triangles_opposite_edge(&mut stack, &mut result, points[i].0);
        }
        side = &points[i].1;
    }

    result
}
