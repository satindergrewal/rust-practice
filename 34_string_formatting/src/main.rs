#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

fn main() {
    let name = "Satinder";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    // define variable once and print repeatedly
    // by defining position of the variable in format!
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr);

    // define name for printing variables inside format!
    let info = format!(
        "the name's {last}. {first} {last}",
        first = "james",
        last = "bond"
    );
    println!("{}", info);

    // mixed format! display with data and position
    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        // "gama", // unused
        data = "delta"
    );
    println!("{}", mixed);
}
