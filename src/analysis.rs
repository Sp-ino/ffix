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

use crate::types::Ffix;



#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub upper: f64,
    pub lower: f64,
}


impl Range {

    pub fn new() -> Range {
        Range{
            upper: 0.0,
            lower: 0.0,
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
            ranges: Vec::new(),
        }
    }

    pub fn register_vars(&mut self, vars: &Vec<Ffix>) {
        
        self.n_variables = vars.len() as i32;
        
        for (idx, var) in vars.iter().enumerate() {
            self.ranges[idx] = var.range();
        }
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
//                 lower: 0.0,
//                 upper: 0.0,
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
//             self.ranges[idx].upper = item.value().upper(self.vars[idx].value());
//             self.ranges[idx].lower = item.value().lower(self.vars[idx].value());
//         }
//     }


//     pub fn set(&mut self, name: &str) {

//     }


//     pub fn get(&self, name: &str) {

//     }

// }