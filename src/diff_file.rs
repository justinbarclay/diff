use crate::diff::{diff_greedy, Edit, decorate_differences, Operation};
use std::collections::HashMap;


// Compare each line of file one and file two, if one file has more lines then the other then assume all lines
// that are more than the other file are either all inserts or deletes. This isn't an ideal implementation
// of diffing files as it ignores when lines are moved down the text.
pub fn diff_files(file_one: &str, file_two: &str)-> Result<Vec<(i32, HashMap<String, Vec<Edit>>)>, String> {
  let file_one_lines: Vec<&str> = file_one.split('\n').collect();
  let file_two_lines: Vec<&str> = file_two.split('\n').collect();
  let combined_lines = file_one_lines.iter().zip(file_two_lines.iter());

  let mut difference_collection = Vec::new();
  for (line_from_file_1, line_from_file_2) in combined_lines {
    match diff_greedy(line_from_file_1, line_from_file_2) {
      Ok(success) => difference_collection.push(success),
      Err(e) =>  {
        return Err(e)
      },
    };
  }

  if file_one_lines.len() > file_two_lines.len(){
    let remaining = file_one_lines.len() - file_two_lines.len();
    for line in file_one_lines[file_one_lines.len() - remaining .. file_one_lines.len()].iter(){
      let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
      map.insert(String::from("insert"), Vec::new());
      map.insert(String::from("delete"), vec![Edit{edit: Operation::Delete,
                                                   at: 0,
                                                   to: line.len()}]);
      difference_collection.push((line.len() as i32, map))
    }
  } else if file_two_lines.len() > file_one_lines.len(){
    let remaining = file_two_lines.len() - file_one_lines.len();
    for line in file_two_lines[file_two_lines.len() - remaining .. file_two_lines.len()].iter(){
      let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
      map.insert(String::from("delete"), Vec::new());
      map.insert(String::from("insert"), vec![Edit{edit: Operation::Insert,
                                                   at: 0,
                                                   to: line.len()}]);
      difference_collection.push((line.len() as i32, map))
    }
  }
  Ok(difference_collection)
}

// Goes through each line in file_one and file_two and applies decoration to the insertions and deletions in each line
pub fn differences_by_line(file_one: &str, file_two: &str, edits: Vec<(i32, HashMap<String, Vec<Edit>>)>) -> String {

  let file_one_lines: Vec<&str> = file_one.split('\n').collect();
  let file_two_lines: Vec<&str> = file_two.split('\n').collect();

  let combined_lines = file_one_lines.iter().zip(file_two_lines.iter()).zip(edits.iter());
  let mut result = String::new();

  for ((line_from_file_1, line_from_file_2), (number_of_differences, differences)) in combined_lines {
    if *number_of_differences == 0 {
      result.push_str(line_from_file_1);
      result.push('\n');
    } else {
      result.push_str(&decorate_differences(line_from_file_1, "delete", &differences["delete"]));
      result.push('\n');
      result.push_str(&decorate_differences(line_from_file_2, "insert", &differences["insert"]));
      result.push('\n');
    }
  }

  if file_one_lines.len() > file_two_lines.len(){
    let remaining = file_one_lines.len() - file_two_lines.len();
    for (i, line) in file_one_lines[file_one_lines.len() - remaining .. file_one_lines.len()].iter().enumerate(){
      let index = i+remaining;
      result.push_str(&decorate_differences(line, "delete", &edits[index].1["delete"]));
      result.push('\n');
    }
  } else if file_two_lines.len() > file_one_lines.len(){
    let remaining = file_two_lines.len() - file_one_lines.len();
    println!("Remaining {}", remaining);
    for (i, line) in file_two_lines[file_two_lines.len() - remaining .. file_two_lines.len()].iter().enumerate(){
      let index = i+remaining;
      result.push_str(&decorate_differences(line, "insert", &edits[index].1["insert"]));
      result.push('\n');
    }
  }
  result.pop(); // Remove the last new line we added
  result
}
