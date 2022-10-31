use std::ops;


fn quantize_fix(x: f64, w: u32, f: u32) -> f64 {
    // This function quantizes a float number as a fixed point
    // signed number with word length w and fraction length f.
    // The rounding method is floor. 
    // quantize_fix() is used by the Ffix structure to implement
    // fixed point operators.
    // Other options will be implemented in the future, such
    // as the possibility to set the signedness and the rounding
    // method.

    if f >= w {
        println!("error:quantize_fix:the number of fractional bits should
be strictly less than the word length. I'm not performing quantization.");
    }

    let base: u32 = 2;    
    let fs = base.pow(w-f-1) as f64;
    let max_int = base.pow(w-1) as f64;
    let normalizing_fact = max_int/fs as f64;
    let rounded = x*normalizing_fact;
    
    rounded.floor()/normalizing_fact
}


#[derive(Debug, Clone, Copy)]
struct Ffix {
    // This struct allows to represent signed fixed point
    // numbers with floor rounding method. For the moment,
    // the user can only choose the word length and the
    // fraction length of the fixed point number. Other
    // features will be implemented in the future, such
    // as the possibility to represent unsigned numbers
    // and a precise rule for operations between numbers
    // with different word and fraction length.
    value: f64,
    word_bits: u32,
    frac_bits: u32,
}

impl Ffix  {
    fn new(val: f64, word_bits: u32, frac_bits: u32) -> Self {
        // This associated function is a constructor for
        // creating new instances of Ffix.
         Ffix {
            value: quantize_fix(val, word_bits, frac_bits),
            word_bits,
            frac_bits,
        }       
    }

    fn from(other: Ffix, word_bits: u32, frac_bits: u32) -> Ffix {
        // This associated function converts an instance
        // to another instance with specified word lenght
        // and fraction length. Note that the instance that
        // is passed to this method is moved into the
        // converted instance so it cannot be used anymore.

        Ffix {
            value: quantize_fix(other.value, word_bits, frac_bits),
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
                                self.word_bits,
                                self.frac_bits);
        }

        Ffix {
            value: result,
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
                                self.word_bits,
                                self.frac_bits),
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
                                self.word_bits,
                                self.frac_bits),
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
                                self.word_bits,
                                self.frac_bits),
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
                                self.word_bits,
                                self.frac_bits),
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
        }
    }
}


fn main() {
    let a = Ffix::new(2.12345678, 18, 12);
    let b = Ffix::new(6.87654321, 18, 12);
    
    let x: f64 = 2.12345678;
    let y: f64 = 6.87654321;

    println!("Fixed point results:");
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
}
