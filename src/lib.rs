#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

mod filesystem;
mod blobstorage;

pub use filesystem::run;