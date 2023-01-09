use ffix::types::Ffix;
use ffix::analysis::RangeAnalyzer;


fn main() {

    // -------------------Some basic operations------------------------------ 
    let a: Ffix<true, 24, 16, 'f'> = Ffix::new(2.12345678);
    let b: Ffix<true, 24, 16, 'f'> = Ffix::new(6.87654321);
    let c: Ffix<true, 24, 16, 'f'> = Ffix::new(32.0);  
    let d: Ffix<true, 24, 16, 'z'> = Ffix::new(32.0);
    let z: Ffix<false, 24, 15, 'c'> = Ffix::new(24.0);

    println!("{}", a.word_bits());
    println!("{}", d.word_bits());
    println!("{}", d.rounding());
    
    println!("z.rounding before: {}", z.rounding());
    let z = Ffix::cast::<false, 20, 16, 'z'>(&z);
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
    // ------------------------------------------------------------------

    // ---------------I can also define vectors of Ffix------------------
    let mut v: Vec<Ffix<true, 24, 16, 'f'>> = Vec::new();
    v.push(a);
    v.push(b);
    v.push(c);
    v.push(c.another(0.5));
    v.push(c.another(15.0));
    v.push(c.another(16.13));

    for item in &v {
        println!("Value of item is {}", item.value());
    }

    for (idx, item) in v.iter().enumerate() {
        println!("Value of item at index {} is {}", idx, item.value());
    }

    println!("value of v[0]: {}", v[0].value());

    v[0] = v[1] - v[2];

    println!("v[0] is {}", v[0].value());
    // ------------------------------------------------------------------

    // ------------------Also, comparing two Ffix is allowed-------------
    let outcome1 = a > b;
    let outcome2 = a < b;
    let outcome3 = a == b;
    println!("Result of comparison a > b: {outcome1}");
    println!("Result of comparison a < b: {outcome2}");
    println!("Result of comparison a == b: {outcome3}");

    let c = Ffix::cast::<true, 18, 16, 'z'>(&a);
    let d = Ffix::cast::<true, 18, 16, 'z'>(&b);
    let outcome1 = c > d;
    let outcome2 = c < d;
    let outcome3 = c == d;
    println!("Result of comparison c > d: {outcome1}");
    println!("Result of comparison c < d: {outcome2}");
    println!("Result of comparison c == d: {outcome3}");
    // ------------------------------------------------------------------

    // --------------Experiments with upd()------------------------------
    println!("\n\n");
    let mut result = v[0].another(10.0);

    for (i, item) in v.iter().enumerate() {
        let iffix = Ffix::<true, 24, 16, 'f'>::new(i as f64);
        let itm = *item;
        let interm = (iffix * itm - itm.another(10.0))/result;
        println!("interm is {}", interm.value());
        result.upd(interm);
    }
    println!("Range of the result variable: {:?}\n\n", result.range());

    let ra = RangeAnalyzer::new(&vec![result],
                                                                vec!["result".to_string()]);

    ra.log();
    // ------------------------------------------------------------------

}