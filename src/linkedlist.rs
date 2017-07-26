
pub struct LinkedList<T> {
  head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>
}

impl<T> LinkedList<T> {

  pub fn new() -> Self {
    LinkedList { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    // replace following pattern with .map
    // match option { None => None, Some(x) => Some(y) }
    self.head.take().map(|node| {
      let node = *node;
      self.head = node.next;
      node.elem
    })
  }

}
