use crate::quilt::Quilt;
use crate::geom::PointLike2D;

pub struct ViewBox { pub min_x: f64, pub min_y: f64, pub width: f64, pub height: f64 }

pub fn fit_vb(quilt: &Quilt) -> ViewBox {
    let corners = [quilt.p1(), quilt.p2(), quilt.p3(), quilt.p4()];
    
    let min_x = corners.iter().map(|vert| vert.vx())
        .reduce(|acc, e| acc.min(e)).unwrap_or(0.0);

    let max_x = corners.iter().map(|vert| vert.vx())
        .reduce(|acc, e| acc.max(e)).unwrap_or(0.0);

    let min_y = corners.iter().map(|vert| vert.vy())
        .reduce(|acc, e| acc.min(e)).unwrap_or(0.0);

    let max_y = corners.iter().map(|vert| vert.vy())
        .reduce(|acc, e| acc.max(e)).unwrap_or(0.0);
    
    let width = max_x - min_x;
    let height = max_y - min_y;
    
    ViewBox { min_x, min_y, width, height }
}
