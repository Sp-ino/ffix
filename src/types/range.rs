// Module name: range.rs
// 
// Author: Valerio Spinogatti
// 
// Description: defines the Range type, which is used
// by Ffix variables to store the range spanned by the
// value field while the variable is being used.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license



#[derive(Clone, Copy, Debug)]
pub struct Range {
    // Data type that represents the range
    // covered by a Ffix variable during its
    // existence
    pub upper: f64,
    pub lower: f64,
}

impl Range {

    pub fn new() -> Range {
        // Constructor for the Range type

        Range{
            upper: 0.0,
            lower: 0.0,
        }
    }

    pub fn update(&mut self, new: f64) {
        // Update upper and lower fields. Updating upper and lower means
        // that upper is replaced by the max between itself and the most
        // recent value (that is, new) while lower is replaced by the minimum
        // between itself and the most recent value. 

        self.upper = self.upper.max(new);
        self.lower = self.lower.min(new);
    }

    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}

