extern crate blas_src;
extern crate ndarray;

use ndarray::{Array2};

#[test]
fn dot_blas_test_38() {
    let a : Array2<f64> = Array2::<f64>::ones((38, 38));
    let b : Array2<f64> = Array2::<f64>::ones((38, 38));
    let result : Array2<f64> = a.dot(&b);
    // println!("{:?}", result);
}

#[test]
fn dot_blas_test_100(){
    let a : Array2<f64> = Array2::<f64>::ones((100, 100));
    let b : Array2<f64> = Array2::<f64>::ones((100, 100));
    let result : Array2<f64> = a.dot(&b);
}
