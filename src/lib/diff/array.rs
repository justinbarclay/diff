use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Clone)]
pub struct NegativeArray {
  max: isize,
  arr: Vec<isize>,
}

impl Index<isize> for NegativeArray {
  type Output = isize;

  fn index(&self, offset: isize) -> &isize {
    &self.arr[(self.max + offset) as usize]
  }
}

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
  pub fn new(max: isize) -> NegativeArray {
    if max >= 0 {
      NegativeArray {
        max: max,
        arr: vec![-1; (1 + max * 2) as usize],
      }
    } else {
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

#[cfg(tests)]
mod tests {
  #[test]
  fn creating_a_new_array() {
    let arr = negative_array::new(10);
    assert_eq!(arr.arr.len(), 21);
  }

  #[test]
  fn indexing_an_array() {
    let mut arr = negative_array::new(10);
    assert_eq!(arr[-1], 0);
  }

  #[test]
  fn using_get_and_set() {
    let mut arr = negative_array::new(10);
    arr.set(-1, 10);
    assert_eq!(arr.get(-1), 10);
  }
}
