#![feature(trait_alias)]
#![feature(seek_seek_relative)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

mod matrix;
mod bmp;
mod quilt;
mod svg;
mod error;
mod viewbox;
mod raster;
mod geom;
mod rasterize;

use bmp::write_bmp;
use matrix::Matrix;
use matrix::matmul_replace;
use quilt::Quilt;
use error::AnyError;
use viewbox::ViewBox;
use rasterize::rasterize_autofit_autoconfig;
use rasterize::rasterize_autoconfig;
use svg::render_svg_autofit;
use svg::render_svg;

// Entrypoint
// The textbook problem 4.15 in Coding the Matrix by Philip N. Klein
// has 6 parts. For each part, a different linear transformation must be
// applied to the input image. 
//
// I have written a procedure for each part and placed them all in this file. 
// To execute one of these procedures/parts, simply uncomment it in the main procedure
// below. I reccomend only uncommenting one procedure at a time, as all together they
// might take a while to complete on slower computers. Then on the command line,
// cd into this directory and type "cargo run". Cargo is the Rust buildtool,
// you can get it by installing the Rust toolchain. 

fn main() -> Result<(), AnyError> {
    part1_identity()?;       // outputs to part1_identity.svg/bmp
    part2_translating()?;    // outputs to part2_translating.svg/bmp
    part3_scaling()?;        // outputs to part3_scaling.svg/bmp
    part4_rotating()?;       // outputs to part4_rotating.svg/bmp
    part5_reflecting()?;     // outputs to part5_reflecting.svg/bmp
    part6_colortransform()?; // outputs to part6_colortransform.svg/bmp
    part7_stretching()?;     // outputs to part7_stretching.svg/bmp
    Ok(())
}



// Procedures/Parts
// The following procedures correspond to parts 1-6 of Lab 4.15 in Coding the Matrix
// by Philip N. Klein. To run one of these procedures simply uncomment its invocation in
// the main procedure above ^^^

fn part1_identity() -> Result<(), AnyError> {
    let t = Matrix::<f64>::literal([
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0]
    ]);

    let mut q = load_input_img()?;
    matmul_replace(&t, &mut q.locmat);

    write_output_img(&q, "part1_identity")?;
    Ok(())
}

fn part2_translating() -> Result<(), AnyError> {
    let xoffset: f64 = 250.0; // The horizontal offset to apply to each vertex
    let yoffset: f64 = 100.0; // The vertical offset to apply to each vertex
    
    let t = Matrix::<f64>::literal([
        [1.0, 0.0, xoffset],
        [0.0, 1.0, yoffset],
        [0.0, 0.0, 1.0]
    ]);

    let mut q = load_input_img()?;
    matmul_replace(&t, &mut q.locmat);

    // Fix the point (0.0, 0.0) as the physical origin so that the translation is apparent. 
    // This, as opposed to autofitting the viewbox, which would reverse the translation
    // by changing the coordinate of the physical origin to be equal to the coordinate of
    // the topleft-most vertex of the quilt.
    let vb = ViewBox { 
        min_x: 0.0, min_y: 0.0, 
        width: (q.pwidth + 1) as f64 + xoffset,
        height: (q.pheight + 1) as f64 + yoffset
    };
    svg::render_svg(&mut std::fs::File::create("transformed_images/part2_translating.svg")?, &q, &vb)?;
    write_bmp(&mut std::fs::File::create("transformed_images/part2_translating.bmp")?, 
        &rasterize_autoconfig(&q, &vb))?;

    Ok(())
}

fn part3_scaling() -> Result<(), AnyError> {
    let xscale: f64 = 3.0;
    let yscale: f64 = 1.0;

    let t = Matrix::<f64>::literal([
        [xscale, 0.0, 0.0],
        [0.0, yscale, 0.0],
        [0.0, 0.0, 1.0]
    ]);

    let mut q = load_input_img()?;
    matmul_replace(&t, &mut q.locmat);
    write_output_img(&q, "part3_scaling")?;
    Ok(())    
}

fn part4_rotating() -> Result<(), AnyError> {
    let theta: f64 = 0.25 * std::f64::consts::PI;

    let t = Matrix::<f64>::literal([
       [theta.cos(), -1.0 * theta.sin(), 0.0],
       [theta.sin(), theta.cos(), 0.0],
       [0.0, 0.0, 1.0]  
    ]);

    let mut q = load_input_img()?;
    matmul_replace(&t, &mut q.locmat);
    write_output_img(&q, "part4_rotating")?;
    Ok(())
}

fn part5_reflecting() -> Result<(), AnyError> {
    // // reflects across the y-axis (negates x value)
    // let reflect_y = Matrix::<f64>::literal([
    //     [-1.0, 0.0, 0.0],
    //     [0.0, 1.0, 0.0],
    //     [0.0, 0.0, 1.0]
    // ]);

    // reflects across x-axis (negates y value)
    let reflect_x = Matrix::<f64>::literal([
        [1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0] 
    ]);

    let mut q = load_input_img()?;
    matmul_replace(&reflect_x, &mut q.locmat);
    write_output_img(&q, "part5_reflecting")?;
    Ok(())
}

fn part6_colortransform() -> Result<(), AnyError> {
    let mut q = load_input_img()?;

    // Colors are 8-bit unsigned integers but we'll need signed
    // integers to do the computation. 
    let mut im_mat: Matrix<i16> = q.colmat.map(|byte| i16::from(*byte));
    
    let negative_coloring = Matrix::<i16>::literal([
        [-1, 0, 0, 255],
        [0, -1, 0, 255],
        [0, 0, -1, 255],
        [0, 0, 0, 1]
    ]);

    matmul_replace(&negative_coloring, &mut im_mat);
    q.colmat = im_mat.map(|i| u8::try_from(*i).unwrap());
    write_output_img(&q, "part6_colortransform")?;
    Ok(())
}

fn part7_stretching() -> Result<(), AnyError> {
    let mut q = load_input_img()?;

    let t: Matrix<f64> = Matrix::literal([
        [1.0, 0.0, 0.0],
        [-1.0, 1.0, 0.0],
        [0.0, 0.0, 1.0]
    ]);

    matmul_replace(&t, &mut q.locmat);
    write_output_img(&q, "part7_stretching")?;
    Ok(())
}


fn load_input_img() -> Result<Quilt, AnyError> {
    let mut file = std::fs::File::open("input.bmp")?;
    let image = bmp::read_bmp(&mut file)?;
    let q = quilt::knit(image);
    Ok(q)
}

fn write_output_img(quilt: &Quilt, name: &str) -> Result<(), std::io::Error> {
    // SVG
    {
        let file_name = format!("transformed_images/{}.svg", name);
        let mut file = std::fs::File::create(file_name)?;
        render_svg_autofit(&mut file, &quilt)?;
    }  
    // BMP
    {
        let file_name = format!("transformed_images/{}.bmp", name);
        let mut file = std::fs::File::create(file_name)?;
        write_bmp(&mut file, &rasterize_autofit_autoconfig(&quilt))?;
    }
    Ok(())
} 
