use interp1d::{Interp1d, pair::InterpNum};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_traits::Float;


fn create_unsorted_float_double(n: usize) -> Interp1d<f64, f64> {

    let x: Vec<f64> = vec![0.0; n].into_iter().map(|_| rand::random::<f64>()).collect();
    let y: Vec<f64> = vec![0.0; n].into_iter().map(|_| rand::random::<f64>()).collect();

    let interp = Interp1d::new_unsorted(x, y).unwrap();

    interp
}

fn create_unsorted_interp_points_double(n: usize) -> Vec<f64> {

    let x: Vec<f64> = vec![0.0; n].into_iter().map(|_| rand::random::<f64>()).collect();

    x
}

fn create_unsorted_float_single(n: usize) -> Interp1d<f32, f32> {

    let x: Vec<f32> = vec![0.0; n].into_iter().map(|_| rand::random::<f32>()).collect();
    let y: Vec<f32> = vec![0.0; n].into_iter().map(|_| rand::random::<f32>()).collect();

    let interp = Interp1d::new_unsorted(x, y).unwrap();

    interp
}

fn create_unsorted_interp_points_single(n: usize) -> Vec<f32> {

    let x: Vec<f32> = vec![0.0; n].into_iter().map(|_| rand::random::<f32>()).collect();

    x
}

fn criterion_benchmark(c: &mut Criterion) {

    for size in [1_000, 10_000, 100_000, 1_000_000] {

        let interp_double = create_unsorted_float_double(size);
        let interp_single = create_unsorted_float_single(size);

        let interp_double_ref = &interp_double;
        let interp_single_ref = &interp_single;

        
        c.bench_function(format!("checked_interp_double_{size}").as_str(), |b| b.iter(|| {
            let interp_data = create_unsorted_interp_points_double(black_box(100_000));
            for x in interp_data {
                let _ = interp_double_ref.interpolate_checked(x);
            }
        }));
        c.bench_function(format!("checked_interp_single_{size}").as_str(), |b| b.iter(|| {
            let interp_data = create_unsorted_interp_points_single(black_box(100_000));
            for x in interp_data {
                let _ = interp_single_ref.interpolate_checked(x);
            }
        }));

        c.bench_function(format!("unchecked_interp_double_{size}").as_str(), |b| b.iter(|| {
            let interp_data = create_unsorted_interp_points_double(black_box(100_000));
            for x in interp_data {
                let _ = interp_double_ref.interpolate(x);
            }
        }));
        c.bench_function(format!("unchecked_interp_single_{size}").as_str(), |b| b.iter(|| {
            let interp_data = create_unsorted_interp_points_single(black_box(100_000));
            for x in interp_data {
                let _ = interp_single_ref.interpolate(x);
            }
        }));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);