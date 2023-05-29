#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::insert_into;
pub mod schema;

mod grpc_models;
use dotenv::{dotenv, Error};
use std::env;
use std::collections::HashMap;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::debug;
use log::error;
use log::info;
use log::warn;

pub fn calculate_sum(numbers: &[i32]) -> Result<i32, String>{
    let sum = numbers.iter().sum();
    Ok(sum)
}

fn main() {

    info!("{}", "Error info");

    let mut subs = HashMap::new();
    subs.insert(String::from("LGR"), 100000);

    let numbers = vec![1,2,3,4];
    match calculate_sum(&numbers){
        Ok(sum) => print!("Sum: {}", sum),
        Err(err) => eprintln!("Error: {}", err)
    }
}
