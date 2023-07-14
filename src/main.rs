#[allow(dead_code)] // for beginning only
use std::fs::read_dir;

use crate::prelude::*;

mod error;
mod prelude;
mod utils;


fn main() -> Result<()> {
    println!("Hello, world!");
    for entry in read_dir("./")?
    .filter_map(|res| res.ok()) {
        let entry = String::try_from(W(&entry))?;
        println!("{}", entry);
    }
    Ok(())
}
