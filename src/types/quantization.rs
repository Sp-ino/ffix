// Module name: quantization.rs
// 
// Author: Valerio Spinogatti
// 
// Description: defines functions used to
// quantize the values stored in Ffix variables
// after each operation/assignment
// 
// Copyright (c) 2022 Valerio Spinogatti
// Licensed under GNU license

use core::panic;
use math::round;



pub fn compute_lims(s: bool, w: u32, f: u32, base: u32) -> (f64, f64, f64) {
    // Computes the full scale value, the upper limit
    // and the lower limit given the signedness, the
    // word and fraction length and the base.

    let fs: f64;

    if s {
        fs = base.pow(w-f-1) as f64;
    } else {
        fs = base.pow(w-f) as f64;
    }

    let upper_lim: f64 = fs - f64::from(base
        .pow(f))
        .powi(-1);

    let lower_lim: f64;
    if s {
    lower_lim = -fs;
    } else {
    lower_lim = 0.0;   
    }

    (fs, upper_lim, lower_lim)
}



pub fn quantize_fix(x: f64, s: bool, w: u32, f: u32, r: char) -> f64 {
    // Quantizes a float number as a fixed point
    // signed/unsigned number with word length w, fraction length
    // f and rounding method r (r can be 'z' (towards zero), 'i'
    // (towards infinity), 'f' (floor), or 'c' (ceil). 
    // quantize_fix() is used by the Ffix structure to implement
    // fixed point operators.

    let fs: f64;
    let upper_lim: f64;
    let lower_lim: f64;
    let scaling_fact: f64;
    let base: u32 = 2;

    // Sanity check on f and w
    if f >= w {
        println!("error:quantize_fix:the number of fractional bits should
be strictly less than the word length. I'm not performing quantization.");
    }

    // Compute full scale and upper/lower saturation limits
    (fs, upper_lim, lower_lim) = compute_lims(s, w, f, base);

    // Compute scaling factor to be used for quantization
    if s {
        scaling_fact = f64::from(base.pow(w-1))/fs;
    } else {
        scaling_fact = f64::from(base.pow(w))/fs;
    }
    
    // Compute quantized number
    let scaled = x*scaling_fact;
    let rounded = round(scaled, r);
    let quantized = rounded/scaling_fact;

    // Check and handle overflows
    // First, we compute upper and lower limits
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
    // This is a private function used by quantization()
    // to map the rounding method onto a rounding operation
    match rounding {
        'f' => value.floor(),
        'c' => value.ceil(),
        'z' => round::half_towards_zero(value, 0),
        'i' => round::half_away_from_zero(value, 0),
        _ => panic!("Wrong rounding type specified"),
    }    
}