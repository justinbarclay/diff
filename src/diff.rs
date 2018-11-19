extern crate time;

use negative_array::NegativeArray;

use std::collections::HashMap;
use std::fmt;
use time::now;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
  Insert,
  Delete,
  Null,
}

/// Display Operation Enum as a String
impl std::fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


#[derive(Debug, Clone, PartialEq)]
/// A Structure for describing change
pub struct Edit {
  edit: Operation,
  at: usize,
  to: usize,
}

/// A helper function for splitting a string into a vector of characters
fn split_string(string: &str) -> Vec<&str> {
  let col: Vec<_> = string.split("").collect();
  let end = col.len() - 1;
  col[1..end].to_vec()
}

/// A recursive based algorithm that traces through a history vector generated by `shortest_edit_sequence`
/// and creates a an edit graph that can transform String 1 to String 2.
// fn generate_edit_graph(first: &str, second: &str, difference: isize, diagonal: isize, history: Vec<NegativeArray>) -> Vec<Edit> {
//   // set constants to match algo
//   let N = first.len() as isize;
//   let M = second.len() as isize;
//   let MAX = N + M as isize;

//   let second_chars = split_string(second);
//   let first_chars = split_string(first);

//   if difference == -1 {
//     return Vec::new();
//   }

//   // Things we will need access to later
//   let op: Edit;
//   let new_diagonal: isize;
//   // Controlling borrowing scope by creating a closure
//   {
//     let ref furthest_path_at_D = if difference % 2 == 0 {
//       &history[(difference + 1) as usize]
//     } else {
//       &history[difference as usize]
//     };
//     // Let's set some state we need access outside of loop
//     let mut best_diagonal = None;
//     let mut bestY = -1;
//     let mut bestX = -1;
//     // Because we're traversing our history. We know as we step back in the history that to get to our current K
//     // it must be through either an insert or a delete. So it must be the K directly
//     // above us or below us.

//     // Let's find which operation, inserting or deleting gets us farther
//     for position in [diagonal - 1, diagonal + 1].iter() {
//       let mut x = furthest_path_at_D[*position];
//       let mut y = x - position;

//       while (0 <= x && x < N)
//         && (0 <= y && y < M)
//         && first_chars[x as usize] == second_chars[y as usize]
//       {
//         x += 1;
//         y += 1;
//       }
//       if x > bestX {
//         best_diagonal = Some(*position);
//         bestX = furthest_path_at_D[*position];
//         bestY = bestX - position;
//       }
//     }

//     new_diagonal = match best_diagonal { // This is ugly we should extract this to a function to hide it
//       Some(diagonal) =>{
//         diagonal
//       },
//       None => {
//         panic!("Oh no, this shouldn't happen at all.");
//       }
//     };
//       op = if new_diagonal == diagonal
//           + 1 {
//       Edit{
//         edit: Operation::Insert,
//         at: bestY as usize,
//         to: bestY as usize
//       }
//     } else {
//       Edit {
//         edit: Operation::Delete,
//         at: bestX as usize,
//         to: bestX as usize
//       }
//     };
//   }

//   let mut editGraph = generate_edit_graph(first, second, difference - 1, new_diagonal, history);
//   editGraph.push(op);

//   editGraph
// }

/// The greedy algorithm for diffing strings. Returns a three-tuple of:
/// 1. The number of differences (either insersts of deletes) that occur from String 1 to String 2
/// 2. The last diagonal (K in Meyer's algorithm) that the diffing algorithm ended on.
/// 3. A copy of the history farthest each diagonal reaches in the algorithm given a difference limit.
fn shortest_edit_sequence(
  first: &str,
  second: &str,
) -> Result<(isize, isize, Vec<NegativeArray>), String> {
  let first_length = first.len() as isize;
  let second_length = second.len() as isize;
  let max = first_length + second_length;

  let second_chars = split_string(second);
  let first_chars = split_string(first);

  let mut traversal_history = NegativeArray::new(max as isize);
  traversal_history[1] = 0;
  let mut history: Vec<NegativeArray> = vec![NegativeArray::new(0); (max + 2) as usize]; // history needs to be able to describe differences from [0..Max +1] so the history needs to hold 2 more elements above max, if I'm smarter about the history data structure we should be able to get it to be much shorter

  for d in 0..=max as isize {
    let mut diagonal = -d;
    while diagonal <= d {
      let mut x: isize;
      let mut y: isize;

      let down = diagonal == -d
        || (diagonal != d && traversal_history[diagonal - 1] < traversal_history[diagonal + 1]);
      x = if down {
        traversal_history[diagonal + 1]
      } else {
        traversal_history[diagonal - 1] + 1
      };
      y = x - diagonal;

      while (0 <= x && x < first_length)
        && (0 <= y && y < second_length)
        && first_chars[x as usize] == second_chars[y as usize]
      {
        x += 1;
        y += 1;
      }

      traversal_history[diagonal] = x;

      if x >= first_length && y >= second_length {
        let final_d = if d % 2 == 0 { d + 1 } else { d };
        history[final_d as usize] = traversal_history.clone();
        return Ok((d, diagonal, history));
      }

      diagonal += 2;
    }

    // We can ignore pushing even slices into the history
    // becuase only an even K will overwrite an even K
    if d % 2 == 1 {
      history[d as usize] = traversal_history.clone();
    }
  }
  Err("Failed To Find Shortest Edit Sequence".to_string())
}

