use std::path::Path;

use glam::UVec2;

use crate::common::color::Color;

fn scale_color(color: Color) -> (u8, u8, u8) {
    (
        scale_color_component(color.r),
        scale_color_component(color.g),
        scale_color_component(color.b),
    )
}

fn scale_color_component(component: f64) -> u8 {
    let component = if component < 0.0 {
        0.0
    } else if component > 1.0 {
        1.0
    } else {
        component
    };

    (component * 255.0) as u8
}

pub struct Canvas {
    size: UVec2,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: UVec2::new(width, height),
            pixels: vec![Color::BLACK; (width * height) as usize],
        }
    }

    pub fn export<P: AsRef<Path>>(&self, path: P) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.size.x, self.size.y);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let (r, g, b) = scale_color(self[y as usize][x as usize]);
            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn pixels_mut(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }
}

impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    // row major
    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.size.x as usize;

        &self.pixels[start..start + self.size.x as usize]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    // row major
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.size.x as usize;

        &mut self.pixels[start..start + self.size.x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.size.x, 10);
        assert_eq!(c.size.y, 20);
        for row in 0..c.size.y-1 {
            for col in 0..c.size.x-1 {
                assert_eq!(c[row as usize][col as usize], Color::BLACK); 
            }
        }
    }

    #[test]
    fn set_pixel() {
        let mut c = Canvas::new(10, 20);
        c[2][3] = Color::RED;
        assert_eq!(c[2][3], Color::RED);
    }
}