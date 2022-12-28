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


struct Range {
    max: f64,
    min: f64,
}


impl Range {

    fn new() -> Range {
        Range{
            max: 0.0,
            min: 0.0,
        }
    }

}


pub struct RangeAnalyzer {
    n_variables: i32,
    ranges: Vec<Range>,
}


impl RangeAnalyzer {
    
    pub fn new() -> RangeAnalyzer {
        RangeAnalyzer {
            n_variables: 0,
            ranges: vec![Range::new()],
        }
    }


    pub fn update(vars: Vec<Ffix>) {

    }
}




// I actually need a ref cell because otherwise I won't be able
// to modify the variables referenced by refs_to_vars
// pub struct RangeAnalyzer<'a> {
//     refs_to_vars: Vec<&'a Ffix>,
//     vars: Vec<Ffix>,
// }

// impl<'a> RangeAnalyzer<'a> {

//     pub fn new(refs_to_vars: Vec<& Ffix>) -> RangeAnalyzer {
//         let mut variables: Vec<Ffix> = Vec::new();

//         for var in &refs_to_vars {
//             variables.push(**var);
//         }

//         RangeAnalyzer {
//             refs_to_vars,
//             vars: variables,
//         }
//     } 

//     pub fn update_ranges(&self) {
//         for var in self.refs_to_vars.iter().enumerate() {

//         }
//     }
// }


// Even this implementation with a VarManager that contains all the
// variables the user needs seems too cumbersome

// pub struct VarManager {
//     vars: Vec<Ffix>,
//     names: Vec<String>,
//     ranges: Vec<Range>,
// }


// impl VarManager {

//     pub fn new(vars: Vec<Ffix>, names: Vec<String>) -> VarManager {
//         let mut ranges: Vec<Range> = Vec::new();

//         if vars.len() != names.len() {
//             panic!("vrs (variables) argument must have same length as nms (names)!");
//         }

//         for _ in 0..vars.len() {
//             let rng = Range{
//                 min: 0.0,
//                 max: 0.0,
//             };
//             ranges.push(rng);
//         }

//         VarManager{
//             vars,
//             names,
//             ranges,
//         }
//     }


//     pub fn update(&mut self) {
//         for (idx, &item) in self.vars.iter().enumerate() {
//             self.ranges[idx].max = item.value().max(self.vars[idx].value());
//             self.ranges[idx].min = item.value().min(self.vars[idx].value());
//         }
//     }


//     pub fn set(&mut self, name: &str) {

//     }


//     pub fn get(&self, name: &str) {

//     }

// }