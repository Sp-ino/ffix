use ffix::types::Ffix;


fn main() {

    let a: Ffix<true, 18, 16, 'f'> = Ffix::new(2.12345678);
    let b: Ffix<true, 18, 16, 'f'> = Ffix::new(6.87654321);
    let c: Ffix<true, 18, 16, 'f'> = Ffix::new(32.0);  
    let d: Ffix<true, 24, 16, 'z'> = Ffix::new(32.0);
    let z: Ffix<false, 24, 15, 'c'> = Ffix::new(24.0);

    println!("{}", a.word_bits());
    println!("{}", d.word_bits());
    println!("{}", d.rounding());
    
    println!("z.rounding before: {}", z.rounding());
    let z = Ffix::from::<false, 20, 18, 'z'>(&z);
    println!("z.rounding after: {}", z.rounding());
    
    let x: f64 = 2.12345678;
    let y: f64 = 6.87654321;
    
    println!("Floating point results:");
    println!("x+y: {}", x+y);
    println!("x-y: {}", x-y);
    println!("x*y: {}", x*y);
    println!("x/y: {}", x/y);
    println!("x**2: {}", x.powi(2));
    
    println!("\nFixed point results:");
    println!("a+b: {}", (a+b).value());
    println!("a-b: {}", (a-b).value());
    println!("a*b: {}", (a*b).value());
    println!("a/b: {}", (a/b).value());
    println!("a**2: {}", a.pow(2).value());
    
    println!("\nOverflow test: b*c is {}", (b*c).value());
    println!("Overflow test: -b*c is {}\n", (-b*c).value());

    // // I can also define vectors of Ffix
    // let mut v: Vec<Ffix<true, 24, 16, 0>> = Vec::new();
    // v.push(a);
    // v.push(b);
    // v.push(c);

    // for item in &v {
    //     println!("Value of item is {}", item.value());
    // }

    // for (idx, item) in v.iter().enumerate() {
    //     println!("Value of item at index {} is {}", idx, item.value());
    // }

    // println!("value of v[0]: {}", v[0].value());

    // v[0] = v[1] - v[2];

    // println!("v[0] is {}", v[0].value());

    // // for i in 1..100 {
    // //     println!("Rounded to zero {}", round::half_towards_zero(-1.0-1.0/f64::from(i), 0));
    // // }
}