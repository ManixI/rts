mod color;
use color::*;

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
}

#[cfg(test)]
mod tests{
    
}