use ffix::types::Ffix;
use ffix::types::Roundings;


fn main() {
    let r: Roundings = Roundings::Floor;
    let a = Ffix::new(2.12345678, true, 18, 12, r);
    let b = Ffix::new(6.87654321, true, 18, 12, r);
    let c = Ffix::new(32.0, true, 18, 12, r);
    
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

    // for i in 1..100 {
    //     println!("Rounded to zero {}", round::half_towards_zero(-1.0-1.0/f64::from(i), 0));
    // }
}