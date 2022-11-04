// Module name: analysis.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains a set of functionalities
// that the user can exploit to size and evaluate fixed-point
// algorithms written using the Ffix type.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license


// This will be a struct which accepts a vector of immutable references to the
// Ffix variables used in the algorithm. After each iteration of the
// algorithm it will be possible to call an update() method that updates the
// max and min values

use crate::types::Ffix;


struct RangeAnalyzer {
    variables: Vec<Ffix>, //actually this should be a vector of &Ffix but I will need lifetimes for this
}

impl RangeAnalyzer {

    fn update_ranges() {

    }
}