pub mod math_utils {


    // Create a grid obj, solver obj

    // Perform a forward FFT of size 1234
    use rustfft::{FftPlanner, num_complex::Complex};

    const _PI:f64 =   std::f64::consts::PI;


    // define a grid unit

    pub fn init_vec(vec: &mut Vec<f64>, width: f64, num_points: i64, f: &dyn Fn(f64) -> f64) {

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
    #[test]
    fn test_sin() {
        let width = 1.0;
        let vec_size = 32;
        let mut v: Vec<f64> = Vec::new();
        init_vec(&mut v,width,vec_size, &f64::sin);
        assert_eq!(v[0],0.0);
        let midway = (vec_size/4) as usize;
        

        for el in &v {
            print!("{} ", el);
       }        
       assert_eq!(v[midway],1.0);

    }
}
