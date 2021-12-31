#[allow(dead_code)]
#[allow(unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub fn generics() {
    let a:Point<f64> = Point { x: 0.0, y: 4f64 };
    let b = Point { x: 1.2, y: 3.4 };

    // won't work initially
    let myline = Line { start: a, end: b };
    println!("start: {} {}",myline.start.x, myline.start.y);
    println!("end: {} {}",myline.end.x, myline.end.y);
}


fn main() {
    generics();
}
