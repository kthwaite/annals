extern crate annals;
extern crate serde_yaml;

use annals::{Context, Scribe};
use std::fs::File;

fn main() {
    let mut scribe = Scribe::new();
    scribe.load_cognates("texts/tracery.yml").unwrap();
    for i in 0..10 {
        println!("{}", scribe.gen("origin").unwrap());
    }
}