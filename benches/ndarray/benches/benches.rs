#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(test)] 

extern crate test;
extern crate ndarray;

use ndarray::{Array2};

#[cfg(test)]
mod bencher {
    use super::*;
    use test::{Bencher};

    #[bench]
    fn dot_test_38(b: &mut Bencher) {
        b.iter(|| {
            let a : Array2<f64> = Array2::<f64>::ones((38, 38));
            let b : Array2<f64> = Array2::<f64>::ones((38, 38));
            let result : Array2<f64> = a.dot(&b);
            //println!("Dot Test: {:?}", result);
            //assert_eq!(result, result);
        });
    }

    #[bench]
    fn dot_test_100(b: &mut Bencher) {
        b.iter(|| {
            let a : Array2<f64> = Array2::<f64>::ones((100, 100));
            let b : Array2<f64> = Array2::<f64>::ones((100, 100));
            let result : Array2<f64> = a.dot(&b);
            //println!("Dot Test: {:?}", result);
            //assert_eq!(result, result);
        });
    }
}