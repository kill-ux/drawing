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

pub struct Line(Point, Point, Color);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut line = Self(p1.clone(), p2.clone(), Color::red());
        line.2 = line.color() ;
        line
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

        let mut x = x0;
        let mut y: i32 = y0;
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err_prev;

        loop {
            image.display(x, y, self.2.clone());
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
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle(p1.clone(), p2.clone(), p3.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut lin1 = Line::new(&self.0, &self.1);
        let mut lin2 = Line::new(&self.1, &self.2);
        let mut lin3 = Line::new(&self.2, &self.0);
        lin1.2 = color.clone() ;
        lin2.2 = color.clone() ;
        lin3.2 = color.clone() ;
        lin1.draw(image);
        lin2.draw(image);
        lin3.draw(image);
    }
}

pub struct Rectangle(Point, Point);
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let p1: Point = Point(self.0.0, self.1.1);
        let p2: Point = Point(self.1.0, self.0.1);
        let color = self.color();
        let mut lin1 = Line::new(&p1, &self.1);
        let mut lin2 = Line::new(&self.1, &p2);
        let mut lin3 = Line::new(&p2, &self.0);
        let mut lin4 = Line::new(&self.0, &p1);
        lin1.2 = color.clone() ;
        lin2.2 = color.clone() ;
        lin3.2 = color.clone() ;
        lin4.2 = color.clone() ;
        lin1.draw(image);
        lin2.draw(image);
        lin3.draw(image);
        lin4.draw(image);
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
        Self::new(
            &Point::random(width, height),
            rand::rng().random_range(0..width.min(height)),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut last_x = ((self.radius as f32) * 0f32.cos() + (self.center.0 as f32)) as i32;
        let mut last_y = ((self.radius as f32) * 0f32.sin() + (self.center.1 as f32)) as i32;
        for i in 0..=360 {
            let p1 = Point::new(last_x, last_y);
            let ang = (i as f32) * std::f32::consts::PI / 180.0;
            let x = ((self.radius as f32) * ang.cos() + (self.center.0 as f32)) as i32;
            let y = ((self.radius as f32) * ang.sin() + (self.center.1 as f32)) as i32;
            let p2 = Point::new(x, y);
            last_x = x;
            last_y = y;
            let mut line = Line::new(&p1, &p2);
            line.2 = color.clone();
            line.draw(image);
        }
    }
}
