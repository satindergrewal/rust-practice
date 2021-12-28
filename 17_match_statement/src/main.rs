#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", //1 to 1000 range
        _ => "invalid" // catch all case
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no"
    };
}
