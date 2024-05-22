use crate::geom::PointLike2D;
use crate::quilt::Quilt;
use crate::quilt::Tile;
use crate::quilt::TileColor;
use crate::raster::Pixel;
use crate::raster::Raster;
use crate::viewbox::ViewBox;
use crate::viewbox::fit_vb;
use crate::geom::euclidean_distance_2d;
use crate::geom::vec_dif_2d;
use crate::geom::unit_vec;
use crate::geom::vec_sum_2d;
use crate::geom::vec_scale_2d;

pub fn rasterize_autofit_autoconfig(quilt: &Quilt) -> Raster {
    let vb = fit_vb(quilt);
    rasterize_autoconfig(&quilt, &vb)
 }

pub fn rasterize_autoconfig(quilt: &Quilt, vb: &ViewBox) -> Raster {
   rasterize(quilt, vb, Pixel::black(), 1f64, 1f64)
}

/// Rasterizes the segment of `quilt` specified by the [`ViewBox`] `vb`. Any space in the
/// viewbox not intersecting the quilt will be filled with the color `bg_color.` 
/// The resultant [`Raster`] will have an aspect ratio equivalent to that of `vb`.
///
/// The [`Raster`] will be `pixel_density * vb.width` pixels wide and `pixel_density * vb.height`
/// pixels tall. Put directly, `pixel_density` is the number of pixels per unit distance. 
pub fn rasterize(quilt: &Quilt, vb: &ViewBox, bg_color: Pixel, scan_px: f64, pixel_density: f64) -> Raster {
    let mut raster = Raster::solid(
        /* color  = */ bg_color, 
        /* width  = */ (vb.width * pixel_density).ceil() as usize,
        /* height = */ (vb.height * pixel_density).ceil() as usize
    );
    
    for tile in quilt.tiles() {
        rasterize_tile(&mut raster, vb, tile, scan_px, pixel_density);
    }

    return raster;
}

fn rasterize_tile<'a>(raster: &mut Raster, vb: &ViewBox, tile: Tile<'a>, scan_px: f64, pixel_density: f64) {
    let v1 = unit_vec(vec_dif_2d(tile.p2(), tile.p1()));
    let v2 = unit_vec(vec_dif_2d(tile.p3(), tile.p1()));

    // The position of `p1` relative to the origin of the viewbox.
    let p1_rel = vec_dif_2d(tile.p1(), (vb.min_x, vb.min_y));

    let d1_dist = euclidean_distance_2d(tile.p1(), tile.p2());
    let d2_dist = euclidean_distance_2d(tile.p1(), tile.p3());

    // The scan distance expressed not in pixels but instead in coordinate distance.
    let scan = scan_px / pixel_density;

    let mut pos_d1 = 0f64;
    let mut pos_d2 = 0f64;

    while pos_d1 < d1_dist {
        while pos_d2 < d2_dist {
            let (x, y) = vec_sum_2d(
                p1_rel,
                vec_sum_2d(
                    vec_scale_2d(v1, pos_d1), 
                    vec_scale_2d(v2, pos_d2)
                )
            ).to_tuple();

            if x >= 0f64 && x < vb.width && y >= 0f64 && y < vb.height {
                let x = (x * pixel_density) as usize;
                let y = (y * pixel_density) as usize;
                raster.set_pixel(x, y, Pixel::from(tile.color()));
            }
                      
            pos_d2 += scan;
        }
        pos_d2 = 0f64;
        pos_d1 += scan;
    }
}

impl<'a> From<TileColor<'a>> for Pixel {
    fn from(value: TileColor<'a>) -> Self {
        Pixel {
            red: value.red(),
            green: value.green(),
            blue: value.blue()
        }
    }
}
