use rand::Rng;
use sdl2::rect::Point;

pub fn is_point_in_polygon(point: Point, vertices: &[Point]) -> bool {
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
        _ => panic!("Unexpected side generated")
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