extern crate clap;
extern crate diff;

use diff::{shortest_edit_sequence};
use clap::{App, Arg};
use std::fs;

fn validate_files(file_one: &str, file_two: &str) -> Result<Vec<String>, std::io::Error>{
  let mut files = Vec::new();

  match fs::read_to_string(file_one){
    Ok(file) => files.push(file),
    Err(err) => return Err(err)
  };

  match fs::read_to_string(file_two){
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
      Arg::with_name("ALGORITHM")
        .short("a")
        .long("algo")
        .value_name("algo")
        .help("Sets the algorithm type to the \"greedy\" or \"linear\" version Myer's diff algorithm")
        .required(false))
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
  let algo = matches.value_of("ALGORITHM").unwrap_or("greedy");

  let files = match validate_files(file_one, file_two){
    Ok(files) => {
      files
    },
    Err(error) => {
      println!("Unable to open file_one {:?}", error);
      return
    }
  };
  println!("{}", files.len());

  let success ={
    // let _guard = flame::start_guard("cpu-heavy calculation");
    shortest_edit_sequence(&files[0], &files[1])
  };
  // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
  println!("Success? {}", success);
}
