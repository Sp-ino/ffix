// Module name: analysis.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains a set of functionalities
// that the user can rely on to size and evaluate fixed-point
// algorithms written using the Ffix type.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

// use crate::types::Ffix;
// use crate::types::internal::Range;

use crate::types::Ffix;



pub struct RangeAnalyzer<const S: bool, const W: u32, const F: u32, const R: char> {
    variables: Vec<Ffix<S, W, F, R>>,
    names: Vec<String>,
}


impl<const S: bool, const W: u32, const F: u32, const R: char> RangeAnalyzer<S, W, F, R> {
    
    pub fn new(vars: &Vec<Ffix<S, W, F, R>>, names: Vec<String>) -> RangeAnalyzer<S, W, F, R> {

        let variables = vars.clone();

        RangeAnalyzer::<S, W, F, R> {
            variables,
            names,
        }
    }

    pub fn log(&self) {
        for (idx, var) in self.variables.iter().enumerate() {
            println!("Range of {}: {}", self.names[idx], var.range().upper)
        }
    }
}