/// A loop based algorithm that traces through a history vector generated by `shortest_edit_sequence`
/// and creates a an edit graph that can transform String 1 to String 2.
fn generate_edit_graph_loop(
  first: &str,
  second: &str,
  diff: isize,
  original_diagonal: isize,
  history: Vec<NegativeArray>,
) -> Result<Vec<Edit>, String> {
  // set constants to match algo
  let first_length = first.len() as isize;
  let second_length = second.len() as isize;

  let second_chars = split_string(second);
  let first_chars = split_string(first);

  // Things we will need access to later
  let mut difference = diff;
  let mut edit_graph = Vec::with_capacity(difference as usize);
  let mut op: Edit;
  let mut new_diagonal: isize;
  let mut diagonal = original_diagonal;
  // Controlling borrowing scope by creating a closure
  while difference > -1 {
    let furthest_path_at_d = if difference % 2 == 0 {
      &history[(difference + 1) as usize]
    } else {
      &history[difference as usize]
    };
    // Let's set some state we need access outside of loop
    let mut best_diagonal = None;
    let mut best_y = -1;
    let mut best_x = -1;
    // Because we're traversing our history. We know as we step back in the history that to get to our current K
    // it must be through either an insert or a delete. So it must be the K directly
    // above us or below us.

    // Let's find which operation, inserting or deleting gets us farther
    for position in [diagonal - 1, diagonal + 1].iter() {
      let mut x = furthest_path_at_d[*position];
      let mut y = x - position;

      while (0 <= x && x < first_length)
        && (0 <= y && y < second_length)
        && first_chars[x as usize] == second_chars[y as usize]
      {
        x += 1;
        y += 1;
      }
      if x > best_x {
        best_diagonal = Some(*position);
        best_x = furthest_path_at_d[*position];
        best_y = best_x - position;
      }
    }

    new_diagonal = match best_diagonal {
      // This is ugly we should extract this to a function to hide it
      Some(diagonal) => diagonal,
      None => {
        return Err("Failed to Generate Edit Graph".to_string());
      }
    };
    op = if new_diagonal == diagonal + 1 {
      Edit {
        edit: Operation::Insert,
        at: best_y as usize,
        to: best_y as usize,
      }
    } else {
      Edit {
        edit: Operation::Delete,
        at: best_x as usize,
        to: best_x as usize,
      }
    };
    edit_graph.push(op);

    difference -= 1;
    diagonal = new_diagonal;
  }

  edit_graph.reverse();
  Ok(edit_graph)
}

/// Simplifies an edit graph into a series of operation that describes series of deletes
/// or inserts.
fn simplify_edit_graph(edit_graph: Vec<Edit>) -> HashMap<String, Vec<Edit>> {
  let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
  map.insert(String::from("insert"), Vec::new());
  map.insert(String::from("delete"), Vec::new());

  let mut previous_edit = Edit {
    edit: Operation::Null,
    at: 0,
    to: 0,
  };

  for edit in edit_graph {
    // If previous
    let mut operation_string = match edit.edit {
      Operation::Insert => String::from("insert"),
      Operation::Delete => String::from("delete"),
      Operation::Null => String::from("null"),
    };
    if previous_edit.edit == edit.edit && edit.at > 0 && previous_edit.at == edit.at - 1 {
      let mut edit_range = map
        .get_mut(&operation_string)
        .unwrap()
        .pop()
        .unwrap()
        .clone();
      edit_range.to = edit.at;
      map.get_mut(&operation_string).unwrap().push(edit_range);
    } else {
      map.get_mut(&operation_string).unwrap().push(edit.clone());
    }
    previous_edit = edit;
  }
  map
}

/// A fucntion to colour the insert and delete sections of a given string.
pub fn decorate_differences(string: &str, edit_type: &str, edits: &[Edit]) -> String {
  let red = "\x1b[31m";
  let end_colour = "\x1b[0m";
  let green = "\x1b[32m";

  let colour = if edit_type == "insert" { green } else { red };
  let mut response = String::new();
  if edits.is_empty() {
    return string.to_string();
  }

  let mut edits_1 = edits.to_vec();
  edits_1.reverse();
  let mut maybe_edit = edits_1.pop();

  for (index, character) in string.chars().enumerate() {
    match maybe_edit.clone() {
      Some(edit) => {
        if index == edit.at as usize {
          response.push_str(colour);
        }
        response.push(character);
        if index == edit.to as usize {
          response.push_str(end_colour);
          maybe_edit = edits_1.pop();
        }
      }
      None => response.push(character),
    }
  }
  response
}

