use num_traits::{Float, PrimInt};

pub mod pair;
pub mod error;
use pair::{Pair, InterpNum};
use error::{Result, InterpError};



pub struct Interp1d<T: InterpNum, U: Float> {
    /// This vec must be sorted!
    inner: Vec<Pair<T, U>>,
    size: usize,
    min: T,
    max: T,
}

impl<T: InterpNum, U: Float> Interp1d<T, U> {

    /// This creates a new interpolator from unsorted floats
    pub fn new_unsorted<I: Float + InterpNum>(x: Vec<I>, y: Vec<U>) -> Result<Interp1d<T, U>>
    where
        Vec<Pair<T, U>>: FromIterator<Pair<I, U>>,
    {
        // For floats, check if there are any nans or infs
        if !x.iter().all(|val| val.is_finite()) {
            return Err(InterpError::InvalidData)
        }

        // Zip into pairs
        let mut inner: Vec<Pair<T, U>> = x
            .into_iter()
            .zip(y.into_iter())
            .map(|x| Pair::from_float(x))
            .collect::<Result<Vec<Pair<T, U>>>>()?;
        
        // Pair impls 
        inner.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());

        // Size of dataset
        let size = inner.len();

        // Domain
        let min = inner.first().unwrap().x;
        let max = inner.last().unwrap().x;

        Ok(Interp1d {
            inner,
            size,
            min,
            max,
        })
    }

    /// This creates a new interpolator from data already sorted
    pub fn new_sorted<I: Float + InterpNum>(x: Vec<I>, y: Vec<U>) -> Result<Interp1d<T, U>>
    where
        Vec<Pair<T, U>>: FromIterator<Pair<I, U>>,
    {
        // For floats, check if there are any nans or infs
        if !x.iter().all(|val| val.is_finite()) {
            return Err(InterpError::InvalidData)
        }

        // Zip into pairs
        let inner: Vec<Pair<T, U>> = x
            .into_iter()
            .zip(y.into_iter())
            .map(|x| Pair::from_float(x))
            .collect::<Result<Vec<Pair<T, U>>>>()?;

        // Size of dataset
        let size = inner.len();

        // Domain
        let min = inner.first().unwrap().x;
        let max = inner.last().unwrap().x;

        Ok(Interp1d {
            inner,
            size,
            min,
            max,
        })
    }

    /// This creates a new interpolator from unsorted ints. 
    /// Don't need to check for inf or nan when using ints.
    pub fn new_unsorted_int<I: PrimInt + InterpNum + Ord>(x: Vec<I>, y: Vec<U>) -> Interp1d<T, U>
    where
        Vec<Pair<T, U>>: FromIterator<Pair<I, U>>
    {

        // Zip into pairs
        let mut inner: Vec<Pair<I, U>> = x
            .into_iter()
            .zip(y.into_iter())
            .map(|x| Pair::from_int(x))
            .collect();
        
        // Pair impls 
        inner.sort_by(|p1, p2| p1.x.cmp(&p2.x));
        let inner: Vec<Pair<T, U>> = inner.into_iter().collect();

        // Size of dataset
        let size = inner.len();

        // Domain
        let min = inner.first().unwrap().x;
        let max = inner.last().unwrap().x;

        Interp1d {
            inner,
            size,
            min,
            max,
        }
    }

    /// This creates a new interpolator from sorted ints. 
    /// Don't need to check for inf or nan when using ints.
    pub fn new_sorted_int<I: PrimInt + InterpNum + Ord>(x: Vec<I>, y: Vec<U>) -> Interp1d<T, U>
    where
        Vec<Pair<T, U>>: FromIterator<Pair<I, U>>
    {

        // Zip into pairs
        let inner: Vec<Pair<T, U>> = x
            .into_iter()
            .zip(y.into_iter())
            .map(|x| Pair::from_int(x))
            .collect();

        // Size of dataset
        let size = inner.len();

        // Domain
        let min = inner.first().unwrap().x;
        let max = inner.last().unwrap().x;

        Interp1d {
            inner: inner.into_iter().collect(),
            size,
            min,
            max,
        }
    }

    /// Interpolation for a single point. This is checked for whether the point is out of bounds,
    /// returning an error with either [InterpError::OutOfRangeLeft] or [InterpError::OutOfRangeLeft]
    /// if the point is out of the domain.
    pub fn interpolate_checked(&self, x: T) -> Result<U> {

        match self.inner.binary_search_by(|a| a.x.partial_cmp(&x).expect("When creating the inner, the values were checked")) {


            Ok(index) => {

                // x is in the data already, so return value in inner
                Ok(self.inner[index].y)
            },


            Err(index) => {

                // x is not in the data already, so interpolate if possible
                match self.determine_region(index) {

                    Region::Left => { 

                        // Out of range, left
                        let point = format!("{x}");
                        let min = format!("{}", self.min);
                        Err(InterpError::OutOfRangeLeft {point, min})
                    },

                    Region::Inside => {
                        
                        let left_pair = self.inner[index-1];
                        let right_pair = self.inner[index];

                        let interp_value = left_pair.y + (right_pair.y - left_pair.y)/U::from(right_pair.x - left_pair.x).unwrap() * U::from(x - left_pair.x).unwrap();
                        Ok(interp_value)
                    },

                    Region::Right => {

                        // Out of range, right
                        let point = format!("{x}");
                        let max = format!("{}", self.max);
                        Err(InterpError::OutOfRangeRight {point, max})
                    }
                }
            }
        }
    }

    /// Interpolation for a single point. If a point is out of the domain, the closest value
    /// (i.e. the edge value) is returned.
    pub fn interpolate(&self, x: T) -> U {

        match self.inner.binary_search_by(|a| a.x.partial_cmp(&x).expect("When creating the inner, the values were checked")) {


            Ok(index) => {

                // x is in the data already, so return value in inner
                self.inner[index].y
            },


            Err(index) => {

                // x is not in the data already, so interpolate if possible
                match self.determine_region(index) {

                    Region::Left => { 

                        // Out of range, left
                        self.inner.first().unwrap().y
                    },

                    Region::Inside => {
                        
                        let left_pair = self.inner[index-1];
                        let right_pair = self.inner[index];

                        let interp_value = left_pair.y + (right_pair.y - left_pair.y)/U::from(right_pair.x - left_pair.x).unwrap() * U::from(x - left_pair.x).unwrap();
                        interp_value
                    },

                    Region::Right => {

                        // Out of range, right
                        self.inner.last().unwrap().y
                    }
                }
            }
        }
    }

    fn determine_region(&self, index: usize) -> Region {

        if index == 0 { Region:: Left }
        else if index == self.size { Region::Right }
        else if index > 0 && index < self.size { Region::Inside }
        else { panic!("Something very odd happened.")}

    }
}

