pub trait PointLike2D {
    type T;
    fn to_tuple(&self) -> (Self::T, Self::T);
}

impl<A: Clone + Copy> PointLike2D for (A, A) {
    type T = A;
    fn to_tuple(&self) -> (Self::T, Self::T) { *self }
}

pub fn euclidean_distance_2d(p1: impl PointLike2D<T = f64>, 
    p2: impl PointLike2D<T = f64>) -> f64
{
    let (p1x, p1y) = p1.to_tuple();
    let (p2x, p2y) = p2.to_tuple();
    ((p1x - p2x).powi(2) + (p1y - p2y).powi(2)).sqrt()
}

pub fn vec_dif_2d(v1: impl PointLike2D<T = f64>, v2: impl PointLike2D<T = f64>) -> (f64, f64)
{
    let (v1x, v1y) = v1.to_tuple();
    let (v2x, v2y) = v2.to_tuple();
    return (v1x - v2x, v1y - v2y);
}

pub fn vec_sum_2d(v1: impl PointLike2D<T = f64>, v2: impl PointLike2D<T = f64>) -> (f64, f64)
{
    let (v1x, v1y) = v1.to_tuple();
    let (v2x, v2y) = v2.to_tuple();
    return (v1x + v2x, v1y + v2y);
}

pub fn vec_scale_2d(v1: impl PointLike2D<T = f64>, scalar: f64) -> (f64, f64)
{
    let (v1x, v1y) = v1.to_tuple();
    return (v1x * scalar, v1y * scalar);
    
}

pub fn unit_vec(v1: impl PointLike2D<T = f64>) -> (f64, f64)
{
    let distance = euclidean_distance_2d((0f64, 0f64), v1.to_tuple());
    let (v1x, v1y) = v1.to_tuple();
    return (
        v1x / distance,
        v1y / distance
    )
}
