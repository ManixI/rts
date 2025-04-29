pub mod color;
use color::*;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Canvas{
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            height,
            width,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0, 0.0); width]; height]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        if y >= self.height || x >= self.width {
            return
        }
        self.pixels[y][x] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    fn get_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    fn get_canvas_as_ppm_data(&self) -> String {
        let out = self.pixels.iter().map(|row| {
            row.iter()
            .fold(String::new(), |s, c| format!("{}{}", s, c.values_as_str(255)))
        })
        .fold(String::new(), |s, val: String| format!("{}{}\n", s, val));

        let mut new_out = String::new();
        for line in out.lines() {
            let mut row = String::new();
            if line.len() > 70 {
                let vals = line.split_ascii_whitespace();
                let mut count = 0;

                for val in vals {
                    if count + val.len() > 70 {
                        row = row.trim().to_string();
                        row += "\n";
                        row += val;
                        row += " ";
                        count = val.len() + 1;
                    } else {
                        row += val;
                        row += " ";
                        count += val.len() + 1;
                    }
                }
            } else {
                row = line.to_string();
            }
            
            new_out += &row.trim();
            new_out += "\n";
        }

        new_out
    } 

    pub fn to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write(self.get_header().as_bytes())?;
        file.write(self.get_canvas_as_ppm_data().as_bytes())?;

        Ok(())
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

#[cfg(test)]
mod tests{
    use crate::canvas::color::Color;
    use super::Canvas;

    #[test]
    fn test_create() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
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
        let c = Canvas::new(3, 5);
        assert_eq!(c.get_header(), "P3\n3 5\n255\n")
    }

    #[test]
    fn test_file_data() {
        let mut c = Canvas::new(5, 3);
        c.set_pixel(0, 0, Color::new(1.5, 0.0, 0.0, 0.0));
        c.set_pixel(2, 1, Color::new(0.0, 0.5, 0.0, 0.0));
        c.set_pixel(4, 2, Color::new(-0.5, 0.0, 1.0, 1.0));


        assert_eq!(c.get_canvas_as_ppm_data(), 
        "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
    }

    #[test]
    fn test_file_data_line_length() {
        let mut c = Canvas::new(10, 2);
        let test_color = Color::new(1.0, 0.8, 0.6, 0.0);
        for x in 0..c.width {
            for y in 0..c.height {
                c.set_pixel(x, y, test_color);
            }
        }
        assert_eq!(c.get_canvas_as_ppm_data(), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
");
    }

    // need to manually check output with gimp
    #[test]
    #[ignore]
    fn test_write_to_file() {
        let c = Canvas::new(500, 500);
        c.to_file("test.ppm").unwrap();
    }
}