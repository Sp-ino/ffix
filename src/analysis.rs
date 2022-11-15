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

// The analyzer might work as follows:
// I add an argument to the new() associated function of Ffix.
// This argument allows to pass a reference to an instance of the
// analyzer. 
// When a Ffix variable is created, a reference to this variable is registered
// in the analyzer instance, if one is passed, and then the analyzer instance
// can update the max and min values when the user calls update()

use crate::types::Ffix;


// I actually need a ref cell because otherwise I won't be able
// to modify the variables referenced by refs_to_vars
pub struct RangeAnalyzer<'a> {
    refs_to_vars: Vec<&'a Ffix>,
    vars: Vec<Ffix>,
}

impl<'a> RangeAnalyzer<'a> {

    pub fn new(refs_to_vars: Vec<& Ffix>) -> RangeAnalyzer {
        let mut variables: Vec<Ffix> = Vec::new();

        for var in &refs_to_vars {
            variables.push(**var);
        }

        RangeAnalyzer {
            refs_to_vars,
            vars: variables,
        }
    } 

    pub fn update_ranges(&self) {
        for var in self.refs_to_vars.iter().enumerate() {

        }
    }
}