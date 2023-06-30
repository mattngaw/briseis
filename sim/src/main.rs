pub mod isa;
pub mod core;
pub mod components;

use crate::components::{
    regfile::RegFile,
    stack::Stack,
};

fn main() {
    env_logger::builder().init();

    log::info!("Hello, world!");

    let rf = RegFile::new();
    println!("{}", rf);

    let stack = Stack::new();
    println!("{}", stack);
}
