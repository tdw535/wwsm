

pub mod math_utils {

    use rustfft::{FftPlanner, num_complex::Complex};
    use rustfft::Fft;
    use std::sync::Arc;
    // Create a grid obj, solver obj

    // Perform a forward FFT of size 1234

    const _PI:f64 =   std::f64::consts::PI;



    // define a grid unit

    // Generalize to more dimensions?
    pub struct FFTManager {
        num_points: usize,
        fft_planner: FftPlanner<f64>,
        plan_forward:  Arc<dyn Fft<f64>>,
        // buffer: vec![Complex{re: 0.0f32, im: 0.0f32}; 1234];
    }

    impl FFTManager {
        pub fn new(num_points: usize) -> FFTManager {
            let mut fft_planner = FftPlanner::new();
            let plan_forward = fft_planner.plan_fft_forward(num_points);
            FFTManager{num_points, fft_planner, plan_forward}
        }
        pub fn fft_forward(&self,buffer: &mut Vec<Complex<f64>>) -> bool {
            self.plan_forward.process(buffer);
            true
        }
    }

    pub fn init_vec(vec: &mut Vec<f64>, width: f64, num_points: usize, f: &dyn Fn(f64) -> f64) {

        let delta:f64 = (2.0*_PI)/(width*((num_points) as f64));
        for ii in 0..num_points {
            let grid_value = f(delta* (ii as f64));
            vec.push(grid_value);
        }
    }



    // fn main() {

    //     println!("Hello, world!");

    //     let width = 1.0;
    //     let vec_size = 128;
    //     let mut v: Vec<f64> = Vec::new();
    //     init_vec(&mut v,width,vec_size);

    //     let mut planner = FftPlanner::new();
    //     let fft = planner.plan_fft_forward(1234);

    //     let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
    //     fft.process(&mut buffer);
    // }
}

#[cfg(test)]
mod tests {
    use super::math_utils::*;
    use rustfft::{num_complex::Complex};
    #[test]
    fn test_sin() {
        let width = 1.0;
        let vec_size: usize = 32;
        let mut v: Vec<Complex<f64>> = Vec::new();
        init_vec(&mut v,width,vec_size, &f64::sin);
        assert_eq!(v[0].re,0.0);
        let midway = (vec_size/4) as usize;
        

        for el in &v {
            print!("{} ", el);
       }        
       assert_eq!(v[midway].re,1.0);

       let fft_manager = FFTManager::new(vec_size);
       fft_manager.fft_forward(&mut v);
    }
}
