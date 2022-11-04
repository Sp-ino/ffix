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

// IMPLEMENT FUNCTION/STRUCT THAT TRACKS VARIABLES AND REGISTERS THEIR
// MAXIMUM AND MINUMUM VALUES

// CONSIDER REMOVING Roundings

// ALTERNATIVE IMPLEMENTATIONS OF new() AND from() THAT ACCEPT A
// VARIABLE OF CUSTOM TYPE (SOMETHING LIKE Settings) TO DEFINE
// THE FIXED POINT SETTINGS.
// Two alternatives: completely abandon interface based
// on separated arguments for new() and from() or
// use an enum to store a tuple with (s, w, f, rounding) in one variant
// and 

mod quantization;

use std::ops;
use crate::types::quantization::quantize_fix;



#[derive(Clone, Copy, Debug)]
pub enum Roundings {
    Floor,
    Ceil,
    Zero,
    Infinity,
}



#[derive(Clone, Copy, Debug)]
pub struct FfixSettings {
    pub signed: bool,
    pub word_bits: u32,
    pub frac_bits: u32,
    pub rounding: Roundings,
}

impl FfixSettings {
    pub fn new(signed: bool, word_bits: u32, frac_bits: u32, rounding: Roundings) -> FfixSettings {
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
    // numbers with floor rounding method. Other
    // features will be implemented in the future, such
    // as a precise rule for operations between numbers
    // with different word and fraction length and the
    // possibility to choose other rounding methods.
    pub value: f64,
    pub settings: FfixSettings,
}

impl Ffix  {

    pub fn new(val: f64, settings: FfixSettings) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
        Ffix {
            value: quantize_fix(val, settings),
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
            value: quantize_fix(other.value, settings),
            settings,
        }
    }

    pub fn pow(&self, exponent: i32) -> Ffix {
        // This method implements fixed point exponentiation.
        
        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = quantize_fix(result*val, self.settings);
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
            value: quantize_fix(self.value+other.value,
                                self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Sub for Ffix {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value-other.value,
                                self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Mul for Ffix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value*other.value,
            self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Div for Ffix {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value/other.value,
                                self.settings),
            settings: self.settings,
        }
    }
}

impl ops::Neg for Ffix {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            value: quantize_fix(-self.value,
            self.settings),
            settings: self.settings,
        }
    }
}