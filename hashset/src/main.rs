#![allow(unused_imports)]

use std::thread;
use std::time;
use std::collections::HashSet;

fn main() {
    let mut ritesh = HashSet::new();
    ritesh.insert("Alpha");
    ritesh.insert("Beta");
    ritesh.insert("Gamma");

    let added_vega = ritesh.insert("Vega");
    if added_vega{
        println!("We added Vega! hooray!");
    }

    if !ritesh.contains("Kappa"){
        println!("We don't have kappa");
    }
    
    let removed = ritesh.remove("Gamma");
    if removed {
        println!("We removed Gamma");
    }
    println!("{:?}",ritesh);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,_2_8.is_subset(&_1_10)
    );

    //Disjoint = no common elements
    println!(
        "is {:?} a subset of {:?}? {}",
        _1_5, _6_10,_1_5.is_disjoint(&_6_10)
    );

    //union, intersection
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8, _6_10,_2_8.union(&_6_10)
    );
    
    //difference
    //symmetric difference = union - intersection
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8, _6_10,_2_8.difference(&_6_10)
    );
    

}
