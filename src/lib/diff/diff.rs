mod array;
use array::NegativeArray;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

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

fn generate_edit_graph(first: &str, second: &str, difference: isize, k: isize, history: Vec<NegativeArray>) -> Vec<Edit> {
  // set constants to match algo
  let N = first.len() as isize;
  let M = second.len() as isize;
  let MAX = N + M as isize;

  let second_chars = split_string(second);
  let first_chars = split_string(first);

  if difference == -1 {
    return Vec::new();
  }

  // Things we will need access to later
  let op: Edit;
  let newK: isize;
  // Controlling borrowing scope by creating a closure
  {
    let ref furthest_path_at_D = if difference % 2 == 0 {
      &history[(difference + 1) as usize]
    } else {
      &history[difference as usize]
    };
    // Let's set some state we need access outside of loop
    let mut bestK = None;
    let mut bestY = -1;
    let mut bestX = -1;
    // Because we're traversing our history. We know as we step back in the history that to get to our current K
    // it must be through either an insert or a delete. So it must be the K directly
    // above us or below us.

    // Let's find which operation, inserting or deleting gets us farther
    for position in [k - 1, k + 1].iter() {
      let mut x = furthest_path_at_D[*position];
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

    newK = match bestK { // This is ugly we should extract this to a function to hide it
      Some(k) =>{
        k
      },
      None => {
        panic!("Oh no, this shouldn't happen at all.");
      }
    };
    op = if newK == k + 1 {
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
  }

  let mut editGraph = generate_edit_graph(first, second, difference - 1, newK, history);
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

  for edit in editGraph {
    // If previous
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
      editRange.to = edit.at;
      map.get_mut(&operation_string).unwrap().push(editRange);
    } else {
      map.get_mut(&operation_string).unwrap().push(edit.clone());
    }
    previousEdit = edit;
  }
  map
}

pub fn shortest_edit_sequence(first: &str, second: &str) -> Result<(isize, isize, Vec<NegativeArray>), String> {
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
        return Ok((d, k, history));
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

  let mut edits_1 = edits.clone().to_vec();
  edits_1.reverse();
  let mut maybe_edit = edits_1.pop();

  for (index, character) in string.chars().enumerate() {
    match maybe_edit.clone() {
      Some(edit) => {
        if index == edit.at as usize {
          response.push_str(COLOUR);
        }
        response.push(character);
        if index == edit.to as usize {
          response.push_str(ENDCOLOUR);
          maybe_edit = edits_1.pop();
        }
      },
      None => response.push(character)
    }
  }
  return response;
}

pub fn diff_greedy(first: &str, second: &str) -> Result<HashMap<String, Vec<Edit>>, String> {
  let (difference, k, history) = match shortest_edit_sequence(first, second) {
    Ok(success) => success,
    Err(e) => {
      return Err("What the hell".to_string())
    },
  };
  let editGraph = generate_edit_graph(first, second, difference - 1, k, history);
  let simpleEditGraph = simplify_edit_graph(editGraph);
  Ok(simpleEditGraph)
}
