#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let x = 3.0;
    let y = 0.0;
    // let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
    // let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

    // while let
}
