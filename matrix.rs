/// A column-major heap-allocated matrix.
pub struct Matrix<T> { content: Vec<T>, rowc: usize }

impl<T> Matrix<T> where T: Clone {
    /// Allocates a matrix with `rowc` rows and `colc` columns, cloning `default`
    /// into each cell.
    pub fn new_uniform(rowc: usize, colc: usize, default: T) -> Self {
        Matrix {
            content: vec![default; rowc * colc],
            rowc
        }
    }   

    pub fn literal<const M: usize, const N: usize>(v: [[T; N]; M]) -> Self {
        let mut content: Vec<T> = Vec::with_capacity(M * N);
        for n in 0..N {
            for m in 0..M {
                content.push(v[m][n].clone());
            }
        }
        Self { content, rowc: M }
    }
}

impl<T> Matrix<T> where T: Clone + Default {
    pub fn new(rowc: usize, colc: usize) -> Self {
        Self::new_uniform(rowc, colc, T::default())
    }
}

impl<T> Matrix<T> {
    pub fn rowc(&self) -> usize { self.rowc }
    pub fn colc(&self) -> usize { self.content.len() / self.rowc }

    /// Transposes this matrix by allocating a new matrix,
    /// moving all values from this matrix into the new matrix,
    /// and discarding this matrix.
    pub fn transpose(mut self) -> Self {
        let (src_rowc, src_colc) = (self.rowc(), self.colc());
        let (dst_rowc, dst_colc) = (src_colc, src_rowc);

        let size = self.content.len();
        let mut content = Vec::with_capacity(size);
        let spare = content.spare_capacity_mut();
        
        for (i, value) in self.content.drain(..).enumerate() {
            let (src_row, src_col) = (i % src_rowc, i / src_rowc);
            let (dst_row, dst_col) = (src_col, src_row);
            let dst_i = (dst_col * dst_rowc) + dst_row;
            let dst_p = &mut spare[dst_i];
            dst_p.write(value);
        }

        unsafe { content.set_len(size); }
        Self { content, rowc: dst_rowc }
    }

    pub fn get_col(&self, i: usize) -> &[T] {
        assert!(i < self.colc());
        let begin = self.rowc() * i;
        let end = begin + self.rowc();
        &self.content[begin..end]
    }

    pub fn get_col_mut(&mut self, i: usize) -> &mut [T] {
        assert!(i < self.colc());
        let begin = self.rowc() * i;
        let end = begin + self.rowc();
        &mut self.content[begin..end]
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Matrix<U>
    {
        let content: Vec<U> = self.content.iter().map(f).collect();
        Matrix { content, rowc: self.rowc }
    }
}

impl<T> Clone for Matrix<T> where T: Clone {
    fn clone(&self) -> Self {
        let size = self.rowc() * self.colc();
        let mut content: Vec<T> = Vec::with_capacity(size);
        for i in 0..size {
            content.push(self.content[i].clone());
        }
        Matrix { content, rowc: self.rowc() }
    }
}

/// A type for which the dot product of two equal-length vectors is defined.
pub trait DotProduct = std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> + Default + Copy;

/// Computes the dot product of two equal-length vectors `a` and `b` and 
/// returns the result.
pub fn dot<T>(a: &[T], b: &[T]) -> T where T: DotProduct {
    assert!(a.len() == b.len());
    let mut dp = T::default();
    for i in 0..a.len() {
        dp = dp + a[i] * b[i]
    }
    return dp;
}


pub fn matmul<T>(left: &Matrix<T>, right: &Matrix<T>, output: &mut Matrix<T>)
where T: DotProduct
{
    assert!(left.colc() == right.rowc());
    assert!(output.rowc() == left.rowc());
    assert!(output.colc() == right.colc());  

    let left_t = left.clone().transpose(); 
    
    for i in 0..left.rowc() {
        for j in 0..right.colc() {
            let dp = dot(left_t.get_col(i), right.get_col(j)); 
            output.get_col_mut(j)[i] = dp;
        }
    }
}

pub fn matmul_replace<T>(left: &Matrix<T>, right: &mut Matrix<T>)
where T: DotProduct + Default
{
    assert!(left.rowc() == right.rowc());
    let mut intermediate: Matrix<T> = Matrix::new(left.rowc(), right.colc());
    matmul(left, right, &mut intermediate);
    *right = intermediate;
}
