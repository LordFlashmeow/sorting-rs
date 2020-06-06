//! This crate provides an implementation of various quality sorting methods.
//! Only the most useless or inefficient sorting algorithms are implemented.
//! You may use them in your production application, altough I would strongly
//! advise against that. Currently, the following sorting algorithms are implemented:
//!
//! ## Panicsort
//!
//! This sorting method kind of follows the principle of _check and surrender_
//! and simply panics when the array or vector is not sorted:
//!
//! ```should_panic
//! # use sorting::*;
//! let unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.panicsort();   // will panic
//! ```
//!
//! ## Slowsort
//!
//! This sorting algorithm recursively sorts the input array by finding the maximum
//! of the sorted array, placing that maximum at the end and sorting the remaining
//! array.
//!
//! ```
//! # use sorting::*;
//! let mut unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.slowsort();
//! ```
//!
//! ## Bogosort
//!
//! This highly inefficient algorithm scrambles the input vector until it is
//! sorted. Depending on your luck and the length of the input vector this might
//! never return.
//!
//! ```
//! # use sorting::*;
//! let mut unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.bogosort();    // might take a while...
//! ```
//!
//! ## Sleepsort
//!
//! This algorithm uses the operating system's scheduler for sorting by putting every
//! value into its own thread and putting that thread to sleep for a time determined
//! by the value.
//!
//! ```ignore
//! # // This takes over two minutes to test, so we exclude it from normal test runs
//! # use sorting::*;
//! let unsorted = vec![5i8, -7, 8, 2, 1, 0, -9];
//! let sorted: Vec<_> = unsorted.sleepsort().collect();
//! ```
//!
//! ## Stalinsort
//!
//! Stalinsort is a very fast sorting algorithm that removes out of order items.
//! Has time and space complexity of O(n),
//! but you may not recognize the list afterwards.
//!
//! ```
//! # use sorting::*;
//! let mut unsorted = vec![5,7,8,1,0]
//! unsorted.stalinsort();
//! ```
//!
//! ## Miraclesort
//!
//! A sorting algorithm that waits for a miracle that automatically makes your vector
//! sorted. Does nothing on its own.
//!
//! ```ignore
//! # //don't run tests for this. Miracles don't happen all the time!
//! # use sorting::*;
//! let unsorted = vec![5i8, -7, 8, 2, 1, 0, -9];
//! let sorted: Vec<_> = unsorted.miraclesort();
//! ```
#![warn(missing_docs)]

mod slowsort;
mod bogosort;
mod panicsort;
mod sleepsort;
mod miraclesort;
mod stalinsort;

pub use crate::slowsort::*;
pub use crate::bogosort::*;
pub use crate::panicsort::*;
pub use crate::sleepsort::*;
pub use crate::miraclesort::*;
pub use crate::stalinsort::*;
