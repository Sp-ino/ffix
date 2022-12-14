// Module name: types.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains the definitions of the types
// that the user needs to implement code with fixed-point variables.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

mod range;
pub mod quantization;

use std::ops;
use std::cmp::PartialOrd;
use num_traits::identities::{One, Zero};
use crate::types::range::Range;



#[derive(Debug, Clone, Copy)]
pub struct Ffix<const S: bool, const W: u32, const F: u32, const R: char> {
    // This struct represents fixed point
    // numbers with arbitrary signedness, word length,
    // fraction length, and rounding method. We refer
    // to these properties as the "settings" of the
    // Ffix type. Settings are specified as generic
    // constant parameters.

    value: f64,
    range: Range,
}

impl<const S: bool, const W: u32, const F: u32, const R: char> Ffix<S, W, F, R> {
    
    pub fn new(val: f64) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
        
        Ffix {
            value: quantization::quantize_fix(val, S, W, F, R),
            range: Range::new(),
        }       
    }
    
    pub fn cast<const NS: bool, const NW: u32, const NF: u32, const NR: char>
                (given: &Ffix<S, W, F, R>) -> Ffix<NS, NW, NF, NR> {
        // This associated function transforms the argument
        // into a new Ffix variable with settings NS, NW, NF, NR
        // Note that the new settings have to be passed as
        // generic constant parameters and that the argument
        // is passed by reference. cast() is useful for performing
        // conversions between different Ffix types in implementations
        // that requires Ffix variables with different settings to coexist.
        // Also note that this function does not consume the value
        // that is passed to it.
    
        Ffix::<NS, NW, NF, NR> {
            value: quantization::quantize_fix(given.value, NS, NW, NF, NR),
            range: Range::new(),
        }
    }

    pub fn another(&self, value: f64) -> Ffix<S, W, F, R> {
        // Returns a new Ffix variables with the same settings as the
        // current one and a new value that is specified by the user.
        // This method is useful to create Ffix variables with the same
        // settings without having to specify the generics every time.

        Ffix::<S, W, F, R> {
            value: quantization::quantize_fix(value, S, W, F, R),
            range: Range::new(),
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
            result = quantization::quantize_fix(result*val, S, W, F, R);
        }

        Ffix {
            value: result,
            range: self.range,
        }
    }

    pub fn upd(&mut self, new: Ffix<S, W, F, R>) {
        // Can be called when assigning the result of an operation between Ffix
        // variables to an existing variable. The reason to use upd() instead of
        // the = operator is that upd() updates the range field
        // so that range analysis can be performed.

        self.range.update(new.value);

        self.value =  new.value;
    }

}

// The methods that follow implement overloaded arithmetic operations
// for fixed point numbers
impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Add for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {

        Self {
            value: quantization::quantize_fix(self.value+other.value,
                                            S, W, F, R),
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Sub for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {

        Self {
            value: quantization::quantize_fix(self.value-other.value,
                                            S, W, F, R),
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Mul for Ffix<S, W, F, R> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {

        Self {
            value: quantization::quantize_fix(self.value*other.value,
                                            S, W, F, R),
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Div for Ffix<S, W, F, R> {
    type Output = Self;

    fn div(self, other: Self) -> Self {

        Self {
            value: quantization::quantize_fix(self.value/other.value,
                                            S, W, F, R),
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> ops::Neg for Ffix<S, W, F, R> {
    type Output = Self;
    
    fn neg(self) -> Self {

        Self {
            value: quantization::quantize_fix(-self.value,
                                            S, W, F, R),
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> One for Ffix<S, W, F, R> {

    fn one() -> Self {
        Ffix::<S, W, F, R> {
            value: 1.,
            range: Range::new(),
        }
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> Zero for Ffix<S, W, F, R> {

    fn zero() -> Self {
        Ffix::<S, W, F, R> {
            value: 0.0,
            range: Range::new(),
        }
    }

    fn is_zero(&self) -> bool {
        
        let base: u32 = 2;
        let lsb = f64::from(base
                                .pow(F))
                                .powi(-1);

        if self.value.abs() < lsb {
            return true
        }

        false
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> PartialEq for Ffix<S, W, F, R> {
    
    fn eq(&self, other: &Self) -> bool {
        // Better to use the eq implementation for float on the values of self and other
        self.value.eq(&other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<const S: bool, const W: u32, const F: u32, const R: char> PartialOrd for Ffix<S, W, F, R> {
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value 
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}