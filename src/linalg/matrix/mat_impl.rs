
// std imports

// local imports
use algebra::{Number, Signed};
use super::mat_traits::LANumberMatrix;
use linalg;
use matrix::matrix::Matrix;
use error::SRError;

impl<T:Number+Signed> LANumberMatrix<T> for Matrix<T>{
    /// Returns determinant of the matrix
    fn det(&self) -> Result<T,SRError>{
        linalg::det::det(self)
    }
}




/******************************************************
 *
 *   Unit tests
 *
 *******************************************************/


#[cfg(test)]
mod test{
    //use super::*;
}


/******************************************************
 *
 *   Bench marks
 *
 *******************************************************/


#[cfg(test)]
mod bench{
    //extern crate test;
    //use self::test::Bencher;
    //use super::*;
}


