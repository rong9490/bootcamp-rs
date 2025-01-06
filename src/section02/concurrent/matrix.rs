use anyhow::Result;
use std::fmt;

// 矩阵计算, SIMD

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug
        + Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::AddAssign<T>
        + std::ops::Mul<Output = T>,
{
    // 必须满足矩阵乘法规则
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }

    // 创建结果矩阵
    // let mut data = Vec::with_capacity(a.rows * b.cols);
    // 使用默认值初始化
    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                // 所有权问题(需要Copy)
                let a_value = a.data[i * a.cols + k];
                let b_value = b.data[k * b.cols + j];
                data[i * b.cols + j] += a_value * b_value;
            }
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.rows, self.cols, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        println!("{}", c);
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }

    // #[test]
    // fn test_matrix2() {
    //     let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
    //     let b = Matrix::new(&[1, 2, 3, 4, 5, 6], 3, 2);
    //     let c = matrix(&a, &b).unwrap();
    //     println!("{}", c);
    // }
}
