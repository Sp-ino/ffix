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

use crate::types::Ffix;
use crate::types::quantization;



pub struct RangeAnalyzer<const S: bool, const W: u32, const F: u32, const R: char> {
    variables: Vec<Ffix<S, W, F, R>>,
    names: Vec<String>,
}
// use crate::types::Ffix;


impl<const S: bool, const W: u32, const F: u32, const R: char> RangeAnalyzer<S, W, F, R> {
    
    pub fn new(vars: &Vec<Ffix<S, W, F, R>>, names: Vec<String>) -> RangeAnalyzer<S, W, F, R> {

        let variables = vars.clone();

        RangeAnalyzer::<S, W, F, R> {
            variables,
            names,
        }
    }

    pub fn log(&self) {
        println!("Name\t\t\tMinimum\t\t\tMaximum\t\t\tPercentage");
        
        for (idx, var) in self.variables.iter().enumerate() {
            let uplim;
            let lowlim;
            (_, uplim, lowlim) = quantization::compute_lims(var.signed(),
                                                            var.word_bits(),
                                                            var.frac_bits(),
                                                            2);
            
            let allowed_width = uplim - lowlim;
            let actual_width = var.range().width();
            let percentage_range = 100.0 * actual_width / allowed_width;
            println!("{}{:24.2}{:24.2}{:24.2}",
                    self.names[idx],
                    var.range().lower,
                    var.range().upper,
                    percentage_range);
        }
    }
}