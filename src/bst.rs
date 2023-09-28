#[derive(Debug, PartialEq)]
enum BinarySearchTree<T: Ord> {
  Empty,
  Node {
    value: T,
    left: Box<BinarySearchTree<T>>,
    right: Box<BinarySearchTree<T>>,
  },
}

impl<T: Ord> BinarySearchTree<T> {
  pub fn insert(&mut self, value: T) {
    match self {
      BinarySearchTree::Empty => {
        *self = BinarySearchTree::Node {
          value,
          left: Box::new(BinarySearchTree::Empty),
          right: Box::new(BinarySearchTree::Empty),
        }
      }
      BinarySearchTree::Node {
        value: v,
        left,
        right,
      } => {
        if value < *v {
          left.insert(value);
        } else {
          right.insert(value);
        }
      }
    }
  }

  pub fn search(&self, value: T) -> bool {
    match self {
      BinarySearchTree::Empty => false,
      BinarySearchTree::Node {
        value: v,
        left,
        right,
      } => {
        if value == *v {
          true
        } else if value < *v {
          left.search(value)
        } else {
          right.search(value)
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_should_correctly_insert_values() {
    use super::BinarySearchTree;

    let mut bst = BinarySearchTree::Empty;
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    assert_eq!(
      bst,
      BinarySearchTree::Node {
        value: 5,
        left: Box::new(BinarySearchTree::Node {
          value: 3,
          left: Box::new(BinarySearchTree::Node {
            value: 1,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Empty),
          }),
          right: Box::new(BinarySearchTree::Node {
            value: 4,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Empty),
          }),
        }),
        right: Box::new(BinarySearchTree::Node {
          value: 7,
          left: Box::new(BinarySearchTree::Node {
            value: 6,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Empty),
          }),
          right: Box::new(BinarySearchTree::Node {
            value: 8,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Empty),
          }),
        }),
      }
    );
  }

  #[test]
  fn it_should_search_bst() {
    use super::BinarySearchTree;

    let mut bst = BinarySearchTree::Empty;
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    assert_eq!(bst.search(5), true);
    assert_eq!(bst.search(3), true);
    assert_eq!(bst.search(7), true);
    assert_eq!(bst.search(1), true);
    assert_eq!(bst.search(4), true);
    assert_eq!(bst.search(6), true);
    assert_eq!(bst.search(8), true);
    assert_eq!(bst.search(9), false);
  }
}
