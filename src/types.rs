// Module name: types.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains the definitions of the types
// that the user needs to implement code with fixed-point variables.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license


// TO DO:
// IMPLEMENT CHECKS ON OVERLOADED OPERATORS TO AVOID OPERATIONS BETWEEN
// Ffix WITH DIFFERENT WORD/FRACTION LENGTHS

// CONSIDER REMOVING Roundings


mod internal;
mod utils;

use std::ops;
use crate::types::internal::Roundings;



#[derive(Clone, Copy, Debug)]
pub struct FfixSettings {
    // This struct wraps the settings
    // of a Ffix object
    pub signed: bool,
    pub word_bits: u32,
    pub frac_bits: u32,
    pub rounding: Roundings,
}

impl FfixSettings {
    pub fn new(signed: bool, word_bits: u32, frac_bits: u32, rounding_s: &str) -> FfixSettings {
        // This associated function is a constructor for
        // the FfixSettings class. For the rounding method,
        // the function expects a string literal. However
        // note that internally 
        let rounding = utils::find_rounding(rounding_s);

        FfixSettings {
            signed,
            word_bits,
            frac_bits,
            rounding,        
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Ffix {
    // This struct allows to represent fixed point
    // numbers with arbitrary signedness, word length,
    // fraction length, and rounding method.
    value: f64,
    settings: FfixSettings,
}

impl Ffix  {

    pub fn new(val: f64, settings: FfixSettings) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
        Ffix {
            value: internal::quantize_fix(val, settings),
            settings,
        }       
    }

    pub fn from(other: Ffix, settings: FfixSettings) -> Ffix {
        // This associated function converts an instance
        // to another instance with specified word lenght
        // and fraction length. Note that the instance that
        // is passed to this method is moved into the
        // converted instance so it cannot be used anymore.
        Ffix {
            value: internal::quantize_fix(other.value, settings),
            settings,
        }
    }

    pub fn value(&self) -> f64 {
        // Getter method for value
        self.value
    }

    pub fn signed(&self) -> bool {
        // Getter method for settings.signed
        self.settings.signed
    }

    pub fn word_bits(&self) -> u32 {
        // Getter method for settings.word_bits
        self.settings.word_bits
    }

    pub fn frac_bits(&self) -> u32 {
        // Getter method for settings.frac_bits
        self.settings.frac_bits
    }

    pub fn rounding(&self) -> Roundings {
        // Getter method for settings.rounding
        self.settings.rounding
    }

    pub fn pow(&self, exponent: i32) -> Ffix {
        // This method implements fixed point exponentiation.        
        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = internal::quantize_fix(result*val, self.settings);
        }

        Ffix {
            value: result,
            settings: self.settings,
        }
    }
}

// The methods that follow implement overloaded arithmetic operations
// for fixed point numbers
impl ops::Add for Ffix {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            value: internal::quantize_fix(self.value+other.value,
                                            self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Sub for Ffix {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            value: internal::quantize_fix(self.value-other.value,
                                            self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Mul for Ffix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: internal::quantize_fix(self.value*other.value,
                                            self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Div for Ffix {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: internal::quantize_fix(self.value/other.value,
                                            self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Neg for Ffix {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            value: internal::quantize_fix(-self.value,
                                            self.settings),
            settings: self.settings,
        }
    }
}