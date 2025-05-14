use rand::prelude::*;
use raster::{Color, Image};

/// Displayable trait for desplay (x, y) in pixel
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/// Drawable trait for drow and pick random color
pub trait Drawable {
    /// draw your sheap
    fn draw(&self, image: &mut Image);
    /// random color
    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self {
            x: rng.random_range(0..width),
            y: rng.random_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        draw_line(self, image, self.color());
    }
}

fn draw_line(line: &Line, image: &mut Image, color: Color) {
    let mut x = line.start.x as f32;
    let mut y = line.start.y as f32;

    let dx = (line.end.x - line.start.x).abs();
    let dy = (line.end.y - line.start.y).abs();
    let steps = dx.max(dy);

    let sx = if line.end.x < line.start.x { -1.0 } else { 1.0 };
    let sy = if line.end.y < line.start.y { -1.0 } else { 1.0 };

    let x_inc = (dx as f32 / steps as f32) * sx;
    let y_inc = (dy as f32 / steps as f32) * sy;

    for _ in 0..=steps {
        image.display(x.round() as i32, y.round() as i32, color.clone());
        x += x_inc;
        y += y_inc;
    }
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        draw_line(&Line::new(&self.p1, &self.p2), image, color.clone());
        draw_line(&Line::new(&self.p2, &self.p3), image, color.clone());
        draw_line(&Line::new(&self.p3, &self.p1), image, color.clone());
    }
}

pub struct Rectangle {
    point1: Point,
    point2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            point1: p1.clone(),
            point2: p2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let p1 = Point {
            x: self.point1.x,
            y: self.point2.y,
        };
        let p2 = Point {
            x: self.point2.x,
            y: self.point1.y,
        };
        let color = self.color();
        draw_line(&Line::new(&p1, &self.point2), image, color.clone());
        draw_line(&Line::new(&self.point2, &p2), image, color.clone());
        draw_line(&Line::new(&p2, &self.point1), image, color.clone());
        draw_line(&Line::new(&self.point1, &p1), image, color.clone());
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(p1: &Point, radius: i32) -> Self {
        Self {
            center: p1.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(
            &Point::random(width, height),
            rng.random_range(0..width.min(height)),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let perimeter = (self.radius.pow(2) as f32) * std::f32::consts::PI;
        let inc = 360.0 / perimeter;
        let mut i: f32 = 0.0;
        while i <= 360.0 {
            let ang = (i as f32) * std::f32::consts::PI / 180.0;
            let x = ((self.radius as f32) * ang.cos() + (self.center.x as f32)).round() as i32;
            let y = ((self.radius as f32) * ang.sin() + (self.center.y as f32)).round() as i32;
            image.display(x, y, color.clone());
            i += inc;
        }
    }
}

pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(p1: &Point, radius: i32) -> Self {
        Self {
            center: p1.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(
            &Point::random(width, height),
            rng.random_range(0..width.min(height)),
        )
    }
}
impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let mut last_x = self.radius + self.center.x;
        let mut last_y = self.center.y;
        let color = self.color();
        for i in 1..=5 {
            let last_point = Point::new(last_x, last_y);
            let angle = ((i as f64 * (360.0 / 5.0)) * std::f64::consts::PI / 180.0) as f32;
            let x = ((self.radius as f32 * angle.cos()) + self.center.x as f32).floor() as i32;
            let y = ((self.radius as f32 * angle.sin()) + self.center.y as f32).floor() as i32;
            last_x = x;
            last_y = y;
            let line = Line::new(&last_point, &Point::new(x, y));
            draw_line(&line, image, color.clone());
        }
    }
}

pub struct Cube {
    point1: Point,
    side: i32,
}

impl Cube {
    pub fn new(p1: &Point, side: i32) -> Self {
        Self {
            point1: p1.clone(),
            side,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let p1 = &self.point1;
        let p2: Point = Point {
            x: self.point1.x + self.side,
            y: self.point1.y,
        };
        let p3: Point = Point {
            x: self.point1.x + self.side,
            y: self.point1.y + self.side,
        };
        let p4: Point = Point {
            x: self.point1.x,
            y: self.point1.y + self.side,
        };
        let d = ((p3.x - p1.x).abs()) / 3;
        let p1_b = Point {
            x: p1.x + d,
            y: p1.y - d,
        };
        let p2_b: Point = Point {
            x: p2.x + d,
            y: p2.y - d,
        };
        let p3_b: Point = Point {
            x: p3.x + d,
            y: p3.y - d,
        };
        let p4_b: Point = Point {
            x: p4.x + d,
            y: p4.y - d,
        };

        let color = self.color();
        draw_line(&Line::new(&p1, &p2), image, color.clone());
        draw_line(&Line::new(&p2, &p3), image, color.clone());
        draw_line(&Line::new(&p3, &p4), image, color.clone());
        draw_line(&Line::new(&p4, &p1), image, color.clone());

        draw_line(&Line::new(&p1_b, &p2_b), image, color.clone());
        draw_line(&Line::new(&p2_b, &p3_b), image, color.clone());
        draw_line(&Line::new(&p3_b, &p4_b), image, color.clone());
        draw_line(&Line::new(&p4_b, &p1_b), image, color.clone());

        draw_line(&Line::new(&p1, &p1_b), image, color.clone());
        draw_line(&Line::new(&p2, &p2_b), image, color.clone());
        draw_line(&Line::new(&p3, &p3_b), image, color.clone());
        draw_line(&Line::new(&p4, &p4_b), image, color.clone());
    }
}
