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
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(rng.random_range(0..width), rng.random_range(0..height))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }
}

pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x0 = self.0.0;
        let y0 = self.0.1;
        let x1 = self.1.0;
        let y1 = self.1.1;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let color = self.color();
        let mut x = x0;
        let mut y: i32 = y0;
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err_prev;

        loop {
            image.display(x, y, color.clone());

            if x == x1 && y == y1 {
                break;
            }

            err_prev = err;

            if err_prev > -dx {
                err -= dy;
                x += sx;
            }
            if err_prev < dy {
                err += dx;
                y += sy;
            }
        }
    }
}

pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle(p1, p2, p3)
    }
}
