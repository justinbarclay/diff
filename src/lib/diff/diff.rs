mod array;
use array::negative_array;

fn generate_k_values(max: isize) -> Vec<isize>{
  let mut collection = Vec::new();
  let mut k = -max;

  while k <= max {
    collection.push(k);
    k = k+2;
  }
  return collection
}

fn split_string(string: &str)-> Vec<&str>{
  let col: Vec<_> = string.split("").collect();
  let end = col.len() - 1;
  col[1..end].to_vec()
}

pub fn shortest_edit_sequence(first: &str, second: &str) -> bool{

  let N = first.len() as isize;
  let M = second.len() as isize;
  let second_chars = split_string(second);
  let first_chars = split_string(first);
  
  let MAX = N + M;

  let mut v = negative_array::new(MAX as isize);
  let mut history = Vec::new();

  for d in 0..MAX as isize {
    let mut k = -d;
    // let iterator = generate_k_values(d);

    while k <= d {
      let mut x: isize;
      let mut y: isize;

      let down = k == -d || (k != d && v[k - 1] < v[k + 1]);
      x = if down {v[k + 1]} else { v[k - 1] + 1 };

      y = x - k;


      while (0 <= x && x < N) && (0 <= y && y < M) && first_chars[x as usize] == second_chars[y as usize]{

        x += 1;
        y += 1;
      }

      v[k] = x;
      if x >= N && y >= M{
        let findalD = if d % 2 == 0 {d + 1} else { d};
        history.push(v.clone());

        println!("D: {}", d);
        return true
      }
      k += 2;
    }
    // println!("D: {}", d)
  }
  return false;
}


#[cfg(tests)]
mod tests{
  #[test]
  fn creating_a_new_array(){
    let arr = negative_array::new(10);
    assert_eq!(arr.arr.len(), 21);
  }

  #[test]
  fn indexing_an_array(){
    let mut arr = negative_array::new(10);
    assert_eq!(arr[-1], -1);
  }

  #[test]
  fn using_get_and_set(){
    let mut arr = negative_array::new(10);
    arr.set(-1, 10);
    assert_eq!(arr.get(-1), 10);
  }
}