enum Region {
    Left,
    Inside,
    Right
}


#[test]
fn test_create_unsorted_float() {

    let x: Vec<f64> = vec![2.0, 1.0, 3.0];
    let y: Vec<f64> = vec![1.0, 0.0, 2.0];

    let interp = Interp1d::new_unsorted(x, y).unwrap();

    let expected: Vec<Pair<f64, f64>> = vec![
        Pair::from_float((1.0, 0.0)).unwrap(),
        Pair::from_float((2.0, 1.0)).unwrap(),
        Pair::from_float((3.0, 2.0)).unwrap(),
    ];

    assert_eq!(
        interp.inner,
        expected,
    )
}

#[test]
fn test_create_unsorted_int() {

    let x: Vec<u32> = vec![2, 1, 3];
    let y: Vec<f64> = vec![1.0, 0.0, 2.0];

    let expected: Vec<Pair<u32, f64>> = vec![
        Pair::from_int((1, 0.0)),
        Pair::from_int((2, 1.0)),
        Pair::from_int((3, 2.0)),
    ];

    let interp = Interp1d::new_unsorted_int(x, y);

    assert_eq!(
        interp.inner,
        expected,
    )
}

#[test]
fn test_create_sorted_float() {

    let x: Vec<f64> = vec![1.0, 2.0, 3.0];
    let y: Vec<f64> = vec![1.0, 0.0, 2.0];

    let interp = Interp1d::new_sorted(x, y).unwrap();

    let expected: Vec<Pair<f64, f64>> = vec![
        Pair::from_float((1.0, 1.0)).unwrap(),
        Pair::from_float((2.0, 0.0)).unwrap(),
        Pair::from_float((3.0, 2.0)).unwrap(),
    ];

    assert_eq!(
        interp.inner,
        expected,
    )
}

#[test]
fn test_create_sorted_int() {

    let x: Vec<u32> = vec![1, 2, 3];
    let y: Vec<f64> = vec![1.0, 0.0, 2.0];

    let expected: Vec<Pair<u32, f64>> = vec![
        Pair::from_int((1, 1.0)),
        Pair::from_int((2, 0.0)),
        Pair::from_int((3, 2.0)),
    ];

    let interp = Interp1d::new_sorted_int(x, y);

    assert_eq!(
        interp.inner,
        expected,
    )
}
