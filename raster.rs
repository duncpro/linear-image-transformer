#[derive(Clone, Copy)]
pub struct Pixel { pub red: u8, pub green: u8, pub blue: u8 }

pub struct Raster { pub pixels: Vec<Pixel>, pub width: usize }

impl Raster {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.pixels.len() / self.width }
    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        assert!(x < self.width);
        assert!(y < self.height());
        let i = y * self.width + x;
        self.pixels[i]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        assert!(x < self.width);
        assert!(y < self.height());
        let i = y * self.width + x;
        self.pixels[i] = pixel;
    }
    
    pub fn solid(color: Pixel, width: usize, height: usize) -> Self {
        let pixels = vec![color; width * height];
        Self { pixels, width }
    }
}


impl Pixel {
    pub fn black() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
}
