mod array;
use array::NegativeArray;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct differences {
  pub difference: isize,
  pub k: isize,
  pub history: Vec<NegativeArray>,
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
  Insert,
  Delete,
  Null,
}

impl std::fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}
#[derive(Debug, Clone)]
pub struct Edit {
  edit: Operation,
  at: usize,
  to: usize,
}

fn generate_k_values(max: isize) -> Vec<isize> {
  let mut collection = Vec::new();
  let mut k = -max;

  while k <= max {
    collection.push(k);
    k = k + 2;
  }
  collection
}

fn split_string(string: &str) -> Vec<&str> {
  let col: Vec<_> = string.split("").collect();
  let end = col.len() - 1;
  col[1..end].to_vec()
}

fn generate_edit_graph(first: &str, second: &str, diff: &mut differences) -> Vec<Edit> {
  // set constants to match algo
  let N = first.len() as isize;
  let M = second.len() as isize;
  let MAX = N + M as isize;

  let second_chars = split_string(second);
  let first_chars = split_string(first);

  if diff.difference == -1 {
    return Vec::new();
  }

  let op: Edit;
  // Controlling borrowing scope by creating a closure
  {
    let ref furthest_path_at_D = if diff.difference % 2 == 0 {
      &diff.history[(diff.difference + 1) as usize]
    } else {
      &diff.history[diff.difference as usize]
    };

    // Let's set some state we need access outside of loop
    let mut bestK = None;
    let mut bestY = -1;
    let mut bestX = -1;
    // Because we're traversing our history. We know as we step back in the history that to get to our current K
    // it must be through either an insert or a delete. So it must be the K directly
    // above us or below us.

    // Let's find which operation, inserting or deleting gets us farther
    println!("K={:?}", diff.k);
    println!("difference={:?}", diff.difference);
    println!("{:?}", furthest_path_at_D);
    println!("{:?}", furthest_path_at_D[diff.k]);
    for position in [diff.k - 1, diff.k + 1].iter() {
      let mut x = furthest_path_at_D.get(*position);
      let mut y = x - position;

      while (0 <= x && x < N)
        && (0 <= y && y < M)
        && first_chars[x as usize] == second_chars[y as usize]
      {
        x += 1;
        y += 1;
      }
      if x > bestX {
        bestK = Some(*position);
        bestX = furthest_path_at_D.get(*position);
        bestY = bestX - position;
      }
    }

    let newK = match bestK { // This is ugly we should extract this to a function to hide it
      Some(k) =>{
        k
      },
      None => {
        panic!("Oh no, this shouldn't happen at all.");
      }
    };
    println!("newK= {:?}", newK);
    println!("X= {}", furthest_path_at_D.get(newK));
    println!("y= {}", furthest_path_at_D.get(newK) - newK);
    op = if newK == diff.k + 1 {
      Edit{
        edit: Operation::Insert,
        at: bestY as usize,
        to: bestY as usize
      }
    } else {
      Edit {
        edit: Operation::Delete,
        at: bestX as usize,
        to: bestX as usize
      }
    };

    diff.history.deref();
    diff.k = newK;
    diff.difference -= 1;
  }

  let mut editGraph = generate_edit_graph(first, second, diff);
  editGraph.push(op);

  editGraph
}

// function concatEditGraph(editGraph){
fn simplify_edit_graph(editGraph: Vec<Edit>) -> HashMap<String, Vec<Edit>> {
  // simplifies an edit graph into a series of operation that describes ranges of deletes
  // or inserts
  let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
  map.insert(String::from("insert"), Vec::new());
  map.insert(String::from("delete"), Vec::new());

  let mut previousEdit = Edit {
    edit: Operation::Null,
    at: 0,
    to: 0,
  };
  println!{"EditGraph \n{:?}", editGraph};
  for edit in editGraph {
    // If previous
    println!("{:?}", edit);
    let mut operation_string = match edit.edit {
      Operation::Insert => String::from("insert"),
      Operation::Delete => String::from("delete"),
      Operation::Null => String::from("null"),
    };
    if previousEdit.edit == edit.edit && edit.at > 0 && previousEdit.at == edit.at - 1 {
      let mut editRange = map
        .get_mut(&operation_string)
        .unwrap()
        .pop()
        .unwrap()
        .clone();
      editRange.to = edit.at.clone();
      map.get_mut(&operation_string).unwrap().push(editRange);
    } else {
      map.get_mut(&operation_string).unwrap().push(edit.clone());
    }
    previousEdit = edit;
  }
  map
}



pub fn shortest_edit_sequence(first: &str, second: &str) -> Result<differences, String> {
  let N = first.len() as isize;
  let M = second.len() as isize;
  let MAX = N + M;

  let second_chars = split_string(second);
  let first_chars = split_string(first);

  let mut v = NegativeArray::new(MAX as isize);
  v[1] = 0;
  let mut history: Vec<NegativeArray> = vec![NegativeArray::new(0); MAX as usize];

  for d in 0..MAX as isize {
    let mut k = -d;

    while k <= d {
      let mut x: isize;
      let mut y: isize;

      let down = k == -d || (k != d && v[k - 1] < v[k + 1]);
      x = if down { v[k + 1] } else { v[k - 1] + 1 };
      y = x - k;

      while (0 <= x && x < N)
        && (0 <= y && y < M)
        && first_chars[x as usize] == second_chars[y as usize]
      {
        x += 1;
        y += 1;
      }

      v[k] = x;
      if x >= N && y >= M {
        let finalD = if d % 2 == 0 {d+1} else {d};
        history[finalD as usize] = v.clone();
        return Ok(differences {
          difference: d,
          k: k,
          history: history,
        });
      }
      k += 2;
    }

    // We can ignore pushing even slices into the history
    // becuase only an even K will overwrite an even K
    if d % 2 == 1 {
      history[d as usize] = v.clone();
    }
  }
  return Err("What the hell?".to_string());
}

pub fn print_differences(string: &str, edit_type: &str, edits: &[Edit]) -> String{
  let RED = "\x1b[31m";
  let ENDCOLOUR = "\x1b[0m";
  let GREEN = "\x1b[32m";

  let COLOUR = if edit_type == "insert" {GREEN}  else {RED};
  let mut response = String::new();

  if edits.len() == 0 {
    return string.to_string();
  }
  let mut offset = 0;
  let mut copy = string.to_string();
  for edit in edits {
    let end_colour_pos = offset + edit.to + 1;
    let start_colour_pos = offset + edit.at;
    println!("Start pos {:?}", start_colour_pos);
    if (copy.len() > end_colour_pos){
      copy.insert_str(end_colour_pos, ENDCOLOUR);
    } else {
      copy.push_str(ENDCOLOUR);
    }
    copy.insert_str(start_colour_pos, COLOUR);
    offset += 2;
  }
  return copy;
}

pub fn diff_greedy(first: &str, second: &str) -> Result<HashMap<String, Vec<Edit>>, String> {
  let mut differences = match shortest_edit_sequence(first, second) {
    Ok(success) => success,
    Err(e) => {
      return Err("What the hell".to_string())
    },
  };
  let mut new_differences = differences.clone();
  new_differences.difference -= 1;
  let editGraph = generate_edit_graph(first, second, &mut new_differences);
  let simpleEditGraph = simplify_edit_graph(editGraph);
  println!("{:?}", simpleEditGraph);
  Ok(simpleEditGraph)
}
