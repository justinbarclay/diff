extern crate clap;
use clap::{App, Arg};
use std::fs;

fn validate_files(file_one: &str, file_two: &str) -> Result<Vec<String>, std::io::Error>{
  let mut files = Vec::new();

  let fOne = match fs::read_to_string(file_one){
    Ok(file) => files.push(file),
    Err(err) => return Err(err)
  };

  let fTwo = match fs::read_to_string(file_two){
    Ok(file) => files.push(file),
    Err(err) => return Err(err)
  };
  Ok(files)
}

fn main() {
  let matches = App::new("Rdiff")
    .version("0.1")
    .author("Justin Barclay <justincbarclay@gmail.com")
    .about("Compares two files")
    .arg(
      Arg::with_name("FILE1")
        .help("Original file")
        .required(true)
        .index(1),
    ).arg(
      Arg::with_name("FILE2")
        .help("File to look for differences from FILE1")
        .required(true)
        .index(2),
    ).get_matches();

  let file_one = matches.value_of("FILE1").unwrap();
  let file_two = matches.value_of("FILE2").unwrap();
  println!("We're going to see how {file_2} has changed from {file_1}", file_1 = file_one , file_2=file_two);

  match validate_files(file_one, file_two){
    Ok(files) => println!("Able to read files"),
    Err(error) => {
      println!("Unable to open file_one {:?}", error)
    }
  };
  println!("Hello, world!");
}
