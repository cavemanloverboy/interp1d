use interp1d::Interp1d;


#[test]
fn test_integer() {

    // Data (already sorted)
    let x: Vec<usize> = vec![1, 3, 5];
    let y: Vec<f64> = vec![5.0, 3.0, 4.0];

    // Using `new_sorted` since data is already sorted
    let interpolator = Interp1d::new_sorted_int(x, y);

    // Points at which we wish to interpolate
    let x_interp = vec![2, 4];

    // Intepolate
    let y_interp: Vec<f64> = x_interp
        .iter()
        .map(|&x| interpolator.interpolate_checked_converted(x))
        .collect::<Result<Vec<f64>, _>>()
        .unwrap(); // all points are in the domain in this example
    
    assert_eq!(y_interp, vec![4.0, 3.5]);
}