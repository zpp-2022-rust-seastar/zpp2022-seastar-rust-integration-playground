#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Point;

        pub fn x(&self) -> f64;
        pub fn y(&self) -> f64;
        pub fn new_point(x: f64, y: f64) -> Box<Point>;
        pub fn is_cross_point(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) -> bool;
    }

    unsafe extern "C++" {
        include!("rust/../server/tasks/line.h");

        type Line;

        fn contains_point(&self, p: &Point) -> bool;
        fn new_line(a: f64, b: f64) -> UniquePtr<Line>;
    }
}

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

pub fn new_point(x: f64, y: f64) -> Box<Point> {
    Box::new(Point { x, y })
}

pub fn is_cross_point(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) -> bool {
    let point = Point { x, y };
    let line1 = ffi::new_line(a, b);
    let line2 = ffi::new_line(c, d);

    line1.contains_point(&point) && line2.contains_point(&point)
}
