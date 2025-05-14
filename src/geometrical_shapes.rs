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
        Point { x, y }
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
        let color = self.color();
        draw_line(self, image, color.clone());
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
        let mut deta = 54.0;
        let ang_intial = (deta * std::f64::consts::PI / 180.0) as f32;
        let mut last_x =
            ((self.radius as f32) * ang_intial.cos() + self.center.x as f32).round() as i32;
        let mut last_y =
            ((self.radius as f32) * ang_intial.sin() + self.center.y as f32).round() as i32;
        let color = self.color();
        for i in 0..5 {
            deta += 72.0;
            let last_point = Point::new(last_x, last_y);
            let ang = (deta as f64) * std::f64::consts::PI / 180.0;
            let x = ((self.radius as f64) * ang.cos() + self.center.x as f64).round() as i32;
            let y = ((self.radius as f64) * ang.sin() + self.center.y as f64).round() as i32;
            last_x = x;
            last_y = y;
            draw_line(
                &Line::new(&last_point, &Point::new(x, y)),
                image,
                color.clone(),
            );
        }
    }
}
