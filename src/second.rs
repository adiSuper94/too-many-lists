use std::mem::replace;

pub struct List {
    head: Link,
}
struct Node {
    elem: u32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: u32) {
        let node: Node = Node {
            elem,
            next: replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<u32> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
