use rand::prelude::*;
use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
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
    let x0 = line.start.x;
    let y0 = line.start.y;
    let x1 = line.end.x;
    let y1 = line.end.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut x = x0;
    let mut y = y0;
    let mut range = if dx > dy { dx } else { -dy } / 2;
    let mut range_prev;
    loop {
        image.display(x, y, color.clone());
        if x == x1 && y == y1 {
            break;
        }

        range_prev = range;
        if range_prev > -dx {
            range -= dy;
            x += sx;
        }

        if range_prev < dy {
            range += dx;
            y += sy;
        }
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
    point2: Point,
}

impl Cube {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            point1: p1.clone(),
            point2: p2.clone(),
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let d = ((self.point2.x - self.point1.x).abs()) / 3;
        let p1: Point = Point {
            x: self.point1.x,
            y: self.point2.y,
        };
        let p2 = &self.point2;
        let p3 = Point {
            x: self.point2.x,
            y: self.point1.y,
        };
        let p4 = &self.point1;

        let p5: Point = Point {
            x: self.point2.x + d,
            y: self.point2.y - d,
        };
        let p6 = Point {
            x: p1.x + d,
            y: p1.y - d,
        };
        let p7 = Point {
            x: self.point1.x + d,
            y: self.point1.y - d,
        };
        let p8 = Point {
            x: p3.x + d,
            y: p3.y - d,
        };

        let color = self.color();
        draw_line(&Line::new(&p1, &p2), image, color.clone());
        draw_line(&Line::new(&p2, &p3), image, color.clone());
        draw_line(&Line::new(&p3, &p4), image, color.clone());
        draw_line(&Line::new(&p4, &p1), image, color.clone());

        draw_line(&Line::new(&p5, &p6), image, color.clone());
        draw_line(&Line::new(&p6, &p7), image, color.clone());
        draw_line(&Line::new(&p7, &p8), image, color.clone());
        draw_line(&Line::new(&p8, &p5), image, color.clone());

        draw_line(&Line::new(&p2, &p5), image, color.clone());
        draw_line(&Line::new(&p1, &p6), image, color.clone());
        draw_line(&Line::new(&p4, &p7), image, color.clone());
        draw_line(&Line::new(&p3, &p8), image, color.clone());
    }
}
