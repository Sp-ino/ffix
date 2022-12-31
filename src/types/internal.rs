// Module name: internal.rs
// 
// Author: Valerio Spinogatti
// 
// Description: this module contains definitions
// related to the internal operation of the
// Ffix and Ffix settings struct
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

use core::panic;

use math::round;



pub fn quantize_fix(x: f64, s: bool, w: u32, f: u32, r: char) -> f64 {
    // This function quantizes a float number as a fixed point
    // signed/unsigned number with word length w and fraction length f.
    // The rounding method is floor. 
    // quantize_fix() is used by the Ffix structure to implement
    // fixed point operators.
    // Other options will be implemented in the future, such
    // as the possibility to set the signedness and the rounding
    // method.

    let fs: f64;
    let scaling_fact: f64;

    if f >= w {
        println!("error:quantize_fix:the number of fractional bits should
be strictly less than the word length. I'm not performing quantization.");
    }

    let base: u32 = 2;

    if s {
        fs = base.pow(w-f-1) as f64;
        scaling_fact = f64::from(base.pow(w-1))/fs;
    } else {
        fs = base.pow(w-f) as f64;
        scaling_fact = f64::from(base.pow(w))/fs;
    }
    
    // Compute quantized number
    let scaled = x*scaling_fact;
    let rounded = round(scaled, r);
    let quantized = rounded/scaling_fact;

    // Check and handle overflows
    // First, we compute upper and lower limits
    let upper_lim: f64 = fs - f64::from(base.pow(f)).powi(-1);
    let lower_lim: f64;
    if s {
        lower_lim = -fs;
    } else {
        lower_lim = 0.0;   
    }
    // Then we check for overflow and saturate the output if overflow occurs
    if quantized > upper_lim {
        upper_lim
    } else if quantized < lower_lim {
        lower_lim
    } else {
        quantized
    }
}



fn round(value: f64, rounding: char) -> f64 {
    // This is a private function use by quantization
    // to map the rounding method onto a rounding operation
    match rounding {
        'f' => value.floor(),
        'c' => value.ceil(),
        'z' => round::half_towards_zero(value, 0),
        'i' => round::half_away_from_zero(value, 0),
        _ => panic!("Wrong rounding type specified"),
    }    
}