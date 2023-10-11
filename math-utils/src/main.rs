pub mod math_utils {

// Perform a forward FFT of size 1234
use rustfft::{FftPlanner, num_complex::Complex};

const _PI:f64 =   std::f64::consts::PI;


// define a grid unit

fn init_vec(vec: &mut Vec<f64>, width: f64, num_points: i64) {

    let delta:f64 = (2.0*_PI)/(width*(num_points as f64));
    for ii in 0..num_points {
        let grid_value = f64::sin(delta* (ii as f64));
        vec.push(grid_value);
    }
}


fn main() {

    println!("Hello, world!");

    let width = 1.0;
    let vec_size = 128;
    let mut v: Vec<f64> = Vec::new();
    init_vec(&mut v,width,vec_size);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1234);

    let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
    fft.process(&mut buffer);
}
}

#[cfg(test)]
mod tests {
    use super::math_utils::*;
    #[test]
    fn test_vec() {
        let width = 1.0;
        let vec_size = 128;
        let mut v: Vec<f64> = Vec::new();
        init_vec(&mut v,width,vec_size);
        assert_eq!(v[0],0.0);
    }
}
