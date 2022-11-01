use std::ops;


fn quantize_fix(x: f64, s: bool, w: u32, f: u32) -> f64 {
    // This function quantizes a float number as a fixed point
    // signed/unsigned number with word length w and fraction length f.
    // The rounding method is floor. 
    // quantize_fix() is used by the Ffix structure to implement
    // fixed point operators.
    // Other options will be implemented in the future, such
    // as the possibility to set the signedness and the rounding
    // method.

    // ADD SANITY CHECKS ON w AND f !

    let fs: f64;
    let scaling_fact: f64;

    if f >= w {
        println!("error:quantize_fix:the number of fractional bits should
be strictly less than the word length. I'm not performing quantization.");
    }

    let base: u32 = 2;

    if s {
        fs = base.pow(w-f-1) as f64;
        scaling_fact = f64::from(base.pow(w-1))/fs as f64;
    } else {
        fs = base.pow(w-f) as f64;
        scaling_fact = f64::from(base.pow(w))/fs as f64;
    }
    
    // Compute quantized number
    let rounded = x*scaling_fact;
    let quantized = rounded.floor()/scaling_fact;

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
    } else if quantized < lower_lim{
        lower_lim
    } else {
        quantized
    }
}


#[derive(Debug, Clone, Copy)]
struct Ffix {
    // This struct allows to represent fixed point
    // numbers with floor rounding method. Other
    // features will be implemented in the future, such
    // as a precise rule for operations between numbers
    // with different word and fraction length and the
    // possibility to choose other rounding methods.
    value: f64,
    signed: bool,
    word_bits: u32,
    frac_bits: u32,
}

impl Ffix  {
    fn new(val: f64, signed: bool, word_bits: u32, frac_bits: u32) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
         Ffix {
            value: quantize_fix(val, signed, word_bits, frac_bits),
            signed,
            word_bits,
            frac_bits,
        }       
    }

    fn from(other: Ffix, signed: bool, word_bits: u32, frac_bits: u32) -> Ffix {
        // This associated function converts an instance
        // to another instance with specified word lenght
        // and fraction length. Note that the instance that
        // is passed to this method is moved into the
        // converted instance so it cannot be used anymore.

        Ffix {
            value: quantize_fix(other.value, signed, word_bits, frac_bits),
            signed,
            word_bits,
            frac_bits,            
        }
    }

    fn pow(&self, exponent: i32) -> Ffix {
        // This method implements fixed point exponentiation.

        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = quantize_fix(result*val,
                                self.signed,
                                self.word_bits,
                                self.frac_bits);
        }

        Ffix {
            value: result,
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
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
                                self.frac_bits),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
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
                                self.frac_bits),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
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
                                self.frac_bits),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
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
                                self.frac_bits),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
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
                self.frac_bits),
            signed: self.signed,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
        }
    }
}


fn main() {
    let a = Ffix::new(2.12345678, true, 18, 12);
    let b = Ffix::new(6.87654321, true, 18, 12);
    let c = Ffix::new(32.0, true, 18, 12);
    
    let x: f64 = 2.12345678;
    let y: f64 = 6.87654321;

    println!("Floating point results:");
    println!("x+y: {}", x+y);
    println!("x-y: {}", x-y);
    println!("x*y: {}", x*y);
    println!("x/y: {}", x/y);
    println!("x**2: {}", x.powi(2));

    println!("\nFixed point results:");
    println!("a+b: {}", (a+b).value);
    println!("a-b: {}", (a-b).value);
    println!("a*b: {}", (a*b).value);
    println!("a/b: {}", (a/b).value);
    println!("a**2: {}", a.pow(2).value);

    println!("\nOverflow test: b*c is {}", (b*c).value);
    println!("Overflow test: -b*c is {}", (-b*c).value);
}
