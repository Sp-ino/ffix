// Module name: types.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains the definitions of the types
// that the user needs to implement code with fixed-point variables.
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

use std::ops;

mod quantization;
use crate::types::quantization::quantize_fix;

// TO DO:
// IMPLEMENT CHECKS ON OVERLOADED OPERATORS TO AVOID OPERATIONS BETWEEN
// Ffix WITH DIFFERENT WORD/FRACTION LENGTHS

// IMPLEMENT FUNCTION/STRUCT THAT TRACKS VARIABLES AND REGISTERS THEIR
// MAXIMUM AND MINUMUM VALUES

// CONSIDER REMOVING Roundings

// IMPLEMENT FixVector



#[derive(Clone, Copy, Debug)]
pub enum Roundings {
    Floor,
    Ceil,
    Zero,
    Infinity,
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
    pub signed: bool,
    pub word_bits: u32,
    pub frac_bits: u32,
    pub rounding: Roundings,
}

impl Ffix  {
    pub fn new(val: f64, signed: bool, word_bits: u32, frac_bits: u32, rounding: Roundings) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
        Ffix {
            value: quantize_fix(val, signed, word_bits, frac_bits, rounding),
            signed,
            word_bits,
            frac_bits,
            rounding,
        }       
    }

    pub fn from(other: Ffix, signed: bool, word_bits: u32, frac_bits: u32, rounding: Roundings) -> Ffix {
        // This associated function converts an instance
        // to another instance with specified word lenght
        // and fraction length. Note that the instance that
        // is passed to this method is moved into the
        // converted instance so it cannot be used anymore.
        
        Ffix {
            value: quantize_fix(other.value, signed, word_bits, frac_bits, rounding),
            signed,
            word_bits,
            frac_bits,
            rounding,
        }
    }

    pub fn pow(&self, exponent: i32) -> Ffix {
        // This method implements fixed point exponentiation.
        
        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = quantize_fix(result*val,
                                self.signed,
                                self.word_bits,
                                self.frac_bits,
                                self.rounding);
        }

        Ffix {
            value: result,
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding
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
                                self.signed,
                                self.word_bits,
                                self.frac_bits,
                                self.rounding),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding,
        }
    }
}

impl ops::Sub for Ffix {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value-other.value,
                                self.signed,
                                self.word_bits,
                                self.frac_bits,
                                self.rounding),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding,
        }
    }
}

impl ops::Mul for Ffix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value*other.value,
                                self.signed,
                                self.word_bits,
                                self.frac_bits,
                                self.rounding),
                                signed: self.signed,
                                word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding,
            
        }
    }
}

impl ops::Div for Ffix {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: quantize_fix(self.value/other.value,
                                self.signed,
                                self.word_bits,
                                self.frac_bits,
                                self.rounding),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding,
        }
    }
}

impl ops::Neg for Ffix {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            value: quantize_fix(-self.value,
                self.signed,
                self.word_bits,
                self.frac_bits,
                self.rounding),
                signed: self.signed,
                word_bits: self.word_bits,
            frac_bits: self.frac_bits,
            rounding: self.rounding,
        }
    }
}