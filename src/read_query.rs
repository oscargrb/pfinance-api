use std::{fs, str::from_utf8};

pub fn read_query(query_name: &str) -> String {

    println!("{}", "./sql_db/".to_owned() + query_name + ".sql");

    let find_query = fs::read("./src/sql_db/".to_owned() + query_name + ".sql").expect("Error");

    let decode_query = from_utf8(&find_query).expect("Error");

    println!("{}", decode_query);

    return  decode_query.to_owned()
}