###[0.2.1]
- Added repo to Cargo.toml

###[0.2.0]
- Changed the trait for `y` to allow for a `Complex` variable to be interpolated. This change introduces a breaking change, and introduces a new function `interpolate_checked_converted` which runs a fallible conversion from `x: T` to `y: U` in addition to the bounds check. This is what should be used for datasets with e.g. `x: usize`, `y: f64`. 