use std::fmt;

type ListIndex = u32;

#[derive(Clone)]
enum ListNode<T: Clone> {
  Empty,
  Used {
    data: T,
    next: Option<Box<ListNode<T>>>,
  }
}

/// Small Linked List Array with Binary Search
struct List<T: Clone> {
  /// Stores first, last, 1/2, 1/4, 1/8, etc, as needed
  indexes: Vec<ListIndex>,
  /// Stores the actual data
  nodes: Vec<ListNode<T>>,
}

impl<T: Clone> List<T> {
  pub fn new() -> Self {
    todo!();
  }
  pub fn with_capacity(capacity: ListIndex) -> Self {
    let mut list = List {
      indexes: vec![],
      nodes: vec![ListNode::Empty; capacity as usize]
    };
    list
  }
  pub fn push(&mut self, item: T) {
    todo!();
  }
  pub fn push_back(&mut self, item: T) {
    todo!();
  }
  pub fn set(&mut self, index: ListIndex, item: T) {
    todo!();
  }
  pub fn get(&self, index: ListIndex) -> Option<&T> {
    todo!();
  }
  pub fn remove(&mut self, index: ListIndex) {
    todo!();
  }
  pub fn map<U: Clone>(&self, f: impl Fn(&T) -> U) -> List<U> {
    todo!()
  }
  pub fn join(&self, sep: &str) -> String
  where
    T: fmt::Display,
  {
    let mut iter = self.iter();
    let first = match iter.next() {
      Some(item) => item,
      None => return String::new(),
    };
    let string = first.to_string();
    for item in iter {
      string.push_str(sep);
      string.push_str(&item.to_string());
    }
    string
  }
}

impl fmt::Debug for List<u32> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // if f.alternate() // pretty
    write!(f, "[{}]", self.map(|item| format!("{item:?}")).join(", "))
  }
}

macro_rules! count_macro_params {
  () => { 0 };
  ($first:tt $($rest:tt)*) => { 1 + count_macro_params!($($rest)*) };
}

macro_rules! list {
  ($($item:expr),*) => {{
    // let list = List::new();
    let mut list = List::with_capacity(count_macro_params!($($item)*));
    $( list.push($item); )*
    list
  }}
}

fn main() {
  let list = list![1, 2, 3, 4, 5];
  println!("{:?}", list);
}
