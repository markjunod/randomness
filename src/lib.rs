// Copyright (c) 2020 Mark Junod. Subject to the MIT License.

//! Easy access to randomness.
//! 
//! Supplies an intuitive way to generate random primitive values.
//! 
//! To use:
//! 
//! ```
//! use randomness::prelude::*;
//! 
//! let mut rand = new_random();
//! 
//! if rand.next_bool() {
//!     println!("The next float is: {}", rand.next_f32());
//! }
//! ```

// TODO figure out top level structure and convenience methods
pub mod prelude {
    pub use crate::*;
}

mod random;

use random::Random;

pub fn new_random() -> Random {
    Random::new()
}

pub fn new_random_from_seed(seed: u64) -> Random {
    Random::from_seed(seed)
}
