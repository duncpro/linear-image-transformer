# Tuesday April 23 2024
I came up with transformation matrices for (1) identity function, (2) translation,
(3) scaling, (4) rotation, (5) reflection, (6) negative color transform,
and (7) stretching.

When calculating the dimensions of a quilt I now only check the corner points as opposed
to checking all the points. 

# Monday April 22 2024
This morning I achieved my first linear transformation of an image using the program.
Specifically I reflected the image across the x and the y axis, and also stretched the image
so that it is no longer a rectangle but instead a parallelogram.

Currently the program renders the transformed image into SVG and not to BMP.
I'm not exactly happy with this as I'd like my program to be able to sample the quilt
itself so that it can render the transformed image to BMP. However I don't think I'll have time
to achieve this as exams are coming up. I think I'll just focus on finishing the
transformation matrices.

I'd also like to implement compositing so that I can combine two images,
however I doubt I'll have time for that either.

The way I calculate the dimensions of a quilt is pretty ineffecient as it involves
iterating over all the vertices and taking the minimum and maximum. The corners
of the image have fixed positions in the location matrix so I should be able to
limit my calculation to 4 points instead of all the points.

# Sunday April 21 2024
The textbook problem represents images using the Portable Network Graphics (PNG) format.
For the sake of simplicity, I have chosen to use the Bitmap format (BMP) instead.

PNG format allows for compression via DEFLATE and also supports animations. 
These features make images in the format more difficult to encode/decode. 
In contrast BMP is simply a header followed by the uncompressed pixel data. 
By choosing BMP I can focus my attention on the linear algebra aspect of this 
problem and not miscellaneous details such as format.

Like PNG, BMP is supported natively by both macOS and Microsoft Windows.
On macOS, BMP images can be opened using the builtin Preview application.
On macOS, the builtin `sips` command-line application can convert more typical
formats like JPEG into BMP images. For example: `sips -s format bmp input.jpg 
--out output.bmp.`

The textbook uses Python, however I have chosen Rust as it is statically-typed, 
and I prefer working with statically-typed languages.

