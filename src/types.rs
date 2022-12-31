// Module name: types.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains the definitions of the types
// that the user needs to implement code with fixed-point variables.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

mod internal;

use std::ops;
use crate::analysis::Range;



#[derive(Debug, Clone, Copy)]
pub struct Ffix<const S: bool, const W: u32, const F: u32, const R: char> {
    // This struct allows to represent fixed point
    // numbers with arbitrary signedness, word length,
    // fraction length, and rounding method.
    value: f64,
    range: Range,
}

impl<const S: bool, const W: u32, const F: u32, const R: char> Ffix<S, W, F, R> {
    
    pub fn new(val: f64) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
        let range = Range::new(); 
        
        Ffix {
            value: internal::quantize_fix(val, S, W, F, R),
            range,
        }       
    }
    
    pub fn from<const NS: bool, const NW: u32, const NF: u32, const NR: char>
                (other: Ffix<S, W, F, R>) -> Ffix<NS, NW, NF, NR> {
        // This associated function converts an instance
        // to another instance with specified word lenght
        // and fraction length. Note that the instance that
        // is passed to this method is moved into the
        // converted instance so it cannot be used anymore.
        let range = Range::new(); 
    
        Ffix::<NS, NW, NF, NR> {
        value: internal::quantize_fix(other.value,
                                    NS,
                                    NW,
                                    NF,
                                    NR),
        range,
        }
    }

    pub fn value(&self) -> f64 {
        // Getter method for value
        self.value
    }

    pub fn signed(&self) -> bool {
        // Getter method for signedness
        S
    }

    pub fn word_bits(&self) -> u32 {
        // Getter method for word length
        W
    }

    pub fn frac_bits(&self) -> u32 {
        // Getter method for fraction length
        F
    }

    pub fn rounding(&self) -> char {
        // Getter method for rounding method
        R
    }

    pub fn range(&self) -> Range {
        // Getter method for settings.range
        self.range
    }

    pub fn pow(&self, exponent: i32) -> Ffix<S, W, F, R> {
        // Method that implements fixed point exponentiation.        
        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = internal::quantize_fix(result*val, S, W, F, R);
        }

        Ffix {
            value: result,
            range: self.range,
        }
    }

    pub fn upd(&mut self, new: Ffix<S, W, F, R>) {
        // Should be called when assigning the result
        // of an operation between Ffix<S, W, F, R> variables to an existing
        // variable. The method updates the range field
        // so that the user can perform range analysis

        self.range.upper = self.range
                                .upper
                                .max(new.value());

        self.range.lower = self.range
                                .lower
                                .min(new.value());

        self.value =  new.value;
    }

}

// The methods that follow implement overloaded arithmetic operations
// for fixed point numbers
impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Add for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        let range = Range::new();

        Self {
            value: internal::quantize_fix(self.value+other.value,
                                            S, W, F, R),
            range,
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Sub for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        let range = Range::new();

        Self {
            value: internal::quantize_fix(self.value-other.value,
                                            S, W, F, R),
            range,
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Mul for Ffix<S, W, F, R> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let range = Range::new();

        Self {
            value: internal::quantize_fix(self.value*other.value,
                                            S, W, F, R),
            range,
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Div for Ffix<S, W, F, R> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let range = Range::new();

        Self {
            value: internal::quantize_fix(self.value/other.value,
                                            S, W, F, R),
            range,
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Neg for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn neg(self) -> Self {
        let range = Range::new();

        Self {
            value: internal::quantize_fix(-self.value,
                                            S, W, F, R),
            range,
        }
    }
}

