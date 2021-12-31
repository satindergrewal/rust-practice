#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

// use std::thread;
// use std::time;
use std::collections::HashSet;

fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}",greeks);

    // hashsets only keep unique elements
    // inserting the same element again will not affect anything
    greeks.insert("delta");
    println!("{:?}",greeks);

    // get boolian value to check if insert operation was successful or not
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega! horray!");
    }
    println!("{:?}",greeks);

    let added_delta = greeks.insert("delta");
    if added_delta {
        println!("we added delta! horray!");
    } else {
        println!("delta is already there");
    }
    println!("{:?}",greeks);

    // check if an element exists or not
    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta");
        println!("{:?}",greeks);
    }

    // making numbers from 1 to 5
    // hashset type will be infered from the range created
    // range created from 1 to 5
    // .collect() to create collection
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // check if it's subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint = no common elements
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5, _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // union - data set from both of the sets
    // intersection - the set of elements which are available in both sets
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );

    println!(
        "items in both {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.intersection(&_6_10)
    );

    // difference = items which are in first set but not in second
    // symmetric_difference = union - intersection = items whcih are in the uniion but not in the intersection
    println!(
        "items in {:?} but not in {:?} are {:?}",
        _6_10,
        _2_8,
        _6_10.difference(&_2_8)
    );

    println!(
        "items union - intersection {:?} and {:?} are {:?}",
        _2_8,
        _1_10,
        _1_10.symmetric_difference(&_2_8)
    );
}
