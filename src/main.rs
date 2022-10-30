use std::ops;


fn quantize_fix(x: f64, w: u32, f: u32) -> f64 {

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
struct Fix {
    value: f64,
    word_bits: u32,
    frac_bits: u32,
}

impl Fix  {
    pub fn new(v: f64, word_bits: u32, frac_bits: u32) -> Self {
         Fix {
            value: quantize_fix(v, word_bits, frac_bits),
            word_bits,
            frac_bits,
        }       
    }

    fn pow(&self, exponent: i32) -> Fix {
        let val = self.value;
        let mut result = self.value;

        for _i in 1..exponent {
            result = quantize_fix(result*val,
                                self.word_bits,
                                self.frac_bits);
        }

        Fix {
            value: result,
            word_bits: self.word_bits,
            frac_bits: self.frac_bits,
        }
    }
}

impl ops::Add for Fix {
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

impl ops::Sub for Fix {
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

impl ops::Mul for Fix {
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

impl ops::Div for Fix {
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
    let a = Fix::new(2.12345678, 18, 12);
    let b = Fix::new(6.87654321, 18, 12);
    
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
