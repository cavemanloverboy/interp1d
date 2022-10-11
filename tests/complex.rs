
use interp1d::Interp1d;
use num_complex::Complex;

#[test]
fn test_interp_complex_y() {

    // Data (already sorted)
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let y: Vec<Complex<f64>> = vec![
        Complex::new(5.0, 4.0),
        Complex::new(3.0, 3.0),
        Complex::new(4.0, 5.0),
    ];

    // Using `new_sorted` since data is already sorted
    let interpolator = Interp1d::new_sorted(x, y).unwrap();

    // Points at which we wish to interpolate
    let x_interp = vec![1.5, 2.5];

    // Intepolate with checked fn
    let y_interp: Vec<Complex<f64>> = x_interp
        .iter()
        .map(|&x| interpolator.interpolate_checked(x))
        .collect::<Result<Vec<Complex<f64>>, _>>()
        .unwrap(); // all points are in the domain in this example
    
    assert_eq!(
        y_interp,
        vec![
            Complex::new(4.0, 3.5),
            Complex::new(3.5, 4.0),
        ],
    );
}