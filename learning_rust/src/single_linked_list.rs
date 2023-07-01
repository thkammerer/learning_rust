type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T: std::fmt::Debug + std::marker::Copy> {
    pub element: T,
    pub next: Pointer<T>,
}

#[derive(Debug)]
pub struct SingleLinkedList<T: std::fmt::Debug + std::marker::Copy> {
    head: Pointer<T>,
}

impl<T: std::fmt::Debug + std::marker::Copy> SingleLinkedList<T> {
    fn create_empty_list() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn peek(&mut self) -> Option<T> {
        match &self.head {
            Some(cur_head) => Some(cur_head.element),
            None => None,
        }
    }

    fn pop_front(&mut self) {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
            }
            None => {}
        }
    }

    fn size(&self) -> usize {
        let mut result = 0;
        let mut cur_node = &self.head;
        while cur_node.is_some() {
            cur_node = &cur_node.as_ref().unwrap().next;
            result += 1;
        }
        result
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        print!("SingleLinkedList [");
        loop {
            match list_traversal {
                Some(cur_node) => {
                    if cur_node.next.is_some() {
                        print!("{:?}, ", cur_node.element);
                    } else {
                        print!("{:?}", cur_node.element);
                    }
                    list_traversal = &cur_node.next;
                }
                None => {
                    println!("]");
                    break;
                }
            }
        }
    }
}

// Tests --------------------------------------------------------------------------
mod test {
    use std::{f32::consts::E, io::empty};

    use super::*;

    #[test]
    fn test_creating_nodes() {
        let list_1 = Node {
            element: 1,
            next: None,
        };

        let list_2 = Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node {
                    element: 3,
                    next: None,
                })),
            })),
        };
    }

    #[test]
    fn test_raw_creating_list() {
        let empty_list: SingleLinkedList<i32> = SingleLinkedList { head: None };

        let list_with_head_only = SingleLinkedList {
            head: Some(Box::new(Node {
                element: 100,
                next: None,
            })),
        };

        let list_with_three_nodes = SingleLinkedList {
            head: Some(Box::new(Node {
                element: 100,
                next: Some(Box::new(Node {
                    element: 200,
                    next: Some(Box::new(Node {
                        element: 300,
                        next: None,
                    })),
                })),
            })),
        };

        println!("list = {:?}", list_with_three_nodes.head);
    }

    #[test]
    fn test_create_empty_list() {
        let list: SingleLinkedList<i32> = SingleLinkedList::create_empty_list();
        assert_eq!(list.head, None);
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_create_list_and_add_some_nodes() {
        let mut list = SingleLinkedList::create_empty_list();
        list.push_front(100);
        assert_eq!(list.size(), 1);
        list.push_front(200);
        list.push_front(300);
        assert_eq!(list.size(), 3);
        println!("list = {:?}", list.head);
    }

    #[test]
    fn test_push_get_and_pop() {
        let mut list = SingleLinkedList::create_empty_list();
        list.push_front(100);
        list.push_front(200);
        list.push_front(300);

        assert_eq!(list.peek(), Some(300));

        list.pop_front();
        assert_eq!(list.size(), 2);
        assert_eq!(list.peek(), Some(200));

        list.pop_front();
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek(), Some(100));

        list.pop_front();
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_printing() {
        let mut list = SingleLinkedList::create_empty_list();
        list.print();
        list.push_front(100);
        list.push_front(200);
        list.push_front(300);
        list.push_front(666);
        list.print();
    }

    #[test]
    fn test_list_with_string() {
        let mut string_list = SingleLinkedList::create_empty_list();
        string_list.push_front("World!");
        string_list.push_front("Hello");
        string_list.print();
    }
}
