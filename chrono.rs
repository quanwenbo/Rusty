use chrono::prelude::*;
use chrono::{Local, DateTime, TimeZone};
use std::env;
use std::fs;



fn main(){

    let dt: DateTime<Local> = Local::now();

    let dt_print = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    let args:Vec<String> = env::args().collect();

    println!("{}",dt_print);

    let query = &args[1];alloca 
    let filename = &args[2];


    println!("args[0] is {}", &args[0]);
    println!("Searching for {}", query);
    println!("In file {}", filename);


    // println!("{:?}",args);



    

    let contents = fs::read_to_string(filename).expect("hahahah");
    println!("With text there is a poem :\n{}", contents);
}