/// The Meyer's greedy string diffing alorithms. Returns a HahsMap of Edit
/// describing: what positions in the first string that need to be deleted to match String 2,
/// under the `delete` key, and what positions in the second string need to be inserted into the first string
/// to trnasform String 1 into String 2, under the `insert` key
pub fn diff_greedy(first: &str, second: &str) -> Result<HashMap<String, Vec<Edit>>, String> {
  // let mut start = time::now();
  if first.len() == 0 && second.len() > 0{
    let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
    map.insert(String::from("insert"), vec![Edit{edit: Operation::Insert, at: 0, to: second.len()}; 1]);
    map.insert(String::from("delete"), Vec::new());
    Ok(map)
  } else if first.len() > 0 && second.len() == 0{
    let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
    map.insert(String::from("delete"), vec![Edit{edit: Operation::Delete , at: 0, to: first.len()}; 1]);
    map.insert(String::from("insert"), Vec::new());
    Ok(map)
  } else {
    let (difference, diagonal, history) = shortest_edit_sequence(first, second)?;
    // println!("{:?}", history);
    // let mut finish = time::now();
    // println!("{:}", finish - start);

    // start = time::now();
    let edit_graph = generate_edit_graph_loop(first, second, difference - 1, diagonal, history)?;
    // println!("{:?}", edit_graph);
    // finish = time::now();
    // println!("{:}", finish - start);

    // start = time::now();
    let simple_edit_graph = simplify_edit_graph(edit_graph);
    // finish = time::now();
    // println!("{:}", finish - start);
    Ok(simple_edit_graph)
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  #[test]
  fn split_string_hello() {
    let split = split_string("Hello");
    let pre_split = vec!["H", "e", "l", "l", "o"];
    assert!(pre_split.len() == split.len());
    assert_eq!(split, pre_split);
  }

  #[test]
  fn short_edit_sequence() {
    let history = vec![
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 5,
        arr: [-1, -1, -1, -1, 2, 1, 0, -1, -1, -1, -1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
    ];
    let result = shortest_edit_sequence("H\n", "Hi\n").unwrap();
    assert_eq!(result.0, 1);
    assert_eq!(result.1, -1);
    assert_eq!(result.2, history);
  }

  #[test]
  fn short_edit_sequence_where_they_are_the_same() {
    let history = vec![
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray {
        max: 10,
        arr: [
          -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ].to_vec(),
      },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
      NegativeArray { max: 0, arr: [-1].to_vec() },
    ];
    let result = shortest_edit_sequence("Hello", "Hello").unwrap();
    assert_eq!(result.0, 0);
    assert_eq!(result.1, 0);
    assert_eq!(result.2, history);
  }

  #[test]
  fn short_edit_sequence_where_nothing_matches() {
    let result = shortest_edit_sequence(&"Hze", &"Nod").unwrap();
    assert_eq!(result.0, 6);
    assert_eq!(result.1, 0);
  }

    #[test]
  fn short_edit_sequence_for_empty_strings() {
    let result = shortest_edit_sequence(&"", &"1").unwrap();
    assert_eq!(result.0, 1);
    assert_eq!(result.1, -1);
  }

  #[test]
  fn gen_edit_graph() {
    let history = vec![
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 5,
        arr: [-1, -1, -1, -1, 2, 1, 0, -1, -1, -1, -1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
    ];
    let edit_graph = vec![Edit {
      edit: Operation::Insert,
      at: 1,
      to: 1,
    }];
    let result = generate_edit_graph_loop("H\n", "Hi\n", 0, -1, history).unwrap();
    assert_eq!(edit_graph, result);
  }

  #[test]
  fn short_edit_sequence_without_newlines() {
    let history = vec![
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 3,
        arr: [-1, -1, 1, 1, 0, -1, -1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
    ];
    let result = shortest_edit_sequence("H", "Hi").unwrap();
    assert_eq!(result.0, 1);
    assert_eq!(result.1, -1);
    assert_eq!(result.2, history);
  }

  #[test]
  fn gen_edit_graph_without_newlines() {
    let history = vec![
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 3,
        arr: [-1, -1, 1, 1, 0, -1, -1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
      NegativeArray {
        max: 0,
        arr: [-1].to_vec(),
      },
    ];
    let edit_graph = vec![Edit {
      edit: Operation::Insert,
      at: 1,
      to: 1,
    }];
    let result = generate_edit_graph_loop("H\n", "Hi\n", 0, -1, history).unwrap();
    assert_eq!(edit_graph, result);
  }

  #[test]
  fn greedy_diff(){
    let mut expected_differences: HashMap<String, Vec<Edit>> = HashMap::new();
    expected_differences.insert(String::from("insert"), Vec::new());
    expected_differences.insert(String::from("delete"), Vec::new());
    expected_differences.get_mut("insert").unwrap().push(Edit { edit: Operation::Insert, at: 1, to: 1 });
    let (number_of_differences, differences) = diff_greedy("H", "Hi").unwrap();
    assert_eq!(number_of_differences, 1);
    assert_eq!(differences, expected_differences);
  }
}
