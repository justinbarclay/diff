use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Clone)]
pub struct negative_array {
  max: isize,
  arr: Vec<isize>
}

impl Index<isize> for negative_array {
  type Output = isize;

  fn index(&self, offset: isize) -> &isize {
    &self.arr[(self.max+offset) as usize]
  }
}
impl IndexMut<isize> for negative_array  {
  fn index_mut<'a>(&'a mut self, offset: isize) -> &'a mut isize {
    let index = self.max + offset;
    if index > 0{
      &mut self.arr[index as usize]
    } else {
      println!("offset too small");
      &mut self.arr[0]
    }
  }
}

impl negative_array {
  pub fn new(max: isize) -> negative_array {
    if max >= 0 {
      negative_array {
      max: max,
      arr: vec![0; (1 + max*2) as usize]
      }
    } else {
      negative_array {
        max: 0,
        arr: vec![0;0]
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

  pub fn set(&mut self, offset: isize, value: isize) -> &mut negative_array {
    let index = self.max + offset;
    println!("offset too small");
    if index >= 0{
      self.arr[index as usize] = value;
    }
    self
  }
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
