use crate::quilt::Quilt;
use crate::viewbox::ViewBox;
use crate::viewbox::fit_vb;

pub fn render_svg_autofit<W>(output: &mut W, quilt: &Quilt) -> std::io::Result<()>
where W: std::io::Write
{
    let vb = fit_vb(quilt);
    render_svg(output, quilt, &vb)
}

pub fn render_svg<W>(output: &mut W, quilt: &Quilt, vb: &ViewBox) -> std::io::Result<()>
where W: std::io::Write
{   
    write!(output, "<svg xmlns=\"http://www.w3.org/2000/svg\" ")?; 
    write!(output, "viewBox=\"{} {} {} {}\" ", vb.min_x, vb.min_y, vb.width, vb.height)?;

    // Fix the display width and display height equal to the viewbox width and height
    // so that the SVG behaves like a typical image. For instance, if we scale the image
    // 2x using a linear transformation, then the image should actually present as 2x
    // larger. In contrast, if we did not fix these values, then whichever program is displaying 
    // the SVG might choose to scale the image down/up based on its own whims.
    write!(output, "width=\"{}\" height=\"{}\" ", vb.width, vb.height)?;  
    write!(output, ">")?;

    for tile in quilt.tiles() {
        write!(output, "<polygon ")?;
        write!(output, "points=\"{},{} {},{} {},{} {},{}\" ",
            tile.p1().vx(), tile.p1().vy(), tile.p3().vx(), tile.p3().vy(),
            tile.p4().vx(), tile.p4().vy(), tile.p2().vx(), tile.p2().vy())?;
        write!(output, "fill=\"rgb({}, {}, {})\" ", 
            tile.color().red(), tile.color().green(), tile.color().blue())?;
        write!(output, "stroke=\"none\" ")?;
        write!(output, "/>")?;
    }
    write!(output, "</svg>")?;
    Ok(())
}
