#[allow(dead_code)]
#[allow(unused_variables)]

pub fn for_loop() {
    // for loop from 1 to 10 with a "range" construct
    // will execute the loop 10 times
    for x in 1..11
    {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }

    // pos = position, index
    // y = value
    for (pos,y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn main() {
    for_loop();
}
