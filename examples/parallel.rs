use interp1d::Interp1d;
use rayon::prelude::*;

fn main() {

    // Data (already sorted)
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let y: Vec<f64> = vec![5.0, 3.0, 4.0];

    // Using `new_sorted` since data is already sorted
    let interpolator = Interp1d::new_sorted(x, y).unwrap();

    // Points at which we wish to interpolate
    let x_interp = vec![1.5, 2.5];

    // Intepolate with checked fn, now using rayon's par_iter
    let y_interp: Vec<f64> = x_interp
        .par_iter()
        .map(|&x| interpolator.interpolate_checked(x))
        .collect::<Result<Vec<f64>, _>>()
        .unwrap(); // all points are in the domain in this example
    
    println!("y_interp = {y_interp:?}");
}