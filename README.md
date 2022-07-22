# interp1d: A simple, lightweight interpolation library written in Rust

This library is intended to be very simple and lightweight. The core `Interp1D` struct takes some set of `(x, y)` pairs and has a simple linear interpolation method. Unlike other libaries I found, this libary is unique in two cores ways:

1) It takes ownership over the `(x, y)` pairs and internally sorts them, using a binary search to find the two neighbors with which interpolation is done. This prevents the sorting/searching that other functions `fn(&[T], &[T], T) -> T` may do.
2) It allows for the types of `x` and `y` to differ. `x` could be an integer or a float and can be of a different type than `y` (`y` must still be a float). This allows for e.g. interpolation on a 1D lattice.


## Simple Usage

An example with double precision floats:
```rust
use interp1d::Interp1d;


fn main() {

    // Data (already sorted)
    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let y: Vec<f64> = vec![5.0, 3.0, 4.0];

    // Using `new_sorted` since data is already sorted
    let interpolator = Interp1d::new_sorted(x, y).unwrap();

    // Points at which we wish to interpolate
    let x_interp = vec![1.5, 2.5];

    // Intepolate with checked fn
    let y_interp: Vec<f64> = x_interp
        .iter()
        .map(|&x| interpolator.interpolate_checked(x))
        .collect::<Result<Vec<f64>, _>>()
        .unwrap(); // all points are in the domain in this example
    
    println!("y_interp = {y_interp:?}");
    // Output:
    // y_interp = [4.0, 3.5]
}
```
An example with `x` as usize:
```rust
use interp1d::Interp1d;


fn main() {

    // Data (already sorted)
    let x: Vec<usize> = vec![1, 3, 5];
    let y: Vec<f64> = vec![5.0, 3.0, 4.0];

    // Using `new_sorted` since data is already sorted
    let interpolator = Interp1d::new_sorted_int(x, y);

    // Points at which we wish to interpolate
    let x_interp = vec![2, 4];

    // Intepolate with checked fn
    let y_interp: Vec<f64> = x_interp
        .iter()
        .map(|&x| interpolator.interpolate_checked(x))
        .collect::<Result<Vec<f64>, _>>()
        .unwrap(); // all points are in the domain in this example
    
    println!("y_interp = {y_interp:?}");
    // Output:
    // y_interp = [4.0, 3.5]
}
```