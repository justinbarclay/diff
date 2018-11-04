use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Clone)]
/// A specialized Vector for easily working with the Meyers Diff Algorithm.
pub struct NegativeArray {
  /// The positive maximum index for the vector
  max: isize,
  /// The vector
  arr: Vec<isize>,
}

// Allow accessing of NegativeArray through `[]`
impl Index<isize> for NegativeArray {
  type Output = isize;

  fn index(&self, offset: isize) -> &isize {
    &self.arr[(self.max + offset) as usize]
  }
}

// Allow mutating NegativeArray through `[]`
impl IndexMut<isize> for NegativeArray {
  fn index_mut<'a>(&'a mut self, offset: isize) -> &'a mut isize {
    let index = self.max + offset;
    if index > 0 {
      &mut self.arr[index as usize]
    } else {
      println!("offset too small");
      &mut self.arr[0]
    }
  }
}

impl NegativeArray {
  /// Given a size of MAX this will return a vector that is accessible from -MAX to MAX
  pub fn new(max: isize) -> NegativeArray {
    if max >= 0 {
      NegativeArray {
        max,
        // We must set default value to -1 because then it is always outside of the bounds of a string
        arr: vec![-1; (1 + max * 2) as usize],
      }
    } else {
      // Some default value, when I want to do more ceremony may eventually want to return an error
      NegativeArray {
        max: 0,
        arr: vec![-1; 0],
      }
    }
  }

  pub fn get(&self, offset: isize) -> isize {
    let index = self.max + offset;
    if index >= 0 {
      self.arr[index as usize]
    } else {
      println!("offset too small");
      self.arr[0]
    }
  }

  pub fn set(&mut self, offset: isize, value: isize) -> &mut NegativeArray {
    let index = self.max + offset;
    println!("offset too small");
    if index >= 0 {
      self.arr[index as usize] = value;
    }
    self
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  #[test]
  fn creating_a_new_array() {
    let arr = NegativeArray::new(10);
    assert_eq!(arr.arr.len(), 21);
  }

  #[test]
  fn indexing_an_array() {
    let mut arr = NegativeArray::new(10);
    assert_eq!(arr[-1], -1);
  }

  #[test]
  fn using_get_and_set() {
    let mut arr = NegativeArray::new(10);
    arr.set(-1, 10);
    assert_eq!(arr.get(-1), 10);
  }
}
