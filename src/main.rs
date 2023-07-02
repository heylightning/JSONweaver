#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_cors;

mod organization;

use std::error::Error;
use csv;
use organization::fill_json;

use rocket_cors::{ CorsOptions, AllowedOrigins };

#[get("/")]
fn jsonweaver() -> String {
    match read_from_file("./public/organizations.csv") {
        Ok(main_array) => {
            fill_json(main_array)
        } Err(e) => e.to_string()
    }
}

fn read_from_file(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut header_array: Vec<Vec<String>> = Vec::new(); // for headers
    let mut main_array: Vec<Vec<String>> = Vec::new();

    let head = reader.headers()?;
    let listofheaders: Vec<String> = head.iter().map(|h| h.to_string()).collect();

    header_array.push(listofheaders); // array of headers

    for result in reader.records() {
        let record = result?;
        let listofrecord: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        
        main_array.push(listofrecord);
    }
    Ok(main_array)
}

fn main() {

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors().unwrap();

    rocket::ignite().mount("/", routes![jsonweaver]).attach(cors).launch();
}
