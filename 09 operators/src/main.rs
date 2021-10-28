#[allow(dead_code)]
#[allow(unused_variables)]

// use std::mem;

fn main() {
    
    // arithmetic

    let mut a = 2+3*4; // +-*/ ... multiplication happens first and then addition
    println!("{}", a);
    a = a+1; // -- ++ is not allowed. instead incremental need number
    a -= 2; // a = a - 2;
            // -= += *= /= %=
    
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // i in powi means it will calculate integer value
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // f in powf means it will calculate floating vallue
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    // in rust bitwise operaters are only avaible to integers
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                    // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("x is 5 = {}", x_is_5);


}
