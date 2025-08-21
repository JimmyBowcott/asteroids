use rand::Rng;
use sdl2::rect::Point;
use std::f64::consts::PI;

pub fn is_point_in_polygon(point: Point, vertices: &[Point]) -> bool {
    if vertices.len() == 0 { return false }
    let mut is_inside = false;
    let mut j = vertices.len() - 1;

    for i in 0..vertices.len() {
        let vertex1 = vertices[i];
        let vertex2 = vertices[j];

        if (vertex1.y > point.y) != (vertex2.y > point.y)
            && (point.x
                < (vertex2.x - vertex1.x) * (point.y - vertex1.y) / (vertex2.y - vertex1.y)
                    + vertex1.x)
        {
            is_inside = !is_inside;
        }

        j = i;
    }
    is_inside
}

pub fn generate_spawn_points(screen_width: u32, screen_height: u32, margin: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x: f64;
    let y: f64;

    let sides = ['L', 'R', 'T', 'B'];
    let side = sides[rng.gen_range(0..4)];

    match side {
        'L' => {
            x = -margin;
            y = rng.gen_range(0.0..screen_height as f64);
        }
        'R' => {
            x = margin + screen_width as f64;
            y = rng.gen_range(0.0..screen_height as f64);
        }
        'T' => {
            y = margin + screen_height as f64;
            x = rng.gen_range(0.0..screen_width as f64);
        }
        'B' => {
            y = -margin;
            x = rng.gen_range(0.0..screen_width as f64);
        }
        _ => {
            x = 0.0;
            y = 0.0;
        }
    }

    (x, y)
}

pub fn interpolate(y: i32, y1: i32, y2: i32, x1: i32, x2: i32) -> i32 {
    if y1 == y2 {
        x1
    } else {
        x1 + (x2 - x1) * (y - y1) / (y2 - y1)
    }
}

pub fn generate_velocity(min: f64, max: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x_multiplier = if rng.gen::<bool>() { 1 } else { -1 };
    let velocity_x = rng.gen_range(min..max) * x_multiplier as f64;
    let y_multiplier = if rng.gen::<bool>() { 1 } else { -1 };
    let velocity_y = rng.gen_range(min..max) * y_multiplier as f64;
    (velocity_x, velocity_y)
}

pub fn triangle_polygon_collision(
    triangle: &Vec<Point>, 
    polygon: &[Point],
) -> bool {

    for &vertex in triangle {
        if is_point_in_polygon(vertex, polygon) {
            return true;
        }
    }

    for i in 0..triangle.len() {
        let p1 = triangle[i];
        let p2 = triangle[(i + 1) % triangle.len()];

        for j in 0..polygon.len() {
            let q1 = polygon[j];
            let q2 = polygon[(j + 1) % polygon.len()];

            if lines_intersect(p1, p2, q1, q2) {
                return true;
            }
        }
    }

    false
}

fn lines_intersect(
    p1: Point, p2: Point,
    q1: Point, q2: Point,
) -> bool {
    fn orientation(p: Point, q: Point, r: Point) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 {
            0
        } else if val > 0 {
            1
        } else {
            -1
        }
    }

    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    o1 != o2 && o3 != o4
}

pub fn get_vertices(point: (f64, f64), angle: f64, scale: f64) -> Vec<Point>{
    let mut vertices = Vec::new();
    let (x, y) = point;

    let tip_x = x + scale * (angle).cos();
    let tip_y = y + scale * (angle).sin();

    let left_x = x + 0.5 * scale * ((angle + 2.0 * PI / 3.0).cos());
    let left_y = y + 0.5 * scale * ((angle + 2.0 * PI / 3.0).sin());

    let right_x = x + 0.5 * scale * ((angle - 2.0 * PI / 3.0).cos());
    let right_y = y + 0.5 * scale * ((angle - 2.0 * PI / 3.0).sin());

    vertices.push(Point::new(tip_x as i32, tip_y as i32));
    vertices.push(Point::new(left_x as i32, left_y as i32));
    vertices.push(Point::new(right_x as i32, right_y as i32));
    vertices
}

