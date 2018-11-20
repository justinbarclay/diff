use diff::{diff_greedy, Edit, decorate_differences, Operation};

pub fn diff_files(file_one: &str, file_two: &str)-> String{
  let file_one_lines: Vec<&str> = file_one.split("\n").collect();
  let file_two_lines: Vec<&str> = file_two.split("\n").collect();
  let combined_lines = file_one_lines.iter().zip(file_two_lines.iter());

  let mut result = String::new();

  for (line_from_file_1, line_from_file_2) in combined_lines {
    let (number_of_differences, differences) = match diff_greedy(line_from_file_1, line_from_file_2) {
      Ok(success) => success,
      Err(e) =>  {
        return e
      },
    };
    if number_of_differences == 0 {
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
    for line in file_one_lines[remaining .. file_one_lines.len()].iter(){
      result.push_str(&decorate_differences(line, "delete", &[ Edit{edit: Operation::Delete,
                                                                    at: 0,
                                                                    to: line.len() }]));
    }

  } else if file_two_lines.len() > file_one_lines.len(){
    let remaining = file_two_lines.len() - file_one_lines.len();
    for line in file_two_lines[remaining .. file_two_lines.len()].iter(){
      result.push_str(&decorate_differences(line, "insert", &[ Edit{edit: Operation::Insert,
                                                                    at: 0,
                                                                    to: line.len() }]));
    }
  }

  result
}
