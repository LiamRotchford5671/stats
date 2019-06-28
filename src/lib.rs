// Copyright © 2019 Liam Rotchford
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    let sum = nums.iter().sum();
    // array iter() trait method sum:  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count

    let counter = nums.len() as f64;

    let result = if sum == 0.0 { 0 } else { 1 }; //conditional assignment for match since it cant match on floats

    match result {
        0 => Some(0.0),
        _ => Some(sum / counter),
    }
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    //algorithm found here: https://www.mathsisfun.com/data/standard-deviation-formulas.html

    let meanvalue = mean(nums); //determine mean
    let count = nums.len() as f64; //determine array lenght

    let mut sum = 0.0;
    for j in nums {
        //Subtract the mean from each value and square result
        sum += (j - meanvalue.unwrap()).powf(2.0); //sum all of those values together,
    }

    sum = (sum / count).sqrt(); //calc Variance and then square it for stand. dev.

    let result = if count == 0.0 { 0 } else { 1 }; //if count is 0.0 we have a empty list
    match result {
        //match can't pattern match on floats
        0 => None,
        1 => Some(sum),
        _ => None,
    }
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.25), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut index = nums.len();

    if index != 0 {
        if index % 2 != 0 {
            //odd length
            index = (index - 1) / 2; //determine median index
            Some(nums[index])
        } else {
            //even length
            index = index / 2;
            Some((nums[index] + nums[index - 1]) / 2.0) //determine average of the 2 mid values
        }
    } else {
        None
    }
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    unimplemented!()
}
