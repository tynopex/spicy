extern crate spicy;

use spicy::{fileformat,solve};


#[cfg_attr(test, allow(dead_code))]
fn main()
{
    let fname =
        std::env::args()
                 .skip(1)
                 .next()
                 .unwrap();

    let gr = fileformat::load_spicy(&fname);
    let sr = solve::solve(gr.as_ref().unwrap());

    println!("Node Voltage    {:?}", sr.v);
    println!("Element Current {:?}", sr.i);
}
