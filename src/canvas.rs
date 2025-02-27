mod color;
use color::*;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Canvas{
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

#[allow(dead_code)]
impl Canvas {
    fn new(height: usize, width: usize) -> Canvas {
        Canvas {
            height,
            width,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0, 0.0); width]; height]
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
    }

    fn get_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    fn to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write(self.get_header().as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use crate::canvas::color::Color;
    use super::Canvas;

    #[test]
    fn test_create() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.height, 10);
        assert_eq!(c.width, 20);
        let test_color = Color::new(0.0, 0.0, 0.0, 0.0);
        for row in c.pixels {
            for val in row {
                assert_eq!(val, test_color);
            }
        }
    }

    #[test]
    fn test_set_pixel() {
        let mut c = Canvas::new(10, 20);
        let default_color = Color::new(0.0, 0.0, 0.0, 0.0);
        let new_color = Color::new(1.0, 0.0, 0.0, 0.0);
        c.set_pixel(2, 3, new_color);
        for (y, row) in c.pixels.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if x == 2 && y == 3 {
                    assert_eq!(*val, new_color);
                } else {
                    assert_eq!(*val, default_color);
                }
            }
        }
    }

    #[test]
    fn test_header() {
        let c = Canvas::new(5, 3);
        assert_eq!(c.get_header(), "P3\n3 5\n255\n")
    }

    #[test]
    fn test_file_data() {
        
    }
}