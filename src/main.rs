mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image: Image = Image::blank(1000, 1000);

    // points
    for _ in 1..1000 {
        gs::Point::random(image.width, image.height).draw(&mut image);
    }

    // lines
    for _ in 1..5 {
        gs::Line::random(1000, 1000).draw(&mut image);
    }

    // triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    // rectangle
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    // circles
    for _ in 1..20 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    // cube
    let cube = gs::Cube::new(&gs::Point::new(500, 500), 300);
    cube.draw(&mut image);

    // pentagon
    for _ in 1..5 {
        gs::Pentagon::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
