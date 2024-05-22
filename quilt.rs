use crate::matrix::Matrix;
use crate::raster::Raster;
use crate::geom::PointLike2D;

pub fn knit(image: Raster) -> Quilt {
    let vertex_count = (image.width() + 1) * (image.height() + 1);
    let pixel_count = image.width() * image.height();
    
    let mut locmat: Matrix<f64> = Matrix::new(3, vertex_count);
    let mut colmat: Matrix<u8> = Matrix::new(4, pixel_count);

    let mut vi = 0;
    for vy in 0..(image.height() + 1) {
        for vx in 0..(image.width() + 1) {
            let vec: &mut [f64] = locmat.get_col_mut(vi);
            vec[0] = vx as f64;
            vec[1] = vy as f64;
            vec[2] = 1.0f64;
            vi += 1;
        }
    }

    let mut pi = 0;
    for py in 0..image.height() {
        for px in 0..image.width() {
            let color = image.get_pixel(px, py);
            let vec: &mut [u8] = colmat.get_col_mut(pi);
            vec[0] = color.red;
            vec[1] = color.green;
            vec[2] = color.blue;
            vec[3] = 1;
            pi += 1;
        }
    }

    Quilt { locmat, colmat, pwidth: image.width(), pheight: image.height() }
}

/// An image represented as a quilt of colored parallelograms of uniform dimension.
///
/// ```
/// p1 ------------ p2
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// p3 ------------ p4
/// ```
///
pub struct Quilt {
    pub locmat: Matrix<f64>,
    pub colmat: Matrix<u8>,
    pub pwidth: usize,
    pub pheight: usize
}

pub struct Vertex<'a> { colv_ref: &'a [f64] }

pub struct TileColor<'a> { colv_ref: &'a [u8] }

#[derive(Clone)]
pub struct Tile<'a> {
    quilt: &'a Quilt,
    px: usize,
    py: usize,   
    pi: usize,
    uli: usize
}

impl Quilt {
    pub fn tiles<'a>(&'a self) -> TileIterator<'a> {
        let initial = Tile { quilt: self, px: 0, py: 0, pi: 0, uli: 0 };
        return TileIterator { tile: initial };
    }
    pub fn p1<'a>(&'a self) -> Vertex<'a> {
        let colv_ref = self.locmat.get_col(0);
        Vertex { colv_ref }
    }
    pub fn p2<'a>(&'a self) -> Vertex<'a> {
        let colv_ref = self.locmat.get_col(self.pwidth);
        Vertex { colv_ref }
    }
    pub fn p3<'a>(&'a self) -> Vertex<'a> {
        let i = (self.pwidth + 1) * self.pheight;
        let colv_ref = self.locmat.get_col(i);
        Vertex { colv_ref }
    }
    pub fn p4<'a>(&'a self) -> Vertex<'a> {
        let i = self.locmat.colc() - 1;
        let colv_ref = self.locmat.get_col(i);
        Vertex { colv_ref }
    }
}

impl<'a> Vertex<'a> {
    pub fn vx(&self) -> f64 { self.colv_ref[0] }
    pub fn vy(&self) -> f64 { self.colv_ref[1] }
}

impl<'a> PointLike2D for Vertex<'a> {
    type T = f64;
    fn to_tuple(&self) -> (Self::T, Self::T) { (self.vx(), self.vy()) }   
}

impl<'a> TileColor<'a> {
    pub fn red(&self) -> u8 { self.colv_ref[0] }
    pub fn green(&self) -> u8 { self.colv_ref[1] }
    pub fn blue(&self) -> u8 { self.colv_ref[2] }
}

impl<'a> Tile<'a> {
    pub fn p1(&self) -> Vertex<'a> {
        let colv_ref = self.quilt.locmat.get_col(self.uli);
        return Vertex { colv_ref };
    }
    pub fn p2(&self) -> Vertex<'a> {
        let colv_ref = self.quilt.locmat.get_col(self.uli + 1);
        return Vertex { colv_ref };
    }
    pub fn p3(&self) -> Vertex<'a> {
        let colv_ref = self.quilt.locmat.get_col(self.uli + 
            self.quilt.pwidth + 1);
        return Vertex { colv_ref };
    }
    pub fn p4(&self) -> Vertex<'a> {
        let colv_ref = self.quilt.locmat.get_col(self.uli + 
            self.quilt.pwidth + 2);
        return Vertex { colv_ref };
    }
    pub fn color(&self) -> TileColor<'a> {
        let colv_ref = self.quilt.colmat.get_col(self.pi);
        return TileColor { colv_ref }
    }
}

pub struct TileIterator<'a> { tile: Tile<'a> }

impl<'a> Iterator for TileIterator<'a> {
    type Item = Tile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tile.py == self.tile.quilt.pheight {
            return None;
        }
        let elapsed = self.tile.clone();
        self.tile.px += 1;
        if self.tile.px >= self.tile.quilt.pwidth {
            self.tile.px = 0;
            self.tile.py += 1;
            self.tile.uli += 1;
        }       
        self.tile.pi += 1;
        self.tile.uli += 1;
        return Some(elapsed);
    }
}

